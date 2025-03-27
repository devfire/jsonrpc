// contains the JSON RPC objects that are used to communicate with the server.
use serde::Deserialize;

// fix the jsonrpc string to 2.0 
const JSONRPC_VERSION: &str ="2.0";

#[derive(Deserialize, Debug)]
pub struct JsonRpcRequest {
    jsonrpc: String,
    method: String,
    params: String,
    id: u32,
}
