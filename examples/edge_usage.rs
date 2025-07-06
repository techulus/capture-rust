use capture_rust::Capture;
use std::collections::HashMap;
use std::fs;
use std::time::Duration;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let api_key =
        std::env::var("CAPTURE_KEY").expect("CAPTURE_KEY environment variable is required");
    let api_secret =
        std::env::var("CAPTURE_SECRET").expect("CAPTURE_SECRET environment variable is required");

    // Using builder pattern
    let capture = Capture::new(api_key, api_secret)
        .with_edge()
        .with_timeout(Duration::from_secs(30));

    let mut request_options = HashMap::new();
    request_options.insert("full".to_string(), serde_json::Value::Bool(true));
    request_options.insert(
        "delay".to_string(),
        serde_json::Value::Number(serde_json::Number::from(3)),
    );
    request_options.insert(
        "t".to_string(),
        serde_json::Value::Number(serde_json::Number::from(
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
        )),
    );

    println!("Using edge endpoint...");

    println!("Building image URL with edge...");
    let image_url = capture.build_image_url("https://capture.page/", Some(&request_options))?;
    println!("Image URL: {}", image_url);

    println!("Fetching image via edge...");
    let image_data = capture
        .fetch_image("https://capture.page/", Some(&request_options))
        .await?;
    fs::write("edge_screenshot.png", image_data)?;
    println!("Edge screenshot saved as edge_screenshot.png");

    Ok(())
}
