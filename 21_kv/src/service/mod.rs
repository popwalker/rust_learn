use std::sync::Arc;

use tracing::debug;

use crate::{command_request::RequestData, CommandRequest, CommandResponse, KvError, MemTable, Storage};
mod command_service;

// 对Command的处理抽象
pub trait CommandService {
    fn execute(self, store: &impl Storage) -> CommandResponse;
}

// Service 数据结构
pub struct Service<Store = MemTable> {
    inner: Arc<ServiceInner<Store>>
}

impl<Store> Clone for Service<Store> {
    fn clone(&self) -> Self {
        Self { inner: Arc::clone(&self.inner) }
    }
}

pub struct ServiceInner<Store> {
    store: Store
}

impl<Store: Storage> Service<Store> {
    pub fn new(store: Store) -> Self {
        Self { inner: Arc::new(ServiceInner{store}) }
    }

    pub fn execute(&self, cmd: CommandRequest) -> CommandResponse {
        debug!("Got request: {:?}", cmd);
        let res = dispatch(cmd, &self.inner.store);
        debug!("Execited response: {:?}", res);
        // TODO: 发送on_execute事件
        res
    }
}

pub fn dispatch(cmd: CommandRequest, store: &impl Storage) -> CommandResponse {
    match cmd.request_data {
        Some(RequestData::Hget(param)) => param.execute(store), 
        Some(RequestData::Hgetall(param)) => param.execute(store), 
        Some(RequestData::Hset(param)) => param.execute(store), 
        None => KvError::InvalidCommand("Request as no data".into()).into(),
        _ => KvError::Internal("Not implemented".into()).into(),
    }
}


#[cfg(test)]
mod tests {
    use std::thread;

    use super::*;
    use crate::{MemTable, Value};

    #[test]
    fn service_should_work() {
        // 我们需要一个service结构至少包含Storage
        let service = Service::new(MemTable::new());

        // service可以运行在多线程环境下，它的clone应该是轻量的
        let cloned = service.clone();

        // 创建一个线程，在table1中写入k1, v1
        let handle = thread::spawn(move || {
            let res = cloned.execute(CommandRequest::new_hset("t1", "k1", "v1".into()));
            assert_res_ok(res, &[Value::default()], &[]);
        });
        handle.join().unwrap();

        // 在当前线程下读取table t1的 k1,应该返回v1
        let res = service.execute(CommandRequest::new_hget("t1", "k1"));
        assert_res_ok(res, &["v1".into()], &[]);
    }
}

#[cfg(test)]
use crate::{Kvpair, Value};

#[cfg(test)]
fn assert_res_ok(mut res: CommandResponse, values: &[Value], pairs: &[Kvpair]) {
    res.pairs.sort_by(|a, b| a.partial_cmp(b).unwrap());
    assert_eq!(res.status, 200);
    assert_eq!(res.message, "");
    assert_eq!(res.values, values);
    assert_eq!(res.pairs, pairs);
}

// 测试失败返回的结果
#[cfg(test)]
pub fn assert_res_error(res: CommandResponse, code: u32, msg: &str) {    
    assert_eq!(res.status, code);    
    assert!(res.message.contains(msg));    
    assert_eq!(res.values, &[]);    
    assert_eq!(res.pairs, &[]);
}