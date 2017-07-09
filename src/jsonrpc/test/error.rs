use {JsonRpc, RpcError};

#[test]
fn data() {
    use serde_json::to_value;

    assert_eq!(
        to_value(JsonRpc::error(
            42,
            RpcError::parse_error().with_data(json!({
                "fail": "reasons",
                "time": 0
            }))
        )).expect("Unable to turn error_data into a Json Value"),
        json!({
            "id": 42,
            "jsonrpc": "2.0",
            "error": {
                "code": -32700,
                "message": "Parse error",
                "data": {
                    "fail": "reasons",
                    "time": 0
                }
            }
        }),
        "Failed to make JsonRpc::Error with data and Id::Num"
    );
}

#[test]
fn no_data() {
    use serde_json::to_value;

    assert_eq!(
        to_value(JsonRpc::error((), RpcError::parse_error()))
            .expect("Unable to turn error_no_data into a Json Value"),
        json!({
            "id": null,
            "jsonrpc": "2.0",
            "error": {
                "code": -32700,
                "message": "Parse error"
            }
        }),
        "Failed to make JsonRpc::Error with no data and Id::None"
    );
}

#[test]
fn get_id_unit() {
    assert_eq!(
        JsonRpc::error((), RpcError::parse_error()).get_id(),
        Some(().into()),
        "Failed to get Id::None out of a JsonRpc::Error with Id::None"
    );
}

#[test]
fn get_id_num() {
    assert_eq!(
        JsonRpc::error(42, RpcError::parse_error()).get_id(),
        Some(42.into()),
        "Failed to get Id::Num out of a JsonRpc::Error with Id::Num"
    );
}

#[test]
fn get_id_str() {
    assert_eq!(
        JsonRpc::error("test", RpcError::parse_error()).get_id(),
        Some("test".into()),
        "Failed to get Id::Str out of a JsonRpc::Error with Id::Str"
    );
}

#[test]
fn get_method() {
    assert!(JsonRpc::error((), RpcError::parse_error()).get_method().is_none(), "Got a method form a JsonRpc::Error");
}

#[test]
fn get_params() {
    assert!(JsonRpc::error((), RpcError::parse_error()).get_params().is_none(), "Got params form a JsonRpc::Error");
}

#[test]
fn get_result() {
    assert!(JsonRpc::error((), RpcError::parse_error()).get_result().is_none(), "Got a result form a JsonRpc::Error");
}

#[test]
fn get_error_no_data() {
    let err = RpcError::parse_error();
    assert_eq!(
        JsonRpc::error((), err.clone()).get_error(),
        Some(&err),
        "Failed to get error out of a JsonRpc::Error with no data"
    );
}

#[test]
fn get_error_data() {
    let err = RpcError::parse_error().with_data(json!(true));
    assert_eq!(
        JsonRpc::error((), err.clone()).get_error(),
        Some(&err),
        "Failed to get error out of a JsonRpc::Error with data"
    );
}

#[test]
fn parse_none() {
    let (is_match, rpc) = super::test_value(
        json!({
            "jsonrpc": "2.0",
            "error": {
                "code": -32700,
                "message": "Parse error"
            },
            "id": null
        }),
        "parse_none_error",
    );
    assert!(is_match, "None JsonRpc::Error Json Value failed to match itself");
    assert!(rpc.is_error(), "None JsonRpc::Error didn't become a success");
    assert_eq!(Some(().into()), rpc.get_id(), "Failed to match Id::None in None JsonRpc::Error");
    assert_eq!(Some(&RpcError::parse_error()), rpc.get_error(), "get error in None JsonRpc::Error");
}

#[test]
fn parse_single() {
    let (is_match, rpc) = super::test_value(
        json!({
            "jsonrpc": "2.0",
            "error": {
                "code": -32700,
                "message": "Parse error",
                "data": true
            },
            "id": null
        }),
        "parse_single_error",
    );
    assert!(is_match, "Single JsonRpc::Error Json Value failed to match itself");
    assert!(rpc.is_error(), "Single JsonRpc::Error didn't become a success");
    assert_eq!(Some(().into()), rpc.get_id(), "Failed to match Id::None in Single JsonRpc::Error");
    assert_eq!(Some(&RpcError::parse_error().with_data(json!(true))), rpc.get_error(), "get error in Single JsonRpc::Error");
}

#[test]
fn parse_vec() {
    let (is_match, rpc) = super::test_value(
        json!({
            "jsonrpc": "2.0",
            "error": {
                "code": -32700,
                "message": "Parse error",
                "data": ["alpha", "bunny", "kilo", "delta"]
            },
            "id": 42
        }),
        "parse_vec_error",
    );
    assert!(is_match, "Vec JsonRpc::Error Json Value failed to match itself");
    assert!(rpc.is_error(), "Vec JsonRpc::Error didn't become a success");
    assert_eq!(Some(42.into()), rpc.get_id(), "Failed to match Id::Num in Vec JsonRpc::Error");
    assert_eq!(
        Some(&RpcError::parse_error().with_data(json!(["alpha", "bunny", "kilo", "delta"]))),
        rpc.get_error(),
        "get error in Vec JsonRpc::Error"
    );
}

#[test]
fn parse_map() {
    let (is_match, rpc) = super::test_value(
        json!({
            "jsonrpc": "2.0",
            "error": {
                "code": -32700,
                "message": "Parse error",
                "data": {
                    "data": [42, 12, 7, 64, 92, 128],
                    "date": "today"
                }
            },
            "id": "alpha"
        }),
        "parse_map_error",
    );
    assert!(is_match, "Map JsonRpc::Error Json Value failed to match itself");
    assert!(rpc.is_error(), "Map JsonRpc::Error didn't become a success");
    assert_eq!(Some("alpha".into()), rpc.get_id(), "Failed to match Id::Str in Map JsonRpc::Error");
    assert_eq!(
        Some(&RpcError::parse_error().with_data(json!({
            "data": [42, 12, 7, 64, 92, 128],
            "date": "today"
        }))),
        rpc.get_error(),
        "get error in Map JsonRpc::Error"
    );
}
