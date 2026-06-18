use capture_rust::{Capture, CaptureOptions, CreateSessionOptions, SessionActionPayload};
use std::collections::HashMap;

#[tokio::test]
async fn test_build_urls() {
    let capture = Capture::new("test_key".to_string(), "test_secret".to_string());

    let mut options = HashMap::new();
    options.insert("full".to_string(), serde_json::Value::Bool(true));
    options.insert(
        "delay".to_string(),
        serde_json::Value::Number(serde_json::Number::from(3)),
    );

    let image_url = capture
        .build_image_url("https://example.com", Some(&options))
        .unwrap();
    assert!(image_url.contains("test_key"));
    assert!(image_url.contains("image"));
    assert!(image_url.contains("full=true"));
    assert!(image_url.contains("delay=3"));

    let pdf_url = capture
        .build_pdf_url("https://example.com", Some(&options))
        .unwrap();
    assert!(pdf_url.contains("test_key"));
    assert!(pdf_url.contains("pdf"));

    let content_url = capture
        .build_content_url("https://example.com", None)
        .unwrap();
    assert!(content_url.contains("test_key"));
    assert!(content_url.contains("content"));

    let metadata_url = capture
        .build_metadata_url("https://example.com", None)
        .unwrap();
    assert!(metadata_url.contains("test_key"));
    assert!(metadata_url.contains("metadata"));
}

#[tokio::test]
async fn test_edge_endpoint() {
    let options = CaptureOptions::new().with_edge();
    let capture = Capture::with_options("test_key".to_string(), "test_secret".to_string(), options);

    let image_url = capture
        .build_image_url("https://example.com", None)
        .unwrap();
    assert!(image_url.contains("edge.capture.page"));
    assert!(!image_url.contains("cdn.capture.page"));
}

#[tokio::test]
async fn test_regular_endpoint() {
    let capture = Capture::new("test_key".to_string(), "test_secret".to_string());

    let image_url = capture
        .build_image_url("https://example.com", None)
        .unwrap();
    assert!(image_url.contains("cdn.capture.page"));
    assert!(!image_url.contains("edge.capture.page"));
}

#[tokio::test]
async fn test_url_encoding() {
    let capture = Capture::new("test_key".to_string(), "test_secret".to_string());

    let mut options = HashMap::new();
    options.insert(
        "selector".to_string(),
        serde_json::Value::String("div.content".to_string()),
    );

    let image_url = capture
        .build_image_url("https://example.com/path with spaces", Some(&options))
        .unwrap();
    assert!(image_url.contains("path%20with%20spaces"));
    assert!(image_url.contains("selector=div.content"));
}

#[tokio::test]
async fn test_token_generation() {
    let capture1 = Capture::new("key1".to_string(), "secret1".to_string());
    let capture2 = Capture::new("key2".to_string(), "secret2".to_string());

    let url1 = capture1
        .build_image_url("https://example.com", None)
        .unwrap();
    let url2 = capture2
        .build_image_url("https://example.com", None)
        .unwrap();

    // Different keys should produce different tokens
    assert_ne!(url1, url2);
}

#[tokio::test]
async fn test_error_handling() {
    let capture = Capture::new("".to_string(), "".to_string());

    let result = capture.build_image_url("https://example.com", None);
    assert!(result.is_err());

    let capture = Capture::new("key".to_string(), "secret".to_string());
    let result = capture.build_image_url("", None);
    assert!(result.is_err());
}

#[tokio::test]
async fn test_request_options_filtering() {
    let capture = Capture::new("test_key".to_string(), "test_secret".to_string());

    let mut options = HashMap::new();
    options.insert(
        "format".to_string(),
        serde_json::Value::String("A4".to_string()),
    );
    options.insert("full".to_string(), serde_json::Value::Bool(true));
    options.insert(
        "empty".to_string(),
        serde_json::Value::String("".to_string()),
    );

    let pdf_url = capture
        .build_pdf_url("https://example.com", Some(&options))
        .unwrap();

    assert!(pdf_url.contains("format=A4"));

    assert!(pdf_url.contains("full=true"));

    assert!(!pdf_url.contains("empty="));
}

#[tokio::test]
#[ignore = "requires live Capture credentials and creates a billable browser session"]
async fn test_live_session_screenshot_example_dot_com() {
    let key = std::env::var("CAPTURE_KEY").expect("CAPTURE_KEY is required");
    let secret = std::env::var("CAPTURE_SECRET").expect("CAPTURE_SECRET is required");
    let capture = Capture::new(key, secret);

    let created = capture
        .create_session(Some(&CreateSessionOptions {
            max_ttl_seconds: Some(120),
            proxy: None,
            bypass_bot_detection: None,
            cdp: None,
        }))
        .await
        .expect("create session");
    let session_id = created["session"]["id"]
        .as_str()
        .expect("created session id")
        .to_string();

    let result = async {
        let mut goto_payload = SessionActionPayload::new();
        goto_payload.insert(
            "url".to_string(),
            serde_json::Value::String("https://example.com".to_string()),
        );

        let goto_response = capture
            .execute_action(&session_id, "goto", Some(&goto_payload))
            .await?;
        assert_eq!(goto_response["success"], true);

        let mut screenshot_payload = SessionActionPayload::new();
        screenshot_payload.insert("fullPage".to_string(), serde_json::Value::Bool(true));

        let screenshot_response = capture
            .execute_action(&session_id, "screenshot", Some(&screenshot_payload))
            .await?;
        assert_eq!(
            screenshot_response["success"], true,
            "screenshot response: {screenshot_response:?}"
        );

        let screenshot = screenshot_response
            .get("result")
            .and_then(|result| {
                if result.get("bodyBase64").is_some() {
                    Some(result)
                } else {
                    result.get("screenshot")
                }
            })
            .or_else(|| screenshot_response.get("screenshot"))
            .or_else(|| {
                if screenshot_response.get("bodyBase64").is_some() {
                    Some(&screenshot_response)
                } else {
                    None
                }
            })
            .unwrap_or_else(|| panic!("screenshot payload missing: {screenshot_response:?}"));

        assert_eq!(
            screenshot["contentType"], "image/png",
            "screenshot response: {screenshot_response:?}"
        );

        let body_base64 = screenshot["bodyBase64"].as_str().unwrap_or("");
        assert!(!body_base64.is_empty());

        Ok::<(), capture_rust::CaptureError>(())
    }
    .await;

    let close_result = capture.close_session(&session_id).await;
    assert!(close_result.is_ok(), "close session: {close_result:?}");

    result.expect("live session screenshot flow");
}
