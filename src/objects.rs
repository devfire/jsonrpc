// contains the JSON RPC objects that are used to communicate with the server.
use serde::{Deserialize, Serialize};
use serde_json::Value;

// fix the jsonrpc string to 2.0 
// const JSONRPC_VERSION: &str ="2.0";

#[derive(Deserialize, Debug)]
pub(crate) struct JsonRpcRequest {
    jsonrpc: String,
    method: String,
    params: Value, // Use Value for flexibility (can be array, object, null)
    id: Option<Value>, // Can be string, number, or null. Option handles notification case (id=null)
}

#[derive(Serialize, Debug)]
struct JsonRpcError {
    code: i32,
    message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    data: Option<Value>,
}

#[derive(Serialize, Debug)]
struct JsonRpcResponse {
    jsonrpc: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    result: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    error: Option<JsonRpcError>,
    id: Option<Value>,
}