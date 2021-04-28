use super::*;

const SEEDER_ARRAY_LENGTH: usize = 4;
pub struct SeederArray {
    seeders: [Bucket; SEEDER_ARRAY_LENGTH],
    in_use: [bool; SEEDER_ARRAY_LENGTH],
}

type SeederArrayIter<'a> = std::iter::Zip<std::slice::Iter<'a, Bucket>, std::slice::Iter<'a, bool>>;

impl SeederArray {
    pub fn new() -> Self {
        Self {
            seeders: Default::default(),
            in_use: [false; SEEDER_ARRAY_LENGTH],
        }
    }

    pub fn iter(&self) -> SeederArrayIter {
        self.seeders.iter().zip(self.in_use.iter())
    }

    pub fn insert(&mut self, k: Key, v: &Value) -> Result<(), ()> {
        // try update
        for (b, &in_use) in self.seeders.iter_mut().zip(self.in_use.iter()) {
            if in_use && b.key == k {
                b.value.update(v);
                b.time_to_compaction = util::get_timestamp() + 2700;
                return Ok(());
            }
        }
        // try push
        for (in_use, seeder) in self.in_use.iter_mut().zip(self.seeders.iter_mut()) {
            if *in_use == false {
                *seeder = Bucket::from(k, v.clone());
                *in_use = true;
                return Ok(());
            }
        }
        // overflow
        return Err(());
    }

    pub fn delete(&mut self, k: Key) {
        for (b, in_use) in self.seeders.iter().zip(self.in_use.iter_mut()) {
            if b.key == k {
                *in_use = false;
                return;
            }
        }
    }

    pub fn compaction(&mut self) {
        let now = util::get_timestamp();
        for (b, in_use) in self.seeders.iter().zip(self.in_use.iter_mut()) {
            if *in_use && now > b.time_to_compaction {
                *in_use = false;
            }
        }
    }

    pub fn gen_response(&self) -> (Vec<u8>, Vec<u8>) {
        let mut buf_peer: Vec<u8> = Vec::with_capacity(SEEDER_ARRAY_LENGTH * 6);
        let mut buf_peer6: Vec<u8> = Vec::with_capacity(SEEDER_ARRAY_LENGTH * 18);
        for (b, &in_use) in self.seeders.iter().zip(self.in_use.iter()) {
            if in_use {
                let p = &b.value;
                if let Some(ref v4) = p.get_ipv4() {
                    buf_peer.extend_from_slice(&v4.octets());
                    buf_peer.extend_from_slice(&p.get_port().to_be_bytes());
                };
                if let Some(v6) = p.get_ipv6() {
                    buf_peer6.extend_from_slice(&v6.octets());
                    buf_peer6.extend_from_slice(&p.get_port().to_be_bytes());
                };
            }
        }
        (buf_peer, buf_peer6)
    }

    pub fn from(sm: &SeederMap) -> Result<Self, ()> {
        if sm.get_seeder_cnt() >= SEEDER_ARRAY_LENGTH {
            return Err(());
        }
        let mut sa = SeederArray::new();
        for (k, v) in sm.iter() {
            sa.insert(*k, v)?;
        }
        Ok(sa)
    }
}

#[cfg(test)]
mod tests {
    use crate::{peerinfo::PeerInfo, seederinfo::SeederMap};

    use super::SeederArray;

    #[test]
    fn check_struct_size() {
        assert!(std::mem::size_of::<SeederArray>() <= 168);
    }

    #[test]
    fn test_insert() {
        let v = PeerInfo::default();
        let mut sa = SeederArray::new();
        assert!(sa.insert(1, &v).is_ok());
        assert!(sa.insert(2, &v).is_ok());
        assert!(sa.insert(3, &v).is_ok());
        assert!(sa.insert(4, &v).is_ok());
        assert!(sa.insert(4, &v).is_ok());
        assert!(sa.insert(6, &v).is_err());
        sa.in_use[0] = false;
        assert!(sa.insert(6, &v).is_ok());
    }

    #[test]
    fn test_delete() {
        let v = PeerInfo::default();
        let mut sa = SeederArray::new();
        assert!(sa.insert(1, &v).is_ok());
        sa.delete(1);
        sa.in_use[0] = false;
        assert_eq!(sa.in_use[0], false);
    }

    #[test]
    fn test_downgrade() {
        let v = PeerInfo::default();
        let mut sm = SeederMap::new();
        sm.insert(1, &v);
        sm.insert(2, &v);
        let sa = SeederArray::from(&sm);
        assert!(sa.is_ok());
        let sa = sa.unwrap();
        assert_eq!(sa.in_use[0], true);
        assert_eq!(sa.in_use[1], true);
        assert_eq!(sa.in_use[2], false);
        assert_eq!(sa.in_use[3], false);
    }
}
