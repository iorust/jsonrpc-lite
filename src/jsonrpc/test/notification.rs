use JsonRpc;

#[test]
fn params_none() {
    use serde_json::to_value;

    assert_eq!(
        to_value(JsonRpc::notification("test")).expect("Unable to turn notification into a Json Value"),
        json!({
            "jsonrpc": "2.0",
            "method": "test"
        }),
        "Failed to make JsonRpc::Notification with no Params"
    );
}

#[test]
fn params_vec() {
    use serde_json::to_value;

    assert_eq!(
        to_value(JsonRpc::notification_with_params(
            "test",
            json!([true, false, false, true]),
        )).expect("Unable to turn notification_with_params_vec into a Json Value"),
        json!({
            "jsonrpc": "2.0",
            "method": "test",
            "params": [true, false, false, true]
        }),
        "Failed to make JsonRpc::Notification with Params::Vec"
    );
}

#[test]
fn params_map() {
    use serde_json::to_value;

    assert_eq!(
        to_value(JsonRpc::notification_with_params(
            "test",
            json!({
                "key": "94151351-5651651658-56151351351",
                "n": 5158,
                "mean": 454.54
            }),
        )).expect("Unable to turn notification_with_params_map into a Json Value"),
        json!({
            "jsonrpc": "2.0",
            "method": "test",
            "params": {
                "key": "94151351-5651651658-56151351351",
                "n": 5158,
                "mean": 454.54
            }
        }),
        "Failed to make JsonRpc::Notification with Params::Map"
    );
}

#[test]
fn get_id() {
    assert!(JsonRpc::notification("test").get_id().is_none(), "Got an Id form a JsonRpc::Notification");
}

#[test]
fn get_method() {
    assert_eq!(
        JsonRpc::notification("test").get_method(),
        Some("test".into()),
        "Failed to get method out of a JsonRpc::Notification"
    );
}

#[test]
fn get_params_single() {
    assert_eq!(
        JsonRpc::notification_with_params("test", json!(true)).get_params().expect("Unable to get params from get_params_single"),
        json!(null).into(),
        "Failed to get Params::None out of a JsonRpc::Notification with Params::None"
    );
}

#[test]
fn get_params_vec() {
    assert_eq!(
        JsonRpc::notification_with_params("test", json!([true, false, true])).get_params().expect("Unable to get params from get_params_vec"),
        json!([true, false, true]).into(),
        "Failed to get Params::Vec out of a JsonRpc::Notification with Params::Vec"
    );
}

#[test]
fn get_params_map() {
    assert_eq!(
        JsonRpc::notification_with_params("test", json!({
            "test": true
        })).get_params().expect("Unable to get params from get_params_map"),
        json!({
            "test": true
        }).into(),
        "Failed to get Params::Map out of a JsonRpc::Notification with Params::Map"
    );
}

#[test]
fn get_result() {
    assert!(JsonRpc::notification("test").get_result().is_none(), "Got a result form a JsonRpc::Notification");
}

#[test]
fn get_error() {
    assert!(JsonRpc::notification("test").get_error().is_none(), "Got an error form a JsonRpc::Notification");
}

#[test]
fn parse_simple() {
    let (is_match, rpc) = super::test_value(
        json!({
            "jsonrpc": "2.0",
            "method": "test",
        }),
        "parse_simple_notification",
    );
    assert!(is_match, "Simple JsonRpc::Notification Json Value failed to match itself");
    assert!(rpc.is_notification(), "Simple JsonRpc::Notification didn't become a notification");
    assert_eq!(Some("test".into()), rpc.get_method(), "Failed to get method in Simple JsonRpc::Notification");
    assert!(rpc.get_params().is_none(), "Found Params in Paramless Simple JsonRpc::Notification");
}

#[test]
fn parse_complex_vec() {
    let (is_match, rpc) = super::test_value(
        json!({
            "jsonrpc": "2.0",
            "method": "test",
            "params": ["alpha", "bunny", "kilo", "delta"],
        }),
        "parse_complex_vec_notification",
    );
    assert!(is_match, "Complex Vec JsonRpc::Notification Json Value failed to match itself");
    assert!(rpc.is_notification(), "Complex Vec JsonRpc::Notification didn't become a notification");
    assert_eq!(Some("test".into()), rpc.get_method(), "Failed to get method in Complex Vec JsonRpc::Notification");
    assert_eq!(
        Some(json!(["alpha", "bunny", "kilo", "delta"]).into()),
        rpc.get_params(),
        "Failed to get Params::Vec in Complex Vec JsonRpc::Notification"
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
        }),
        "parse_complex_map_notification",
    );
    assert!(is_match, "Complex Map JsonRpc::Notification Json Value failed to match itself");
    assert!(rpc.is_notification(), "Complex Map JsonRpc::Notification didn't become a notification");
    assert_eq!(Some("test".into()), rpc.get_method(), "Failed to get method in Complex Map JsonRpc::Notification");
    assert_eq!(
        Some(json!({
            "data": [42, 12, 7, 64, 92, 128],
            "date": "today"
        }).into()),
        rpc.get_params(),
        "Failed to get Params::Map in Complex Map JsonRpc::Notification"
    );
}
