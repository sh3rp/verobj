
use std::result::Result;
use serde::{Serialize,Deserialize};
use kv::Store;
use kv::Json;

#[derive(Serialize, Deserialize, PartialEq)]
#[derive(Clone,Debug)]
pub struct Record {
    key: String,
    deltas: Vec<Delta>,
    head: Delta,
}

#[derive(Serialize, Deserialize, PartialEq)]
#[derive(Clone,Debug)]
pub struct Delta {
    key: String,
    data: Vec<u8>,
    version: u32,
}

#[derive(Clone,Debug)]
pub struct KVStore {
    datastore: Store,
}

impl KVStore {
    pub fn new(ds: Store) -> KVStore {
        KVStore { datastore: ds }
    }

    pub fn get(&self, key: &str) -> Result<Json<Record>,kv::Error> {
        let bucket = self.datastore.bucket::<&str,Json<Record>>(None)?;
        let result: Json<Record> = bucket.get(key)?.unwrap();
        Ok(result)
    }

    pub fn put(&self, key: String, val: Record) -> Result<Json<Record>,std::io::Error> {
        #[cfg(feature = "json-value")]
        {
            let bucket = self.datastore.bucket::<&str, Json<T>>(None)?;
            bucket.set(key,Json(val))?;
        }
        Ok(Json(val))
    }

    fn del(&self, key: String) {
    }
}

#[cfg(test)]
mod test {    
    use kv::*;
    use crate::{KVStore,Record,Delta};
    use 

    #[test]
    fn test_kvstore() {
        let cfg = Config::new("./test/example1");
        let store = Store::new(cfg).unwrap();
        let ds: KVStore = KVStore::new(store);

        let record: Record = Record{
            key: "test",
        };
        ds.put(record.key,record);
        let r = ds.get(record.key).unwrap();
        dbg!(r);
    }
}