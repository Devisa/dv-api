use serde::{Serialize, Deserialize};
use serde_json::Value;

use crate::error::JsonRpcError;

pub mod jsonrpc {

    use super::*;

    #[derive(Debug, Serialize, Deserialize)]
    pub struct Request {
        pub id: Value,
        pub jsonrpc: String,
        pub method: String,
        pub params: Vec<Value>,
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct Response {
        pub id: Value,
        pub jsonrpc: String,
        pub result: Value,
        pub error: Option<JsonRpcError>,
    }

    impl Default for Response {
        fn default() -> Self {
            Self {
                jsonrpc: "2.0".into(),
                result: Value::Null,
                error: None,
                id: Value::Null,
            }
        }
    }

    impl Request {
        pub fn dump(&self) -> String {
            serde_json::to_string(self).expect("Error serde_json::to_string")
        }

    }

    impl Response {
        pub fn dump(&self) -> String {
            serde_json::to_string(self).expect("Error serde_json::to_string")
        }
    }

}

