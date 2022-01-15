use std::error::Error;
use kv::*;

#[cfg(test)]

struct Record {
    key: String,
    deltas: Vec<Delta>,
    head: Delta,
}

struct Delta {
    key: String,
    data: Vec<u8>,
    version: u32,
}

struct KVStore {
    datastore: Store,
}

impl<T> KVStore<T> {
    fn new(ds: Store) -> KVStore {
        KVStore { datastore: ds }
    }

    fn get(&self, key: String) -> Result<Json<T>,Error> {
        let result: Json<T> = self.datastore.bucket.get(key)?.unwrap();
        Ok(result)
    }

    fn put<T>(&self, key: String, val: T) -> Result<T,Error> {
        #[cfg(feature = "json-value")]
        {
            let bucket = self.datastore.bucket<&str, Json<T>>(None)?;
            bucket.set(key,Json(val))?;
        }
        Ok(Json(val))
    }

    fn del(&self, key: String) -> Error {
        Err(())
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