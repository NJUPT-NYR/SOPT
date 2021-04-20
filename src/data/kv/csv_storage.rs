use super::*;
use csv::{Reader, Writer};
use serde::Deserialize;
use std::io::Write;

#[derive(Debug, Deserialize)]
struct KeyValue {
    key: String,
    value: Box<[u8]>,
}

pub struct CSVReader {
    pub path: String,
}

impl KVStorage for CSVReader {
    fn put(&self, cf: &str, key: &[u8], val: &[u8]) -> Result<(), Error> {
        // FIXME: so dumb for now

        let path = Path::new(&self.path).join(format!("{}.csv", cf));
        let mut file = std::fs::OpenOptions::new()
            .write(true)
            .append(true)
            .open(&path);
        if let Err(_) = file {
            file = std::fs::File::create(&path);
        }
        let mut f = file.map_err(|e| Error::OtherError(e.to_string()))?;
        f.write(key).map_err(|e| Error::OtherError(e.to_string()))?;
        f.write(b",")
            .map_err(|e| Error::OtherError(e.to_string()))?;
        f.write(val).map_err(|e| Error::OtherError(e.to_string()))?;
        f.write(b"\n")
            .map_err(|e| Error::OtherError(e.to_string()))?;
        Ok(())
    }

    fn get_string(&self, cf: &str, key: &[u8]) -> Result<Option<String>, Error> {
        let path = Path::new(&self.path).join(format!("{}.csv", cf));
        let key = String::from_utf8(key.to_vec()).map_err(|e| Error::OtherError(e.to_string()))?;

        let mut reader = Reader::from_path(path)?;
        for record in reader.deserialize() {
            let record: KeyValue = record?;
            if record.key == key {
                let res = String::from_utf8(record.value.as_ref().to_vec())
                    .map_err(|e| Error::OtherError(e.to_string()))?;
                return Ok(Some(res));
            }
        }

        Ok(None)
    }

    fn get_number(&self, cf: &str, key: &[u8]) -> Result<Option<i64>, Error> {
        let path = Path::new(&self.path).join(format!("{}.csv", cf));
        let key = String::from_utf8(key.to_vec()).map_err(|e| Error::OtherError(e.to_string()))?;

        let mut reader = Reader::from_path(path)?;
        for record in reader.deserialize() {
            let record: KeyValue = record?;
            if record.key == key {
                let res = i64::from_ne_bytes(
                    record
                        .value
                        .as_ref()
                        .split_at(std::mem::size_of::<i64>())
                        .0
                        .try_into()
                        .unwrap(),
                );
                return Ok(Some(res));
            }
        }

        Ok(None)
    }

    fn get_float(&self, cf: &str, key: &[u8]) -> Result<Option<f64>, Error> {
        let path = Path::new(&self.path).join(format!("{}.csv", cf));
        let key = String::from_utf8(key.to_vec()).map_err(|e| Error::OtherError(e.to_string()))?;

        let mut reader = Reader::from_path(path)?;
        for record in reader.deserialize() {
            let record: KeyValue = record?;
            if record.key == key {
                let res = f64::from_ne_bytes(
                    record
                        .value
                        .as_ref()
                        .split_at(std::mem::size_of::<f64>())
                        .0
                        .try_into()
                        .unwrap(),
                );
                return Ok(Some(res));
            }
        }

        Ok(None)
    }

    fn delete(&self, cf: &str, _key: &[u8]) -> Result<(), Error> {
        let path = Path::new(&self.path).join(format!("{}.csv", cf));
        let mut _writer = Writer::from_path(path)?;
        // FIXME: delete hasn't been supported for csv!

        Ok(())
    }
}
