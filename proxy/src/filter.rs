use bloom::CountingBloomFilter;
use std::ops::DerefMut;
use std::sync::atomic::*;
use tokio::sync::RwLock;

enum Operation {
    Set,
    Delete,
}

/// A Filter built for solve both read and write in high concurrency
/// Updates will be operated in batch.
/// When a operation come, the update will be performed in 2 phases.
/// 1. add to cache
/// 2. once the cache is large enough, the updates will be committed.
///
/// And once the filter touch the capacity, in_expand will be set,
/// then, a new thread will be spawned to expand the capacity. meanwhile
/// the cache will not commit until the new filter build up.
pub struct Filter {
    inner: RwLock<CountingBloomFilter>,
    capacity: AtomicU32,
    // cache: RwLock<HashMap<String, Operation>>,
    cache: RwLock<Vec<(String, Operation)>>,
    amount: AtomicU32,
    in_expand: AtomicBool,
}

impl Filter {
    const BATCH_SIZE: usize = 32;

    fn batch_update(&self, inner: &mut CountingBloomFilter, ops: Vec<(String, Operation)>) {
        for (key, op) in ops.into_iter() {
            match op {
                Operation::Set => {
                    inner.insert_get_count(&key);
                    self.amount.fetch_add(1, Ordering::SeqCst);
                }
                Operation::Delete => {
                    inner.remove(&key);
                    self.amount.fetch_sub(1, Ordering::SeqCst);
                }
            };
        }
    }

    async fn fetch_cache(&self) -> Vec<(String, Operation)> {
        let mut new_cache = Vec::with_capacity(Filter::BATCH_SIZE * 2);
        let mut cache = self.cache.write().await;
        std::mem::swap(cache.deref_mut(), &mut new_cache);
        return new_cache;
    }

    pub fn new() -> Self {
        let capacity = AtomicU32::new(8192);
        let filter_inner = CountingBloomFilter::with_rate(4, 0.05, 8192);
        let inner = RwLock::new(filter_inner);
        let cache = RwLock::new(Vec::with_capacity(Filter::BATCH_SIZE * 2));
        let amount = AtomicU32::new(0);
        let in_expand = AtomicBool::new(false);
        // let expand_thread = None;
        Self {
            inner,
            capacity,
            amount,
            cache,
            in_expand,
            // expand_thread,
        }
    }

    pub async fn delete(&self, key: String) {
        let size;
        {
            let mut cache = self.cache.write().await;
            cache.push((key, Operation::Delete));
            size = cache.len();
        }
        if size > Filter::BATCH_SIZE {
            if self.in_expand.load(Ordering::Relaxed) == false {
                let cache = self.fetch_cache().await;
                let mut inner = self.inner.write().await;
                self.batch_update(inner.deref_mut(), cache);
            }
        }
    }

    pub async fn insert(&self, key: String) {
        let size;
        {
            let mut cache = self.cache.write().await;
            cache.push((key, Operation::Set));
            size = cache.len();
        }
        if size > Filter::BATCH_SIZE {
            if self.in_expand.load(Ordering::Relaxed) == false {
                let cache = self.fetch_cache().await;
                let mut inner = self.inner.write().await;
                self.batch_update(inner.deref_mut(), cache);
            }
        }
    }

    pub async fn contains(&self, key: &String) -> bool {
        let mut find;
        {
            let inner = self.inner.read().await;
            find = inner.estimate_count(key) > 0;
        }
        if !find {
            let cache = self.cache.read().await;
            for (k, v) in cache.iter() {
                if k == key && matches!(v, Operation::Set) {
                    find = true;
                    break;
                }
            }
        }
        return find;
    }

    pub fn check_expand(&self) -> bool {
        if self.in_expand.load(Ordering::Relaxed) == false {
            if self.amount.load(Ordering::Relaxed) > self.capacity.load(Ordering::Relaxed) {
                return true;
            }
        }
        false
    }

    // use some stream like, as Vec<String> is too large
    pub async fn expand(&self, keys: Vec<String>) {
        if self
            .in_expand
            .compare_exchange(false, true, Ordering::Acquire, Ordering::Relaxed)
            .is_ok()
        {
            let new_cap = (keys.len() * 3 / 2) as u32;
            let mut new_filter = CountingBloomFilter::with_rate(4, 0.05, new_cap);
            self.capacity.store(new_cap, Ordering::Relaxed);
            {
                // before expand, commit batch
                let cache = self.fetch_cache().await;
                let mut inner = self.inner.write().await;
                self.batch_update(inner.deref_mut(), cache);
            }
            self.amount.fetch_and(0, Ordering::SeqCst);
            for key in keys.into_iter() {
                new_filter.insert_get_count(&key);
                self.amount.fetch_add(1, Ordering::SeqCst);
            }
            let cache = self.fetch_cache().await;
            for (key, op) in cache.into_iter() {
                match op {
                    Operation::Set => {
                        new_filter.insert_get_count(&key);
                        self.amount.fetch_add(1, Ordering::SeqCst);
                    }
                    Operation::Delete => {
                        // do nothing, as it might cause false negative
                    }
                };
            }
            {
                let mut inner = self.inner.write().await;
                std::mem::swap(inner.deref_mut(), &mut new_filter);
            }
            self.in_expand.store(false, Ordering::Relaxed);
        }
    }
}
