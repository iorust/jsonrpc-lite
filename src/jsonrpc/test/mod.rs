use {JsonRpc, RpcError};
use serde_json::{Value, to_value, to_string};

mod request;
mod notification;
mod success;
mod error;

pub(self) fn test_value(val: Value, name: &'static str) -> (bool, JsonRpc) {
    let rpc = JsonRpc::from_str(
        &to_string(&val).expect(&format!("Unable to turn {} into a String", name)),
    ).expect(&format!("Unable to turn {} into a JsonRpc", name));
    (
        val.clone() == to_value(rpc.clone()).expect(&format!("Unable to turn {} into a Json Value", name)),
        rpc,
    )
}

#[test]
fn call_pos_param() {
    assert_eq!(
        JsonRpc::from_str(r#"{"jsonrpc": "2.0", "method": "subtract", "params": [42, 23], "id": 1}"#).expect("Unable to make substract #1 request from_str"),
        JsonRpc::request_with_params(1, "subtract", json!([42, 23])),
        "Failed to equate substract #1 request"
    );
    assert_eq!(
        JsonRpc::from_str(r#"{"jsonrpc": "2.0", "result": 19, "id": 1}"#).expect("Unable to make substract #1 success from_str"),
        JsonRpc::success(1, &json!(19)),
        "Failed to equate substract #1 success"
    );

    assert_eq!(
        JsonRpc::from_str(r#"{"jsonrpc": "2.0", "method": "subtract", "params": [23, 42], "id": 2}"#).expect("Unable to make substract #2 request from_str"),
        JsonRpc::request_with_params(2, "subtract", json!([23, 42])),
        "Failed to equate substract #2 request"
    );
    assert_eq!(
        JsonRpc::from_str(r#"{"jsonrpc": "2.0", "result": -19, "id": 2}"#).expect("Unable to make substract #2 success from_str"),
        JsonRpc::success(2, &json!(-19)),
        "Failed to equate substract #2 success"
    );

    assert_ne!(
        JsonRpc::request_with_params(1, "subtract", json!([42, 23])),
        JsonRpc::request_with_params(1, "subtract", json!([23, 42])),
        "Equated params in wrong order"
    );

    assert_ne!(
        JsonRpc::from_str(r#"{"jsonrpc": "2.0", "method": "subtract", "params": [42, 23], "id": 1}"#)
            .expect("Unable to make substract #1 request from_str, while checking param eq"),
        JsonRpc::from_str(r#"{"jsonrpc": "2.0", "method": "subtract", "params": [23, 42], "id": 1}"#)
            .expect("Unable to make substract #2 request from_str, while checking param eq"),
        "Equated params in wrong order, from_str"
    );
}

#[test]
fn call_named_param() {
    assert_eq!(
        JsonRpc::from_str(r#"{"jsonrpc": "2.0", "method": "subtract", "params": {"subtrahend": 23, "minuend": 42}, "id": 3}"#)
            .expect("Unable to make substract #3 request from_str"),
        JsonRpc::request_with_params(3, "subtract", json!({"subtrahend": 23, "minuend": 42})),
        "Failed to equate substract #3 request"
    );
    assert_eq!(
        JsonRpc::from_str(r#"{"jsonrpc": "2.0", "result": 19, "id": 3}"#)
            .expect("Unable to make substract #3 success from_str"),
        JsonRpc::success(3, &json!(19)),
        "Failed to equate substract #3 success"
    );

    assert_eq!(
        JsonRpc::from_str(r#"{"jsonrpc": "2.0", "method": "subtract", "params": {"minuend": 42, "subtrahend": 23}, "id": 4}"#)
            .expect("Unable to make substract #4 request from_str"),
        JsonRpc::request_with_params(4, "subtract", json!({"minuend": 42, "subtrahend": 23})),
        "Failed to equate substract #4 request"
    );
    assert_eq!(
        JsonRpc::from_str(r#"{"jsonrpc": "2.0", "result": 19, "id": 4}"#)
            .expect("Unable to make substract #4 success from_str"),
        JsonRpc::success(4, &json!(19)),
        "Failed to equate substract #4 success"
    );

    assert_eq!(
        JsonRpc::request_with_params(1, "subtract", json!({"subtrahend": 23, "minuend": 42})),
        JsonRpc::request_with_params(1, "subtract", json!({"minuend": 42, "subtrahend": 23})),
        "Params out of order in map"
    );

    assert_eq!(
        JsonRpc::from_str(r#"{"jsonrpc": "2.0", "method": "subtract", "params": {"subtrahend": 23, "minuend": 42}, "id": 3}"#)
            .expect("Unable to make substract #1 request from_str, while checking param eq"),
        JsonRpc::from_str(r#"{"jsonrpc": "2.0", "method": "subtract", "params": {"minuend": 42, "subtrahend": 23}, "id": 3}"#)
            .expect("Unable to make substract #2 request from_str, while checking param eq"),
        "Params out of order in map, from_str"
    );
}

#[test]
fn call_invalid_json() {
    assert_eq!(
        JsonRpc::from_str(r#"{"jsonrpc": "2.0", "method": "foobar, "params": "bar", "baz]"#).expect("Expected Error return not found"),
        JsonRpc::error((), RpcError::parse_error()),
        "Invalid Json should lead to parse error"
    );
}

#[test]
fn call_invalid_req() {
    assert_eq!(
        JsonRpc::from_str(r#"{"jsonrpc": "2.0", "method": 1, "params": "bar"}"#).expect("Expected Error return not found"),
        JsonRpc::error((), RpcError::invalid_request()),
        "Invalid Request should lead to invalid request error"
    );
}

#[test]
fn call_batch_invalid_json() {
    let res = JsonRpc::from_str_vec(
        r#"[
            {"jsonrpc": "2.0", "method": "sum", "params": [1,2,4], "id": "1"},
            {"jsonrpc": "2.0", "method"
        ]"#,
    );
    assert!(res.len() == 1, "Batch request with invalid Json had more than one value");
    assert_eq!(
        res[0].as_ref().expect("Expected Error return not found"),
        &JsonRpc::error((), RpcError::parse_error()),
        "Single value from invalid Json batch call was not a parse error"
    );
}

#[test]
fn call_batch_empty() {
    let res = JsonRpc::from_str_vec("[]");
    assert!(res.len() == 1, "Batch request with empty array had more than one value");
    assert_eq!(
        res[0].as_ref().expect("Expected Error return not found"),
        &JsonRpc::error((), RpcError::invalid_request()),
        "Single value from empty batch call was not an invalid request error"
    );
}

#[test]
fn call_batch_invalid_req_single() {
    let res = JsonRpc::from_str_vec("[1]");
    assert!(res.len() == 1, "Batch request with single invalid request had more than one value");
    assert_eq!(
        res[0].as_ref().expect("Expected Error return not found"),
        &JsonRpc::error((), RpcError::invalid_request()),
        "Single value from invalid request batch call was not an invalid request error"
    );
}

#[test]
fn call_batch_invalid_req_multi() {
    for val in JsonRpc::from_str_vec("[1,2,3]") {
        assert_eq!(
            val.as_ref().expect("Expected Error return not found"),
            &JsonRpc::error((), RpcError::invalid_request()),
            "A value from invalid request batch call was not an invalid request error"
        );
    }
}

#[test]
fn call_batch_valid_and_invalid() {
    let vals = JsonRpc::from_str_vec(
        r#"[
            {"jsonrpc": "2.0", "method": "sum", "params": [1,2,4], "id": "1"},
            {"jsonrpc": "2.0", "method": "notify_hello", "params": [7]},
            {"jsonrpc": "2.0", "method": "subtract", "params": [42,23], "id": "2"},
            {"foo": "boo"},
            {"jsonrpc": "2.0", "method": "foo.get", "params": {"name": "myself"}, "id": "5"},
            {"jsonrpc": "2.0", "method": "get_data", "id": "9"}
        ]"#,
    );
    assert_eq!(
        vals[0].as_ref().expect("Expected request not found"),
        &JsonRpc::request_with_params("1", "sum", json!([1,2,4])),
        "First value from batch call with a mix of valid and invalid, failed to match itself"
    );
    assert_eq!(
        vals[1].as_ref().expect("Expected request not found"),
        &JsonRpc::notification_with_params("notify_hello", json!([7])),
        "Second value from batch call with a mix of valid and invalid, failed to match itself"
    );
    assert_eq!(
        vals[2].as_ref().expect("Expected request not found"),
        &JsonRpc::request_with_params("2", "subtract", json!([42,23])),
        "Third value from batch call with a mix of valid and invalid, failed to match itself"
    );
    assert_eq!(
        vals[3].as_ref().expect("Expected request not found"),
        &JsonRpc::error((), RpcError::invalid_request()),
        "Fourth value from batch call with a mix of valid and invalid, failed to match itself"
    );
    assert_eq!(
        vals[4].as_ref().expect("Expected request not found"),
        &JsonRpc::request_with_params("5", "foo.get", json!({"name": "myself"})),
        "Fifth value from batch call with a mix of valid and invalid, failed to match itself"
    );
    assert_eq!(
        vals[5].as_ref().expect("Expected request not found"),
        &JsonRpc::request("9", "get_data"),
        "Sixth value from batch call with a mix of valid and invalid, failed to match itself"
    );
}
