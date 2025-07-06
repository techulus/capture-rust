# Capture Rust SDK

Rust SDK for [capture.page](https://capture.page) - Browser automation and screenshot API.

Get your API Key and Secret from [https://capture.page/console](https://capture.page/console)

List of all capture options: [https://docs.capture.page](https://docs.capture.page)

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
capture-rust = "0.1.0"
```

## Usage

### Basic Usage

```rust
use capture_rust::{Capture, RequestOptions};
use std::collections::HashMap;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let capture = Capture::new("your_api_key".to_string(), "your_api_secret".to_string());
    
    let mut options = HashMap::new();
    options.insert("full".to_string(), serde_json::Value::Bool(true));
    options.insert("delay".to_string(), serde_json::Value::Number(serde_json::Number::from(3)));
    
    // Build URL
    let url = capture.build_image_url("https://capture.page/", Some(&options))?;
    println!("Screenshot URL: {}", url);
    
    // Fetch image data
    let image_data = capture.fetch_image("https://capture.page/", Some(&options)).await?;
    std::fs::write("screenshot.png", image_data)?;
    
    Ok(())
}
```

### Using Edge Endpoint

```rust
use capture_rust::{Capture, CaptureOptions};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let options = CaptureOptions { use_edge: true };
    let capture = Capture::with_options("your_api_key".to_string(), "your_api_secret".to_string(), options);
    
    let image_data = capture.fetch_image("https://capture.page/", None).await?;
    std::fs::write("edge_screenshot.png", image_data)?;
    
    Ok(())
}
```

### Image Capture

```rust
use capture_rust::Capture;
use std::collections::HashMap;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let capture = Capture::new("your_api_key".to_string(), "your_api_secret".to_string());
    
    let mut options = HashMap::new();
    options.insert("full".to_string(), serde_json::Value::Bool(true));
    options.insert("delay".to_string(), serde_json::Value::Number(serde_json::Number::from(3)));
    
    // Build URL
    let url = capture.build_image_url("https://capture.page/", Some(&options))?;
    
    // Fetch image
    let image_data = capture.fetch_image("https://capture.page/", Some(&options)).await?;
    std::fs::write("screenshot.png", image_data)?;
    
    Ok(())
}
```

### PDF Generation

```rust
use capture_rust::Capture;
use std::collections::HashMap;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let capture = Capture::new("your_api_key".to_string(), "your_api_secret".to_string());
    
    let mut options = HashMap::new();
    options.insert("full".to_string(), serde_json::Value::Bool(true));
    options.insert("delay".to_string(), serde_json::Value::Number(serde_json::Number::from(3)));
    
    // Build URL
    let url = capture.build_pdf_url("https://capture.page/", Some(&options))?;
    
    // Fetch PDF
    let pdf_data = capture.fetch_pdf("https://capture.page/", Some(&options)).await?;
    std::fs::write("page.pdf", pdf_data)?;
    
    Ok(())
}
```

### Content Extraction

```rust
use capture_rust::Capture;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let capture = Capture::new("your_api_key".to_string(), "your_api_secret".to_string());
    
    // Build URL
    let url = capture.build_content_url("https://capture.page/")?;
    
    // Fetch content
    let content = capture.fetch_content("https://capture.page/", None).await?;
    println!("Success: {}", content.success);
    println!("HTML: {}", content.html);
    println!("Text: {}", content.text_content);
    
    Ok(())
}
```

### Metadata Extraction

```rust
use capture_rust::Capture;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let capture = Capture::new("your_api_key".to_string(), "your_api_secret".to_string());
    
    // Build URL
    let url = capture.build_metadata_url("https://capture.page/")?;
    
    // Fetch metadata
    let metadata = capture.fetch_metadata("https://capture.page/", None).await?;
    println!("Success: {}", metadata.success);
    println!("Metadata: {:?}", metadata.metadata);
    
    Ok(())
}
```

## Examples

Run the examples with your API credentials:

```bash
# Set your API credentials
export CAPTURE_API_KEY="your_api_key"
export CAPTURE_API_SECRET="your_api_secret"

# Run basic usage example
cargo run --example basic_usage

# Run edge endpoint example
cargo run --example edge_usage
```

## Error Handling

The SDK uses the `CaptureError` enum for error handling:

```rust
use capture_rust::{Capture, CaptureError};

#[tokio::main]
async fn main() {
    let capture = Capture::new("key".to_string(), "secret".to_string());
    
    match capture.fetch_image("https://example.com", None).await {
        Ok(image_data) => {
            println!("Got image data: {} bytes", image_data.len());
        },
        Err(CaptureError::HttpError(e)) => {
            println!("HTTP error: {}", e);
        },
        Err(CaptureError::MissingCredentials) => {
            println!("API key and secret are required");
        },
        Err(e) => {
            println!("Other error: {}", e);
        }
    }
}
```

## API Reference

### `Capture`

The main client for interacting with the capture.page API.

#### Constructors

- `new(key: String, secret: String) -> Self` - Create a new client with API credentials
- `with_options(key: String, secret: String, options: CaptureOptions) -> Self` - Create a client with custom options

#### URL Building Methods

- `build_image_url(url: &str, options: Option<&RequestOptions>) -> Result<String>` - Build image capture URL
- `build_pdf_url(url: &str, options: Option<&RequestOptions>) -> Result<String>` - Build PDF capture URL
- `build_content_url(url: &str, options: Option<&RequestOptions>) -> Result<String>` - Build content extraction URL
- `build_metadata_url(url: &str, options: Option<&RequestOptions>) -> Result<String>` - Build metadata extraction URL

#### Fetch Methods

- `fetch_image(url: &str, options: Option<&RequestOptions>) -> Result<Vec<u8>>` - Fetch image as bytes
- `fetch_pdf(url: &str, options: Option<&RequestOptions>) -> Result<Vec<u8>>` - Fetch PDF as bytes
- `fetch_content(url: &str, options: Option<&RequestOptions>) -> Result<ContentResponse>` - Fetch page content
- `fetch_metadata(url: &str, options: Option<&RequestOptions>) -> Result<MetadataResponse>` - Fetch page metadata

### Types

- `RequestOptions` - HashMap of capture options
- `CaptureOptions` - SDK configuration options
- `ContentResponse` - Response from content extraction
- `MetadataResponse` - Response from metadata extraction
- `CaptureError` - Error types for the SDK

## License

MIT