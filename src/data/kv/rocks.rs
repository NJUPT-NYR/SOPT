use super::*;

pub struct RocksWrapper {
    pub db: rocksdb::DB,
}

impl KVStorage for RocksWrapper {
    fn put(&self, cf: &str, key: &[u8], val: &[u8]) -> Result<(), Error> {
        let cf = self
            .db
            .cf_handle(cf)
            .ok_or(Error::KVError("no such column family".to_string()))?;
        self.db.put_cf(cf, key, val)?;

        Ok(())
    }

    fn get_string(&self, cf: &str, key: &[u8]) -> Result<Option<String>, Error> {
        let cf = self
            .db
            .cf_handle(cf)
            .ok_or(Error::KVError("no such column family".to_string()))?;
        let result = self.db.get_cf(cf, key)?;
        let mut ret: Option<String> = None;

        if result.is_some() {
            let result = result.unwrap();
            let string = String::from_utf8(result).map_err(|e| Error::OtherError(e.to_string()))?;
            ret = Some(string);
        }

        Ok(ret)
    }

    fn get_number(&self, cf: &str, key: &[u8]) -> Result<Option<i64>, Error> {
        let cf = self
            .db
            .cf_handle(cf)
            .ok_or(Error::KVError("no such column family".to_string()))?;
        let result = self.db.get_cf(cf, key)?;
        let mut ret: Option<i64> = None;

        if result.is_some() {
            let result = result.unwrap();
            let num = i64::from_ne_bytes(
                result
                    .as_slice()
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
        let cf = self
            .db
            .cf_handle(cf)
            .ok_or(Error::KVError("no such column family".to_string()))?;
        let result = self.db.get_cf(cf, key)?;
        let mut ret: Option<f64> = None;

        if result.is_some() {
            let result = result.unwrap();
            let num = f64::from_ne_bytes(
                result
                    .as_slice()
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
        let cf = self
            .db
            .cf_handle(cf)
            .ok_or(Error::KVError("no such column family".to_string()))?;
        self.db.delete_cf(cf, key)?;

        Ok(())
    }
}
