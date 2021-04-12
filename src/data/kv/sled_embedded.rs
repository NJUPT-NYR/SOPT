use super::*;
use sled::{IVec, Serialize};
use std::convert::TryInto;

pub struct SledWrapper {
    pub db: sled::Db,
    // TODO: logger and other
}

impl KVStorage for SledWrapper {
    fn put(&self, cf: &str, key: &[u8], val: &[u8]) -> Result<(), Error> {
        let tree = self.db.open_tree(cf)?;
        let val = IVec::from(val);
        tree.insert(key, val)?;

        Ok(())
    }

    fn get_string(&self, cf: &str, key: &[u8]) -> Result<Option<String>, Error> {
        let tree = self.db.open_tree(cf)?;
        let result = tree.get(key)?;
        let mut ret: Option<String> = None;

        if result.is_some() {
            let iv = result.unwrap();
            let string =
                String::from_utf8(iv.serialize()).map_err(|e| Error::OtherError(e.to_string()))?;
            ret = Some(string);
        }

        Ok(ret)
    }

    fn get_number(&self, cf: &str, key: &[u8]) -> Result<Option<i64>, Error> {
        let tree = self.db.open_tree(cf)?;
        let result = tree.get(key)?;
        let mut ret: Option<i64> = None;

        if result.is_some() {
            let iv = result.unwrap();
            let num = i64::from_ne_bytes(
                iv.as_ref()
                    .split_at(std::mem::size_of::<i64>())
                    .0
                    .try_into()
                    .unwrap(),
            );
            ret = Some(num);
        }

        Ok(ret)
    }

    fn get_float(&self, cf: &str, key: &[u8]) -> Result<Option<f64>, Error> {
        let tree = self.db.open_tree(cf)?;
        let result = tree.get(key)?;
        let mut ret: Option<f64> = None;

        if result.is_some() {
            let iv = result.unwrap();
            let num = f64::from_ne_bytes(
                iv.as_ref()
                    .split_at(std::mem::size_of::<f64>())
                    .0
                    .try_into()
                    .unwrap(),
            );
            ret = Some(num);
        }

        Ok(ret)
    }

    fn delete(&self, cf: &str, key: &[u8]) -> Result<(), Error> {
        let tree = self.db.open_tree(cf)?;
        tree.remove(key)?;

        Ok(())
    }
}
