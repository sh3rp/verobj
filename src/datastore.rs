
use std::result::Result;
use std::error::Error;
use serde::{Serialize,Deserialize};
use serde_json::*;
use kv::Store;
use kv::Json;

#[derive(Serialize, Deserialize, PartialEq)]
pub struct Record {
    key: String,
    deltas: Vec<Delta>,
    head: Delta,
}

#[derive(Serialize, Deserialize, PartialEq)]
pub struct Delta {
    key: String,
    data: Vec<u8>,
    version: u32,
}

pub struct KVStore {
    datastore: Store,
}

pub impl KVStore {
    fn new(ds: Store) -> KVStore {
        KVStore { datastore: ds }
    }

    fn get(&self, key: String) -> Result<Json<Record>,std::io::Error> {
        let bucket = self.datastore.bucket::<&str,Json<Record>>(None)?;
        let result: Json<Record> = bucket.get(key)?.unwrap();
        Ok(result)
    }

    fn put(&self, key: String, val: Record) -> Result<Record,std::io::Error> {
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

    #[test]
    fn test_kvstore() {
        //let ds: Datastore<Record> = KVStore::<Record>();
        assert!(true);
    }
}