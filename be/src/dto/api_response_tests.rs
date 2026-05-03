use std::time::Duration;

use serde::Serialize;
use serde_json::{Value, json};

use crate::dto::api_response::{ApiMeta, ApiResponse, ApiTimer};

#[derive(Debug, Serialize)]
struct TestData {
    value: u8,
}

#[test]
fn success_response_serializes_with_standard_shape() {
    let response = ApiResponse::ok(TestData { value: 7 });
    let serialized_result = serde_json::to_value(response.into_body());
    assert!(serialized_result.is_ok());

    let serialized = match serialized_result {
        Ok(value) => value,
        Err(_) => return,
    };

    assert_eq!(serialized["success"], json!(true));
    assert_eq!(serialized["data"]["value"], json!(7));
    assert_eq!(serialized["error"], Value::Null);
    assert!(serialized["meta"]["timestamp"].is_string());
    assert_eq!(serialized["meta"]["processing_duration"], json!("PT0S"));
}

#[tokio::test]
async fn success_response_uses_scoped_api_timer() {
    let serialized_result = ApiTimer::start()
        .scope(async {
            tokio::time::sleep(Duration::from_millis(5)).await;
            let response = ApiResponse::ok(TestData { value: 7 });
            serde_json::to_value(response.into_body())
        })
        .await;
    assert!(serialized_result.is_ok());

    let serialized = match serialized_result {
        Ok(value) => value,
        Err(_) => return,
    };
    let processing_duration = match serialized["meta"]["processing_duration"].as_str() {
        Some(value) => value,
        None => return,
    };

    assert!(processing_duration.starts_with("PT0."));
}

#[test]
fn meta_details_are_optional() {
    let meta = ApiMeta::with_processing_duration(Duration::from_micros(1042)).with_details(json!({
        "page": 1
    }));
    let serialized_result = serde_json::to_value(meta);
    assert!(serialized_result.is_ok());

    let serialized = match serialized_result {
        Ok(value) => value,
        Err(_) => return,
    };

    assert_eq!(serialized["processing_duration"], json!("PT0.001042S"));
    assert_eq!(serialized["details"]["page"], json!(1));
}
