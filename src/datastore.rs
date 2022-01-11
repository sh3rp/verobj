
struct Record {
    key: String,
    deltas: Vec<Delta>,
    head: Delta,
}

struct Delta {
    key: String,
    data: [u8],
    version: u32,
}

trait Datastore {
    fn get<T>(&self, key: String) -> Result<T,Error>
    fn put<T>(&self, key: String, val: T) -> Result<T,Error>
    fn del(&self, key: String) -> Error
}

struct KVStore<T> {
    datastore: Store,
}

impl Datastore for KVStore<T> {
    fn get<T>(&self, key: String) -> Result<T,Error> {}
    fn put<T>(&self, key: String, val: T) -> Result<T,Error> {
        #[cfg(feature = "json-value")]
        {
            let bucket = self.datastore.bucket<&str, Json<T>>(None)?;
            bucket.set(key,Json(val))?;
        }
    }
    fn del(&self, key: String) -> Error {}
}