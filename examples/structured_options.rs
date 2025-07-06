use capture_rust::{Capture, ContentOptions, MetadataOptions, PdfOptions, ScreenshotOptions};
use std::collections::HashMap;
use std::fs;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let api_key =
        std::env::var("CAPTURE_KEY").expect("CAPTURE_KEY environment variable is required");
    let api_secret =
        std::env::var("CAPTURE_SECRET").expect("CAPTURE_SECRET environment variable is required");

    let capture = Capture::new(api_key, api_secret);

    // Example 1: Screenshot with structured options
    let screenshot_options = ScreenshotOptions {
        vw: Some(1920),
        vh: Some(1080),
        full: Some(true),
        delay: Some(3),
        dark_mode: Some(true),
        image_type: Some("png".to_string()),
        block_cookie_banners: Some(true),
        ..Default::default()
    };

    println!("Building screenshot URL with structured options...");
    let screenshot_url =
        capture.build_screenshot_url("https://capture.page/", Some(&screenshot_options))?;
    println!("Screenshot URL: {screenshot_url}");

    // Example 2: PDF with structured options
    let pdf_options = PdfOptions {
        format: Some("A4".to_string()),
        landscape: Some(true),
        margin_top: Some("1cm".to_string()),
        margin_bottom: Some("1cm".to_string()),
        margin_left: Some("1cm".to_string()),
        margin_right: Some("1cm".to_string()),
        delay: Some(2),
        ..Default::default()
    };

    println!("Building PDF URL with structured options...");
    let pdf_url = capture.build_pdf_url_structured("https://capture.page/", Some(&pdf_options))?;
    println!("PDF URL: {pdf_url}");

    // Example 3: Content with structured options
    let content_options = ContentOptions {
        delay: Some(1),
        wait_for: Some("#main-content".to_string()),
        ..Default::default()
    };

    println!("Building content URL with structured options...");
    let content_url =
        capture.build_content_url_structured("https://capture.page/", Some(&content_options))?;
    println!("Content URL: {content_url}");

    // Example 4: Using generic override mechanism
    let mut additional_options = HashMap::new();
    additional_options.insert(
        "futureOption".to_string(),
        serde_json::Value::String("futureValue".to_string()),
    );
    additional_options.insert(
        "anotherOption".to_string(),
        serde_json::Value::Number(42.into()),
    );

    let screenshot_with_override = ScreenshotOptions {
        vw: Some(1440),
        vh: Some(900),
        full: Some(true),
        additional_options: Some(additional_options),
        ..Default::default()
    };

    println!("Building screenshot URL with override options...");
    let override_url =
        capture.build_screenshot_url("https://capture.page/", Some(&screenshot_with_override))?;
    println!("Override URL: {override_url}");

    // Example 5: Metadata with generic options only
    let mut metadata_additional = HashMap::new();
    metadata_additional.insert(
        "customParam".to_string(),
        serde_json::Value::String("customValue".to_string()),
    );

    let metadata_options = MetadataOptions {
        additional_options: Some(metadata_additional),
    };

    println!("Building metadata URL with generic options...");
    let metadata_url =
        capture.build_metadata_url_structured("https://capture.page/", Some(&metadata_options))?;
    println!("Metadata URL: {metadata_url}");

    // Example 6: Fetch screenshot with structured options
    println!("Fetching screenshot with structured options...");
    let screenshot_data = capture
        .fetch_screenshot("https://capture.page/", Some(&screenshot_options))
        .await?;
    fs::write("structured_screenshot.png", screenshot_data)?;
    println!("Screenshot saved as structured_screenshot.png");

    // Example 7: Fetch PDF with structured options
    println!("Fetching PDF with structured options...");
    let pdf_data = capture
        .fetch_pdf_structured("https://capture.page/", Some(&pdf_options))
        .await?;
    fs::write("structured_page.pdf", pdf_data)?;
    println!("PDF saved as structured_page.pdf");

    // Example 8: Fetch content with structured options
    println!("Fetching content with structured options...");
    let content = capture
        .fetch_content_structured("https://capture.page/", Some(&content_options))
        .await?;
    println!("Content success: {}", content.success);
    println!("HTML length: {}", content.html.len());
    println!("Text content length: {}", content.text_content.len());

    // Example 9: Fetch metadata with structured options
    println!("Fetching metadata with structured options...");
    let metadata = capture
        .fetch_metadata_structured("https://capture.page/", Some(&metadata_options))
        .await?;
    println!("Metadata success: {}", metadata.success);
    println!(
        "Metadata keys: {:?}",
        metadata.metadata.keys().collect::<Vec<_>>()
    );

    Ok(())
}
