use JsonRpc;

#[test]
fn params_none() {
    use serde_json::to_value;

    assert_eq!(
        to_value(JsonRpc::request((), "test")).expect("Unable to turn request into a Json Value"),
        json!({
            "id": null,
            "jsonrpc": "2.0",
            "method": "test"
        }),
        "Failed to make JsonRpc::Request with no Params and Id::None"
    );
}

#[test]
fn params_vec() {
    use serde_json::to_value;

    assert_eq!(
        to_value(JsonRpc::request_with_params(
            46714,
            "test",
            json!([true, false, false, true]),
        )).expect("Unable to turn request_with_params_vec into a Json Value"),
        json!({
            "id": 46714,
            "jsonrpc": "2.0",
            "method": "test",
            "params": [true, false, false, true]
        }),
        "Failed to make JsonRpc::Request with Params::Vec and Id::Num"
    );
}

#[test]
fn params_map() {
    use serde_json::to_value;

    assert_eq!(
        to_value(JsonRpc::request_with_params(
            "alpha-gamma-06714",
            "test",
            json!({
                "key": "94151351-5651651658-56151351351",
                "n": 5158,
                "mean": 454.54
            }),
        )).expect("Unable to turn request_with_params_map into a Json Value"),
        json!({
            "id": "alpha-gamma-06714",
            "jsonrpc": "2.0",
            "method": "test",
            "params": {
                "key": "94151351-5651651658-56151351351",
                "n": 5158,
                "mean": 454.54
            }
        }),
        "Failed to make JsonRpc::Request with Params::Map and Id::Str"
    );
}

#[test]
fn get_id_unit() {
    assert_eq!(
        JsonRpc::request((), "test").get_id(),
        Some(().into()),
        "Failed to get Id::None out of a JsonRpc::Request with Id::None"
    );
}

#[test]
fn get_id_num() {
    assert_eq!(
        JsonRpc::request(42, "test").get_id(),
        Some(42.into()),
        "Failed to get Id::Num out of a JsonRpc::Request with Id::Num"
    );
}

#[test]
fn get_id_str() {
    assert_eq!(
        JsonRpc::request("test", "test").get_id(),
        Some("test".into()),
        "Failed to get Id::Str out of a JsonRpc::Request with Id::Str"
    );
}

#[test]
fn get_method() {
    assert_eq!(
        JsonRpc::request((), "test").get_method(),
        Some("test".into()),
        "Failed to get method out of a JsonRpc::Request"
    );
}

#[test]
fn get_params_single() {
    assert_eq!(
        JsonRpc::request_with_params((), "test", json!(true)).get_params().expect("Unable to get params from get_params_single"),
        json!(null).into(),
        "Failed to get Params::None out of a JsonRpc::Request with Params::None"
    );
}

#[test]
fn get_params_vec() {
    assert_eq!(
        JsonRpc::request_with_params((), "test", json!([true, false, true])).get_params().expect("Unable to get params from get_params_vec"),
        json!([true, false, true]).into(),
        "Failed to get Params::Vec out of a JsonRpc::Request with Params::Vec"
    );
}

#[test]
fn get_params_map() {
    assert_eq!(
        JsonRpc::request_with_params((), "test", json!({
            "test": true
        })).get_params().expect("Unable to get params from get_params_map"),
        json!({
            "test": true
        }).into(),
        "Failed to get Params::Map out of a JsonRpc::Request with Params::Map"
    );
}

#[test]
fn get_result() {
    assert!(JsonRpc::request((), "test").get_result().is_none(), "Got a result form a JsonRpc::Request");
}

#[test]
fn get_error() {
    assert!(JsonRpc::request((), "test").get_error().is_none(), "Got an error form a JsonRpc::Request");
}

#[test]
fn parse_simple() {
    let (is_match, rpc) = super::test_value(
        json!({
            "jsonrpc": "2.0",
            "method": "test",
            "id": null
        }),
        "parse_simple_request",
    );
    assert!(is_match, "Simple JsonRpc::Request Json Value failed to match itself");
    assert!(rpc.is_request(), "Simple JsonRpc::Request didn't become a request");
    assert_eq!(Some(().into()), rpc.get_id(), "Failed to match Id::None in Simple JsonRpc::Request");
    assert_eq!(Some("test".into()), rpc.get_method(), "Failed to get method in Simple JsonRpc::Request");
    assert!(rpc.get_params().is_none(), "Found Params in Paramless Simple JsonRpc::Request");
}

#[test]
fn parse_complex_vec() {
    let (is_match, rpc) = super::test_value(
        json!({
            "jsonrpc": "2.0",
            "method": "test",
            "params": ["alpha", "bunny", "kilo", "delta"],
            "id": 4242
        }),
        "parse_complex_vec_request",
    );
    assert!(is_match, "Complex Vec JsonRpc::Request Json Value failed to match itself");
    assert!(rpc.is_request(), "Complex Vec JsonRpc::Request didn't become a request");
    assert_eq!(Some(4242.into()), rpc.get_id(), "Failed to match Id::Num in Complex Vec JsonRpc::Request");
    assert_eq!(Some("test".into()), rpc.get_method(), "Failed to get method in Complex Vec JsonRpc::Request");
    assert_eq!(
        Some(json!(["alpha", "bunny", "kilo", "delta"]).into()),
        rpc.get_params(),
        "Failed to get Params::Vec in Complex Vec JsonRpc::Request"
    );
}

#[test]
fn parse_complex_map() {
    let (is_match, rpc) = super::test_value(
        json!({
            "jsonrpc": "2.0",
            "method": "test",
            "params": {
                "data": [42, 12, 7, 64, 92, 128],
                "date": "today"
            },
            "id": "Alpha-Nine-Two"
        }),
        "parse_complex_map_request",
    );
    assert!(is_match, "Complex Map JsonRpc::Request Json Value failed to match itself");
    assert!(rpc.is_request(), "Complex Map JsonRpc::Request didn't become a request");
    assert_eq!(Some("Alpha-Nine-Two".into()), rpc.get_id(), "Failed to match Id::Str in Complex Map JsonRpc::Request");
    assert_eq!(Some("test".into()), rpc.get_method(), "Failed to get method in Complex Map JsonRpc::Request");
    assert_eq!(
        Some(json!({
            "data": [42, 12, 7, 64, 92, 128],
            "date": "today"
        }).into()),
        rpc.get_params(),
        "Failed to get Params::Map in Complex Map JsonRpc::Request"
    );
}
