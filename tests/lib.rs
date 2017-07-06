extern crate serde;
#[macro_use]
extern crate serde_json;
extern crate jsonrpc_lite;

use serde_json::{Value, to_value};
use jsonrpc_lite::JsonRpc;

#[test]
fn request() {
    let jsonrpc = to_value(JsonRpc::request(0, "test"))
        .expect("Unable to turn request into Json Value");
    assert_eq!(
        jsonrpc,
        json!({
            "id": 0,
            "jsonrpc": "2.0",
            "method": "test"
        })
    );
}

#[test]
fn request_with_params() {
    let jsonrpc = to_value(JsonRpc::request_with_params(
        String::from("a"),
        "test",
        vec![Value::Bool(true), Value::Bool(false), Value::Bool(true)],
    )).expect("Unable to turn request_with_params into Json Value");
    assert_eq!(
        jsonrpc,
        json!({
            "id": "a",
            "jsonrpc": "2.0",
            "method": "test",
            "params": [true, false, true]
        })
    );
}
