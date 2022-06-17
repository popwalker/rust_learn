pub mod abi;

use crate::KvError;
use abi::{command_request::RequestData, *};
use bytes::Bytes;
use http::StatusCode;
use prost::Message;

impl CommandRequest {
    // 创建HSET命令
    pub fn new_hset(table: impl Into<String>, key: impl Into<String>, value: Value) -> Self {
        Self {
            request_data: Some(RequestData::Hset(Hset {
                table: table.into(),
                pair: Some(Kvpair::new(key, value)),
            })),
        }
    }
    pub fn new_hmset(table: impl Into<String>, pairs: Vec<Kvpair>) -> Self {
        Self {
            request_data: Some(RequestData::Hmset(Hmset {
                table: table.into(),
                pairs: pairs,
            })),
        }
    }

    // 创建hget命令
    pub fn new_hget(table: impl Into<String>, key: impl Into<String>) -> Self {
        Self {
            request_data: Some(RequestData::Hget(Hget {
                table: table.into(),
                key: key.into(),
            })),
        }
    }

    pub fn new_hgetall(table: impl Into<String>) -> Self {
        Self {
            request_data: Some(RequestData::Hgetall(Hgetall {
                table: table.into(),
            })),
        }
    }

    pub fn new_hmget(table: impl Into<String>, keys: Vec<String>) -> Self {
        Self {
            request_data: Some(RequestData::Hmget(Hmget {
                table: table.into(),
                keys: keys,
            })),
        }
    }

    pub fn new_hdel(table: impl Into<String>, key: impl Into<String>) -> Self {
        Self {
            request_data: Some(RequestData::Hdel(Hdel {
                table: table.into(),
                key: key.into(),
            })),
        }
    }

    pub fn new_hmdel(table: impl Into<String>, keys: Vec<String>) -> Self {
        Self {
            request_data: Some(RequestData::Hmdel(Hmdel {
                table: table.into(),
                keys: keys,
            })),
        }
    }
    pub fn new_hexist(table: impl Into<String>, key: impl Into<String>) -> Self {
        Self {
            request_data: Some(RequestData::Hexist(Hexist {
                table: table.into(),
                key: key.into(),
            })),
        }
    }
    pub fn new_hmexist(table: impl Into<String>, keys: Vec<String>) -> Self {
        Self {
            request_data: Some(RequestData::Hmexist(Hmexist {
                table: table.into(),
                keys: keys,
            })),
        }
    }

    pub fn new_subscribe(name: impl Into<String>) -> Self {
        Self { request_data: Some(RequestData::Subscribe(Subscribe{topic: name.into()})) }
    }

    pub fn new_unsubscribe(name: impl Into<String>, id: u32) -> Self {
        Self { request_data: Some(RequestData::Unsubscribe(Unsubscribe{topic: name.into(), id})) }
    }

    pub fn new_publish(name: impl Into<String>, data: Vec<Value>) -> Self {
        Self { 
            request_data: Some(RequestData::Publish(Publish{
                topic: name.into(),
                data,
            })) 
        }
    }

    pub fn format(&self) -> String {
        format!("{:?}", self)
    }
}

impl CommandResponse {
    pub fn ok() -> Self {
        CommandResponse { status: StatusCode::OK.as_u16() as _, ..Default::default()}
    }
    pub fn internal_error(msg: String) -> Self {
        CommandResponse { status: StatusCode::INTERNAL_SERVER_ERROR.as_u16() as _, message: msg, ..Default::default() }
    }
    pub fn format(&self) -> String {
        format!("{:?}", self)
    }
}

impl Value {
    pub fn format(&self) -> String {
        format!("{:?}", self)
    }
}

impl Kvpair {
    pub fn new(key: impl Into<String>, value: Value) -> Self {
        Self {
            key: key.into(),
            value: Some(value),
        }
    }
}

impl From<String> for Value {
    fn from(s: String) -> Self {
        Self {
            value: Some(value::Value::String(s)),
        }
    }
}

impl From<&str> for Value {
    fn from(s: &str) -> Self {
        Self {
            value: Some(value::Value::String(s.into())),
        }
    }
}

impl From<i64> for Value {
    fn from(v: i64) -> Self {
        Self {
            value: Some(value::Value::Integer(v)),
        }
    }
}

impl From<f64> for Value {
    fn from(f: f64) -> Self {
        Self {
            value: Some(value::Value::Float(f)),
        }
    }
}

impl From<bool> for Value {
    fn from(b: bool) -> Self {
        Self {
            value: Some(value::Value::Bool(b)),
        }
    }
}

impl<const N: usize> From<&[u8; N]> for Value {
    fn from(v: &[u8; N]) -> Self {
        bytes::Bytes::copy_from_slice(&v[..]).into()
    }
}

impl From<Bytes> for Value {
    fn from(buf: Bytes) -> Self {
        Self {
            value: Some(value::Value::Binary(buf)),
        }
    }
}

impl From<Vec<Value>> for CommandResponse {
    fn from(v: Vec<Value>) -> Self {
        Self {
            status: StatusCode::OK.as_u16() as _,
            values: v,
            ..Default::default()
        }
    }
}

impl From<Value> for CommandResponse {
    fn from(v: Value) -> Self {
        Self {
            status: StatusCode::OK.as_u16() as _,
            values: vec![v],
            ..Default::default()
        }
    }
}

impl From<KvError> for CommandResponse {
    fn from(e: KvError) -> Self {
        let mut result = Self {
            status: StatusCode::INTERNAL_SERVER_ERROR.as_u16() as _,
            message: e.to_string(),
            values: vec![],
            pairs: vec![],
        };

        match e {
            KvError::NotFound(_) => result.status = StatusCode::NOT_FOUND.as_u16() as _,
            KvError::InvalidCommand(_) => result.status = StatusCode::BAD_REQUEST.as_u16() as _,
            _ => {}
        }
        result
    }
}

impl From<Vec<Kvpair>> for CommandResponse {
    fn from(pairs: Vec<Kvpair>) -> Self {
        Self {
            status: StatusCode::OK.as_u16() as _,
            pairs: pairs,
            ..Default::default()
        }
    }
}

impl TryFrom<&[u8]> for Value {
    type Error = KvError;
    fn try_from(data: &[u8]) -> Result<Self, Self::Error> {
        let msg = Value::decode(data)?;
        Ok(msg)
    }
}

impl TryFrom<Value> for Vec<u8> {
    type Error = KvError;
    fn try_from(v: Value) -> Result<Self, Self::Error> {
        let mut buf = Vec::with_capacity(v.encoded_len());
        v.encode(&mut buf)?;
        Ok(buf)
    }
}

impl TryFrom<Value> for bool {
    type Error = KvError;
    fn try_from(v: Value) -> Result<Self, Self::Error> {
        match v.value {
            Some(value::Value::Bool(b)) => Ok(b),
            _ => Err(KvError::ConvertError(v.format(), "Bollean")),
        }
    }
}

impl TryFrom<Value> for Bytes {
    type Error = KvError;
    fn try_from(v: Value) -> Result<Self, Self::Error> {
        match v.value {
            Some(value::Value::Binary(b)) => Ok(b),
            _ => Err(KvError::ConvertError(v.format(), "Binary")),
        }
    }
}

impl TryFrom<Value> for f64 {
    type Error = KvError;
    fn try_from(v: Value) -> Result<Self, Self::Error> {
        match v.value {
            Some(value::Value::Float(f)) => Ok(f),
            _ => Err(KvError::ConvertError(v.format(), "Float")),
        }
    }
}

impl TryFrom<Value> for i64 {
    type Error = KvError;
    fn try_from(v: Value) -> Result<Self, Self::Error> {
        match v.value {
            Some(value::Value::Integer(i)) => Ok(i),
            _ => Err(KvError::ConvertError(v.format(), "Integer")),
        }
    }
}
impl TryFrom<&Value> for i64 {
    type Error = KvError;
    fn try_from(v: &Value) -> Result<Self, Self::Error> {
        match v.value {
            Some(value::Value::Integer(i)) => Ok(i),
            _ => Err(KvError::ConvertError(v.format(), "Integer")),
        }
    }
}

impl TryFrom<&CommandResponse> for i64 {
    type Error = KvError;
    fn try_from(value: &CommandResponse) -> Result<Self, Self::Error> {
        if value.status != StatusCode::OK.as_u16() as u32 {
            return Err(KvError::ConvertError(value.format(), "CommandResponse"));
        }

        match value.values.get(0) {
            Some(v) => v.try_into(),
            None => Err(KvError::ConvertError(value.format(), "CommandResponse")),
        }
    }
}