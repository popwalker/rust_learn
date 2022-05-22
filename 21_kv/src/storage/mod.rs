mod memory;

pub use memory::MemTable;

use crate::{KvError, Kvpair, Value};

pub trait Storage {
    // 从一个HashTable中里获取一个key的value
    fn get(&self, table: &str, key: &str) -> Result<Option<Value>, KvError>;
    // 从一个HashTable里设置一个key的value，返回旧的value
    fn set(&self, table: &str, key: String, value: Value) -> Result<Option<Value>, KvError>;
    // 查看HashTable中是否有key
    fn contains(&self, table: &str, key: &str) -> Result<bool, KvError>;
    // 从HashTable中删除一个key
    fn del(&self, table: &str, key: &str) -> Result<Option<Value>, KvError>;
    // 遍历HashTable，返回所有kv pair
    fn get_all(&self, table: &str) -> Result<Vec<Kvpair>, KvError>;
    // 遍历HashTable，返回kv pair的Iterator
    fn get_iter(&self, table: &str) -> Result<Box<dyn Iterator<Item = Kvpair>>, KvError>;
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn memtable_basic_interface_should_work() {
        let store = MemTable::new();
        test_basi_interface(store);
    }

    #[test]
    fn memtable_get_all_should_work() {
        let store = MemTable::new();
        test_get_all(store);
    }

    fn test_basi_interface(store: impl Storage) {
        let v = store.set("t1", "hello".into(), "world".into());
        assert!(v.unwrap().is_none());
        let v1 = store.set("t1", "hello".into(), "world1".into());
        assert_eq!(v1, Ok(Some("world".into())));
        let v = store.get("t1", "hello");
        assert_eq!(v, Ok(Some("world1".into())));

        assert_eq!(Ok(None), store.get("t1", "hello1"));
        assert!(store.get("t2", "hello1").unwrap().is_none());

        assert_eq!(store.contains("t1", "hello"), Ok(true));
        assert_eq!(store.contains("t1", "hello1"), Ok(false));
        assert_eq!(store.contains("t2", "hello"), Ok(false));

        let v = store.del("t1", "hello");
        assert_eq!(v, Ok(Some("world1".into())));

        assert_eq!(Ok(None), store.del("t1", "hello1"));
        assert_eq!(Ok(None), store.del("t2", "hello1"));
    }

    fn test_get_all(store: impl Storage) {
        store.set("t2", "k1".into(), "v1".into()).unwrap();
        store.set("t2", "k2".into(), "v2".into()).unwrap();

        let mut data = store.get_all("t2").unwrap();
        data.sort_by(|a, b| a.partial_cmp(b).unwrap());
        assert_eq!(
            data,
            vec![
                Kvpair::new("k1", "v1".into()),
                Kvpair::new("k2", "v2".into()),
            ]
        )
    }

    fn test_get_iter(store: impl Storage) {
        store.set("t2", "k1".into(), "v1".into()).unwrap();
        store.set("t2", "k2".into(), "v2".into()).unwrap();
        let mut data: Vec<_> = store.get_iter("t2").unwrap().collect();
        data.sort_by(|a, b| a.partial_cmp(b).unwrap());
        assert_eq!(
            data,
            vec![
                Kvpair::new("k1", "v1".into()),
                Kvpair::new("k2", "v2".into()),
            ]
        )
    }
}
