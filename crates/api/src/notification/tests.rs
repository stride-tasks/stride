use super::Value;

#[test]
fn value_round_trips_like_json() {
    let input = r#"{"id":"repo-1","enabled":true,"count":3,"items":[1,"two",null]}"#;
    let value: Value = serde_json::from_str(input).unwrap();

    assert!(matches!(value, Value::Map(_)));
    let json = serde_json::to_string(&value).unwrap();
    let expected = serde_json::json!({
        "id": "repo-1",
        "enabled": true,
        "count": 3.0,
        "items": [1.0, "two", null],
    });
    assert_eq!(
        serde_json::from_str::<serde_json::Value>(&json).unwrap(),
        expected
    );
}
