use JsonRpc;

#[test]
fn single() {
    use serde_json::to_value;

    assert_eq!(
        to_value(JsonRpc::success((), &json!(true))).expect("Unable to turn success_single into a Json Value"),
        json!({
            "id": null,
            "jsonrpc": "2.0",
            "result": true
        }),
        "Failed to make JsonRpc::Success with single result and Id::None"
    );
}

#[test]
fn vec() {
    use serde_json::to_value;

    assert_eq!(
        to_value(JsonRpc::success(345, &json!([4, 9, 5, 4, 6, 6, 3, 7, 4, 5])))
            .expect("Unable to turn success_vec into a Json Value"),
        json!({
            "id": 345,
            "jsonrpc": "2.0",
            "result": [4, 9, 5, 4, 6, 6, 3, 7, 4, 5]
        }),
        "Failed to make JsonRpc::Success with vec result and Id::Num"
    );
}

#[test]
fn map() {
    use serde_json::to_value;

    assert_eq!(
        to_value(JsonRpc::success("delta-nova", &json!({
            "random": {
                "data": [4, 9, 5, 4, 6, 6, 3, 7, 4, 5],
                "completionTime": "2017-07-05 21:21:40Z"
            },
            "bitsUsed": 33,
            "bitsLeft": 980134,
            "requestsLeft": 199969,
            "advisoryDelay": 50
        }))).expect("Unable to turn success_map into a Json Value"),
        json!({
            "id": "delta-nova",
            "jsonrpc": "2.0",
            "result": {
                "random": {
                    "data": [4, 9, 5, 4, 6, 6, 3, 7, 4, 5],
                    "completionTime": "2017-07-05 21:21:40Z"
                },
                "bitsUsed": 33,
                "bitsLeft": 980134,
                "requestsLeft": 199969,
                "advisoryDelay": 50
            }
        }),
        "Failed to make JsonRpc::Success with map result and Id::Str"
    );
}

#[test]
fn get_id_unit() {
    assert_eq!(
        JsonRpc::success((), &json!(true)).get_id(),
        Some(().into()),
        "Failed to get Id::None out of a JsonRpc::Success with Id::None"
    );
}

#[test]
fn get_id_num() {
    assert_eq!(
        JsonRpc::success(42, &json!(true)).get_id(),
        Some(42.into()),
        "Failed to get Id::Num out of a JsonRpc::Success with Id::Num"
    );
}

#[test]
fn get_id_str() {
    assert_eq!(
        JsonRpc::success("test", &json!(true)).get_id(),
        Some("test".into()),
        "Failed to get Id::Str out of a JsonRpc::Success with Id::Str"
    );
}

#[test]
fn get_method() {
    assert!(JsonRpc::success((), &json!(true)).get_method().is_none(), "Got a method form a JsonRpc::Success");
}

#[test]
fn get_params() {
    assert!(JsonRpc::success((), &json!(true)).get_params().is_none(), "Got params form a JsonRpc::Success");
}

#[test]
fn get_result_single() {
    assert_eq!(
        JsonRpc::success((), &json!(true)).get_result(),
        Some(&json!(true)),
        "Failed to get a single result out of a JsonRpc::Success"
    );
}

#[test]
fn get_result_vec() {
    assert_eq!(
        JsonRpc::success((), &json!([true, false, false, true])).get_result(),
        Some(&json!([true, false, false, true])),
        "Failed to get a vec result out of a JsonRpc::Success"
    );
}

#[test]
fn get_result_map() {
    assert_eq!(
        JsonRpc::success((), &json!({
            "data": "test",
            "things": null
        })).get_result(),
        Some(&json!({
            "data": "test",
            "things": null
        })),
        "Failed to get a map result out of a JsonRpc::Success"
    );
}

#[test]
fn get_error() {
    assert!(JsonRpc::success((), &json!(true)).get_error().is_none(), "Got an error form a JsonRpc::Success");
}

#[test]
fn parse_single() {
    let (is_match, rpc) = super::test_value(
        json!({
            "jsonrpc": "2.0",
            "result": true,
            "id": null
        }),
        "parse_single_success",
    );
    assert!(is_match, "Single JsonRpc::Success Json Value failed to match itself");
    assert!(rpc.is_success(), "Single JsonRpc::Success didn't become a success");
    assert_eq!(Some(().into()), rpc.get_id(), "Failed to match Id::None in Single JsonRpc::Success");
    assert_eq!(Some(&json!(true).into()), rpc.get_result(), "get result in Single JsonRpc::Success");
}

#[test]
fn parse_vec() {
    let (is_match, rpc) = super::test_value(
        json!({
            "jsonrpc": "2.0",
            "result": ["alpha", "bunny", "kilo", "delta"],
            "id": 42
        }),
        "parse_vec_success",
    );
    assert!(is_match, "Vec JsonRpc::Success Json Value failed to match itself");
    assert!(rpc.is_success(), "Vec JsonRpc::Success didn't become a success");
    assert_eq!(Some(42.into()), rpc.get_id(), "Failed to match Id::Num in Vec JsonRpc::Success");
    assert_eq!(
        Some(&json!(["alpha", "bunny", "kilo", "delta"]).into()),
        rpc.get_result(),
        "get result in Vec JsonRpc::Success"
    );
}

#[test]
fn parse_map() {
    let (is_match, rpc) = super::test_value(
        json!({
            "jsonrpc": "2.0",
            "result": {
                "data": [42, 12, 7, 64, 92, 128],
                "date": "today"
            },
            "id": "alpha"
        }),
        "parse_map_success",
    );
    assert!(is_match, "Map JsonRpc::Success Json Value failed to match itself");
    assert!(rpc.is_success(), "Map JsonRpc::Success didn't become a success");
    assert_eq!(Some("alpha".into()), rpc.get_id(), "Failed to match Id::Str in Map JsonRpc::Success");
    assert_eq!(
        Some(&json!({
            "data": [42, 12, 7, 64, 92, 128],
            "date": "today"
        }).into()),
        rpc.get_result(),
        "get result in Map JsonRpc::Success"
    );
}
