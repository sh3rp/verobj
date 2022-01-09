
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
    fn Get<T>(key: String) -> Result<T,Error>
    fn Put<T>(key: String, val: T) -> Result<T,Error>
    fn Del(key: String) -> Error
}

impl 