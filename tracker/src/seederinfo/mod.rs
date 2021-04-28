use super::*;
mod seederarray;
mod seedermap;
use peerinfo::PeerInfo;
use seederarray::SeederArray;
pub use seedermap::SeederMap;

type Key = u64;
type Value = PeerInfo;

#[derive(Clone)]
pub struct Bucket {
    time_to_compaction: u64,
    pub key: Key,
    pub value: Value,
}

impl Bucket {
    pub fn new() -> Self {
        Self {
            time_to_compaction: 0,
            key: Default::default(),
            value: Default::default(),
        }
    }

    pub fn from(k: Key, v: Value) -> Self {
        Bucket {
            time_to_compaction: util::get_timestamp() + 2700,
            key: k,
            value: v,
        }
    }
}

impl Default for Bucket {
    fn default() -> Self {
        Self::new()
    }
}

pub enum SeederInfo {
    InlineSeeder(SeederArray),
    MulitSeeder(SeederMap),
}

impl SeederInfo {
    pub fn new() -> Self {
        SeederInfo::InlineSeeder(SeederArray::new())
    }

    pub fn compaction(&mut self) {
        match self {
            SeederInfo::InlineSeeder(sa) => sa.compaction(),
            SeederInfo::MulitSeeder(sm) => {
                sm.compaction();
                if sm.get_seeder_cnt() < 3 {
                    if let Ok(sa) = SeederArray::from(sm) {
                        *self = SeederInfo::InlineSeeder(sa);
                    }
                }
            }
        }
    }

    pub fn gen_response(&self, num_want: usize) -> RedisValue {
        let (peers, peers6) = match self {
            SeederInfo::MulitSeeder(sm) => sm.gen_response(num_want),
            SeederInfo::InlineSeeder(sa) => sa.gen_response(),
        };
        RedisValue::Array(vec![
            // interval
            RedisValue::Integer(1800),
            RedisValue::Buffer(peers),
            RedisValue::Buffer(peers6),
        ])
    }

    pub fn delete(&mut self, uid: u64) {
        match self {
            SeederInfo::MulitSeeder(sm) => sm.delete(uid),
            SeederInfo::InlineSeeder(sa) => sa.delete(uid),
        }
    }

    pub fn insert(&mut self, uid: u64, p: PeerInfo) {
        match self {
            SeederInfo::MulitSeeder(sm) => sm.insert(uid, &p),
            SeederInfo::InlineSeeder(sa) => {
                if let Err(_) = sa.insert(uid, &p) {
                    let mut sm = SeederMap::from(sa);
                    sm.insert(uid, &p);
                    *self = SeederInfo::MulitSeeder(sm);
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::peerinfo::PeerInfo;

    use super::{seederarray::SeederArray, Bucket, SeederInfo, SeederMap};

    #[test]
    fn test_struct_size() {
        assert_eq!(std::mem::size_of::<Bucket>(), 40);
        assert!(std::mem::size_of::<SeederInfo>() <= 176);
    }

    #[test]
    fn test_compaction() {
        let sm = SeederMap::new();
        let mut si = SeederInfo::MulitSeeder(sm);
        si.compaction();
        assert!(match si {
            SeederInfo::MulitSeeder(_) => false,
            SeederInfo::InlineSeeder(_) => true,
        });
    }

    #[test]
    fn test_upgrade() {
        let v = PeerInfo::new();
        let mut sa = SeederArray::new();
        assert!(sa.insert(1, &v).is_ok());
        assert!(sa.insert(2, &v).is_ok());
        assert!(sa.insert(3, &v).is_ok());
        assert!(sa.insert(4, &v).is_ok());
        let mut si = SeederInfo::InlineSeeder(sa);
        si.insert(5, v);
        assert!(match si {
            SeederInfo::MulitSeeder(_) => true,
            SeederInfo::InlineSeeder(_) => false,
        });
    }
}
