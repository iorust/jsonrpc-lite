extern crate serde;
extern crate serde_json;
extern crate jsonrpc_lite;

use serde_json::{Value, from_str};
use serde_json::value::ToJson;
use jsonrpc_lite::{JsonRPC, Id, Params};

#[test]
fn request() {
    let jsonrpc = JsonRPC::request(&Id::Num(0), "test");
    assert_eq!(jsonrpc.to_json(),
        from_str::<Value>(r#"{"id": 0, "jsonrpc": "2.0", "method": "test"}"#).unwrap());
}

#[test]
fn request_with_params() {
    let jsonrpc = JsonRPC::request_with_params(&Id::Str("a".to_string()), "test",
        &Params::Array(vec![Value::Bool(true)]));
    // println!("{:?}", jsonrpc);
    assert_eq!(jsonrpc.to_json(),
        from_str::<Value>(r#"{"id": "a", "jsonrpc": "2.0", "method": "test",
            "params": [true]}"#).unwrap());
}

// static BATCH_JSON: &'static str = r#"[
//   {"jsonrpc": "2.0", "method": "sum", "params": [1,2,4], "id": "1"},
//   {"jsonrpc": "2.0", "method": "notify_hello", "params": [7]},
//   {"jsonrpc": "2.0", "method": "subtract", "params": [42,23], "id": "2"},
//   {"foo": "boo"},
//   {"jsonrpc": "2.0", "method": "foo.get", "params": {"name": "myself"}, "id": "5"},
//   {"jsonrpc": "2.0", "method": "get_data", "id": "9"},
//   {"jsonrpc": "2.0", "result": 7, "id": "1"},
//   {"jsonrpc": "2.0", "result": 19, "id": "2"},
//   {"jsonrpc": "2.0", "error": {"code": -32600, "message": "Invalid Request"}, "id": null},
//   {"jsonrpc": "2.0", "error": {"code": -32601, "message": "Method not found"}, "id": "5"},
//   {"jsonrpc": "2.0", "result": ["hello", 5], "id": "9"}
// ]"#;
//
// #[test]
// fn parse() {
//     println!("{:?}", JsonRPC::parse(BATCH_JSON));
// }
