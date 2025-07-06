use capture_rust::Capture;
use std::collections::HashMap;
use std::fs;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let api_key =
        std::env::var("CAPTURE_KEY").expect("CAPTURE_KEY environment variable is required");
    let api_secret =
        std::env::var("CAPTURE_SECRET").expect("CAPTURE_SECRET environment variable is required");

    let capture = Capture::new(api_key, api_secret);

    let mut options = HashMap::new();
    options.insert("full".to_string(), serde_json::Value::Bool(true));
    options.insert(
        "delay".to_string(),
        serde_json::Value::Number(serde_json::Number::from(3)),
    );

    println!("Building image URL...");
    let image_url = capture.build_image_url("https://capture.page/", Some(&options))?;
    println!("Image URL: {image_url}");

    println!("Building PDF URL...");
    let pdf_url = capture.build_pdf_url("https://capture.page/", Some(&options))?;
    println!("PDF URL: {pdf_url}");

    println!("Fetching image...");
    let image_data = capture
        .fetch_image("https://capture.page/", Some(&options))
        .await?;
    fs::write("screenshot.png", image_data)?;
    println!("Screenshot saved as screenshot.png");

    println!("Fetching PDF...");
    let pdf_data = capture
        .fetch_pdf("https://capture.page/", Some(&options))
        .await?;
    fs::write("page.pdf", pdf_data)?;
    println!("PDF saved as page.pdf");

    println!("Fetching content...");
    let content = capture.fetch_content("https://capture.page/", None).await?;
    println!("Content success: {}", content.success);
    println!("HTML length: {}", content.html.len());
    println!("Text content length: {}", content.text_content.len());

    println!("Fetching metadata...");
    let metadata = capture
        .fetch_metadata("https://capture.page/", None)
        .await?;
    println!("Metadata success: {}", metadata.success);
    println!(
        "Metadata keys: {:?}",
        metadata.metadata.keys().collect::<Vec<_>>()
    );

    Ok(())
}
