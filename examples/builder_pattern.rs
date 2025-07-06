use capture_rust::{Capture, CaptureOptions};
use std::time::Duration;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let api_key =
        std::env::var("CAPTURE_KEY").expect("CAPTURE_KEY environment variable is required");
    let api_secret =
        std::env::var("CAPTURE_SECRET").expect("CAPTURE_SECRET environment variable is required");

    // Example 1: Using builder pattern with CaptureOptions
    println!("Example 1: Builder pattern with CaptureOptions");
    let options = CaptureOptions::new()
        .with_edge()
        .with_timeout(Duration::from_secs(30));

    let capture = Capture::with_options(api_key.clone(), api_secret.clone(), options);

    let image_url = capture.build_image_url("https://capture.page/", None)?;
    println!("Image URL with edge: {}", image_url);

    // Example 2: Using builder pattern directly on Capture
    println!("\nExample 2: Builder pattern on Capture struct");
    let capture = Capture::new(api_key.clone(), api_secret.clone())
        .with_edge()
        .with_timeout(Duration::from_secs(15));

    let pdf_url = capture.build_pdf_url("https://capture.page/", None)?;
    println!("PDF URL with edge and timeout: {}", pdf_url);

    // Example 3: Custom HTTP client
    println!("\nExample 3: Custom HTTP client");
    let custom_client = reqwest::Client::builder()
        .timeout(Duration::from_secs(60))
        .user_agent("CustomUserAgent/1.0")
        .build()?;

    let capture = Capture::new(api_key.clone(), api_secret.clone()).with_client(custom_client);

    let content_url = capture.build_content_url("https://capture.page/", None)?;
    println!("Content URL with custom client: {}", content_url);

    // Example 4: Chaining multiple configurations
    println!("\nExample 4: Chaining multiple configurations");
    let capture = Capture::new(api_key, api_secret)
        .with_edge()
        .with_timeout(Duration::from_secs(45));

    // Fetch actual content
    println!("Fetching content with chained configuration...");
    let content = capture.fetch_content("https://capture.page/", None).await?;
    println!("Content success: {}", content.success);
    println!("HTML length: {}", content.html.len());

    // Example 5: Using CaptureOptions builder for complex setup
    println!("\nExample 5: Complex CaptureOptions setup");
    let complex_client = reqwest::Client::builder()
        .timeout(Duration::from_secs(120))
        .user_agent("CaptureRustSDK/1.0")
        .build()?;

    let complex_options = CaptureOptions::new()
        .with_edge()
        .with_timeout(Duration::from_secs(90))
        .with_client(complex_client);

    let capture = Capture::with_options(
        std::env::var("CAPTURE_KEY").unwrap_or_default(),
        std::env::var("CAPTURE_SECRET").unwrap_or_default(),
        complex_options,
    );

    let metadata_url = capture.build_metadata_url("https://capture.page/", None)?;
    println!("Metadata URL with complex setup: {}", metadata_url);

    Ok(())
}
