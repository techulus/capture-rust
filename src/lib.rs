use reqwest::Client;
use serde::Deserialize;
use std::collections::HashMap;
use std::time::Duration;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum CaptureError {
    #[error("HTTP request failed: {0}")]
    HttpError(#[from] reqwest::Error),
    #[error("URL parsing failed: {0}")]
    UrlError(#[from] url::ParseError),
    #[error("Key and Secret are required")]
    MissingCredentials,
    #[error("URL is required")]
    MissingUrl,
    #[error("URL should be a string")]
    InvalidUrl,
}

pub type Result<T> = std::result::Result<T, CaptureError>;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RequestType {
    Image,
    Pdf,
    Content,
    Metadata,
    Animated,
}

impl RequestType {
    fn as_str(&self) -> &'static str {
        match self {
            RequestType::Image => "image",
            RequestType::Pdf => "pdf",
            RequestType::Content => "content",
            RequestType::Metadata => "metadata",
            RequestType::Animated => "animated",
        }
    }
}

pub type RequestOptions = HashMap<String, serde_json::Value>;

#[derive(Debug, Clone, Default)]
pub struct ScreenshotOptions {
    // Viewport Options
    pub vw: Option<u32>,
    pub vh: Option<u32>,
    pub scale_factor: Option<f64>,

    // Capture Customization
    pub full: Option<bool>,
    pub delay: Option<u32>,
    pub wait_for: Option<String>,
    pub wait_for_id: Option<String>,

    // Visual Modifications
    pub dark_mode: Option<bool>,
    pub transparent: Option<bool>,
    pub selector: Option<String>,
    pub selector_id: Option<String>,

    // Performance/Detection
    pub block_cookie_banners: Option<bool>,
    pub block_ads: Option<bool>,
    pub bypass_bot_detection: Option<bool>,

    // Image Options
    pub image_type: Option<String>,
    pub best_format: Option<bool>,
    pub resize_width: Option<u32>,
    pub resize_height: Option<u32>,

    // Additional Options
    pub http_auth: Option<String>,
    pub user_agent: Option<String>,
    pub fresh: Option<bool>,

    // Generic override for any future options
    pub additional_options: Option<RequestOptions>,
}

#[derive(Debug, Clone, Default)]
pub struct PdfOptions {
    // Authentication
    pub http_auth: Option<String>,
    pub user_agent: Option<String>,

    // Page Dimensions
    pub width: Option<String>,
    pub height: Option<String>,
    pub format: Option<String>,

    // Margins
    pub margin_top: Option<String>,
    pub margin_right: Option<String>,
    pub margin_bottom: Option<String>,
    pub margin_left: Option<String>,

    // Rendering Options
    pub scale: Option<f64>,
    pub landscape: Option<bool>,
    pub delay: Option<u32>,

    // Storage/Output
    pub file_name: Option<String>,
    pub s3_acl: Option<String>,
    pub s3_redirect: Option<bool>,
    pub timestamp: Option<bool>,

    // Generic override for any future options
    pub additional_options: Option<RequestOptions>,
}

#[derive(Debug, Clone, Default)]
pub struct ContentOptions {
    pub http_auth: Option<String>,
    pub user_agent: Option<String>,
    pub delay: Option<u32>,
    pub wait_for: Option<String>,
    pub wait_for_id: Option<String>,

    // Generic override for any future options
    pub additional_options: Option<RequestOptions>,
}

#[derive(Debug, Clone, Default)]
pub struct MetadataOptions {
    // Generic override for any future options
    pub additional_options: Option<RequestOptions>,
}

impl ScreenshotOptions {
    pub fn to_request_options(&self) -> RequestOptions {
        let mut options = RequestOptions::new();

        if let Some(vw) = self.vw {
            options.insert("vw".to_string(), serde_json::Value::Number(vw.into()));
        }
        if let Some(vh) = self.vh {
            options.insert("vh".to_string(), serde_json::Value::Number(vh.into()));
        }
        if let Some(scale_factor) = self.scale_factor {
            if let Some(num) = serde_json::Number::from_f64(scale_factor) {
                options.insert("scaleFactor".to_string(), serde_json::Value::Number(num));
            }
        }
        if let Some(full) = self.full {
            options.insert("full".to_string(), serde_json::Value::Bool(full));
        }
        if let Some(delay) = self.delay {
            options.insert("delay".to_string(), serde_json::Value::Number(delay.into()));
        }
        if let Some(wait_for) = &self.wait_for {
            options.insert(
                "waitFor".to_string(),
                serde_json::Value::String(wait_for.clone()),
            );
        }
        if let Some(wait_for_id) = &self.wait_for_id {
            options.insert(
                "waitForId".to_string(),
                serde_json::Value::String(wait_for_id.clone()),
            );
        }
        if let Some(dark_mode) = self.dark_mode {
            options.insert("darkMode".to_string(), serde_json::Value::Bool(dark_mode));
        }
        if let Some(transparent) = self.transparent {
            options.insert(
                "transparent".to_string(),
                serde_json::Value::Bool(transparent),
            );
        }
        if let Some(selector) = &self.selector {
            options.insert(
                "selector".to_string(),
                serde_json::Value::String(selector.clone()),
            );
        }
        if let Some(selector_id) = &self.selector_id {
            options.insert(
                "selectorId".to_string(),
                serde_json::Value::String(selector_id.clone()),
            );
        }
        if let Some(block_cookie_banners) = self.block_cookie_banners {
            options.insert(
                "blockCookieBanners".to_string(),
                serde_json::Value::Bool(block_cookie_banners),
            );
        }
        if let Some(block_ads) = self.block_ads {
            options.insert("blockAds".to_string(), serde_json::Value::Bool(block_ads));
        }
        if let Some(bypass_bot_detection) = self.bypass_bot_detection {
            options.insert(
                "bypassBotDetection".to_string(),
                serde_json::Value::Bool(bypass_bot_detection),
            );
        }
        if let Some(image_type) = &self.image_type {
            options.insert(
                "type".to_string(),
                serde_json::Value::String(image_type.clone()),
            );
        }
        if let Some(best_format) = self.best_format {
            options.insert(
                "bestFormat".to_string(),
                serde_json::Value::Bool(best_format),
            );
        }
        if let Some(resize_width) = self.resize_width {
            options.insert(
                "resizeWidth".to_string(),
                serde_json::Value::Number(resize_width.into()),
            );
        }
        if let Some(resize_height) = self.resize_height {
            options.insert(
                "resizeHeight".to_string(),
                serde_json::Value::Number(resize_height.into()),
            );
        }
        if let Some(http_auth) = &self.http_auth {
            options.insert(
                "httpAuth".to_string(),
                serde_json::Value::String(http_auth.clone()),
            );
        }
        if let Some(user_agent) = &self.user_agent {
            options.insert(
                "userAgent".to_string(),
                serde_json::Value::String(user_agent.clone()),
            );
        }
        if let Some(fresh) = self.fresh {
            options.insert("fresh".to_string(), serde_json::Value::Bool(fresh));
        }

        // Merge additional options, allowing overrides
        if let Some(additional) = &self.additional_options {
            for (key, value) in additional {
                options.insert(key.clone(), value.clone());
            }
        }

        options
    }
}

impl PdfOptions {
    pub fn to_request_options(&self) -> RequestOptions {
        let mut options = RequestOptions::new();

        if let Some(http_auth) = &self.http_auth {
            options.insert(
                "httpAuth".to_string(),
                serde_json::Value::String(http_auth.clone()),
            );
        }
        if let Some(user_agent) = &self.user_agent {
            options.insert(
                "userAgent".to_string(),
                serde_json::Value::String(user_agent.clone()),
            );
        }
        if let Some(width) = &self.width {
            options.insert(
                "width".to_string(),
                serde_json::Value::String(width.clone()),
            );
        }
        if let Some(height) = &self.height {
            options.insert(
                "height".to_string(),
                serde_json::Value::String(height.clone()),
            );
        }
        if let Some(format) = &self.format {
            options.insert(
                "format".to_string(),
                serde_json::Value::String(format.clone()),
            );
        }
        if let Some(margin_top) = &self.margin_top {
            options.insert(
                "marginTop".to_string(),
                serde_json::Value::String(margin_top.clone()),
            );
        }
        if let Some(margin_right) = &self.margin_right {
            options.insert(
                "marginRight".to_string(),
                serde_json::Value::String(margin_right.clone()),
            );
        }
        if let Some(margin_bottom) = &self.margin_bottom {
            options.insert(
                "marginBottom".to_string(),
                serde_json::Value::String(margin_bottom.clone()),
            );
        }
        if let Some(margin_left) = &self.margin_left {
            options.insert(
                "marginLeft".to_string(),
                serde_json::Value::String(margin_left.clone()),
            );
        }
        if let Some(scale) = self.scale {
            if let Some(num) = serde_json::Number::from_f64(scale) {
                options.insert("scale".to_string(), serde_json::Value::Number(num));
            }
        }
        if let Some(landscape) = self.landscape {
            options.insert("landscape".to_string(), serde_json::Value::Bool(landscape));
        }
        if let Some(delay) = self.delay {
            options.insert("delay".to_string(), serde_json::Value::Number(delay.into()));
        }
        if let Some(file_name) = &self.file_name {
            options.insert(
                "fileName".to_string(),
                serde_json::Value::String(file_name.clone()),
            );
        }
        if let Some(s3_acl) = &self.s3_acl {
            options.insert(
                "s3Acl".to_string(),
                serde_json::Value::String(s3_acl.clone()),
            );
        }
        if let Some(s3_redirect) = self.s3_redirect {
            options.insert(
                "s3Redirect".to_string(),
                serde_json::Value::Bool(s3_redirect),
            );
        }
        if let Some(timestamp) = self.timestamp {
            options.insert("timestamp".to_string(), serde_json::Value::Bool(timestamp));
        }

        // Merge additional options, allowing overrides
        if let Some(additional) = &self.additional_options {
            for (key, value) in additional {
                options.insert(key.clone(), value.clone());
            }
        }

        options
    }
}

impl ContentOptions {
    pub fn to_request_options(&self) -> RequestOptions {
        let mut options = RequestOptions::new();

        if let Some(http_auth) = &self.http_auth {
            options.insert(
                "httpAuth".to_string(),
                serde_json::Value::String(http_auth.clone()),
            );
        }
        if let Some(user_agent) = &self.user_agent {
            options.insert(
                "userAgent".to_string(),
                serde_json::Value::String(user_agent.clone()),
            );
        }
        if let Some(delay) = self.delay {
            options.insert("delay".to_string(), serde_json::Value::Number(delay.into()));
        }
        if let Some(wait_for) = &self.wait_for {
            options.insert(
                "waitFor".to_string(),
                serde_json::Value::String(wait_for.clone()),
            );
        }
        if let Some(wait_for_id) = &self.wait_for_id {
            options.insert(
                "waitForId".to_string(),
                serde_json::Value::String(wait_for_id.clone()),
            );
        }

        // Merge additional options, allowing overrides
        if let Some(additional) = &self.additional_options {
            for (key, value) in additional {
                options.insert(key.clone(), value.clone());
            }
        }

        options
    }
}

impl MetadataOptions {
    pub fn to_request_options(&self) -> RequestOptions {
        let mut options = RequestOptions::new();

        // Merge additional options, allowing overrides
        if let Some(additional) = &self.additional_options {
            for (key, value) in additional {
                options.insert(key.clone(), value.clone());
            }
        }

        options
    }
}

#[derive(Debug, Clone, Default)]
pub struct CaptureOptions {
    pub use_edge: bool,
    pub timeout: Option<Duration>,
    pub client: Option<Client>,
}

impl CaptureOptions {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_edge(mut self) -> Self {
        self.use_edge = true;
        self
    }

    pub fn with_timeout(mut self, timeout: Duration) -> Self {
        self.timeout = Some(timeout);
        self
    }

    pub fn with_client(mut self, client: Client) -> Self {
        self.client = Some(client);
        self
    }
}

#[derive(Debug, Deserialize)]
pub struct ContentResponse {
    pub success: bool,
    pub html: String,
    #[serde(rename = "textContent")]
    pub text_content: String,
    pub markdown: String,
}

#[derive(Debug, Deserialize)]
pub struct MetadataResponse {
    pub success: bool,
    pub metadata: HashMap<String, serde_json::Value>,
}

pub struct Capture {
    key: String,
    secret: String,
    options: CaptureOptions,
    client: Client,
}

impl Capture {
    const API_URL: &'static str = "https://cdn.capture.page";
    const EDGE_URL: &'static str = "https://edge.capture.page";

    pub fn new(key: String, secret: String) -> Self {
        let options = CaptureOptions::default();
        let client = options.client.clone().unwrap_or_else(|| {
            let mut builder = Client::builder();
            if let Some(timeout) = options.timeout {
                builder = builder.timeout(timeout);
            }
            builder.build().unwrap_or_else(|_| Client::new())
        });

        Self {
            key,
            secret,
            options,
            client,
        }
    }

    pub fn with_options(key: String, secret: String, options: CaptureOptions) -> Self {
        let client = options.client.clone().unwrap_or_else(|| {
            let mut builder = Client::builder();
            if let Some(timeout) = options.timeout {
                builder = builder.timeout(timeout);
            }
            builder.build().unwrap_or_else(|_| Client::new())
        });

        Self {
            key,
            secret,
            options,
            client,
        }
    }

    pub fn with_edge(mut self) -> Self {
        self.options.use_edge = true;
        self
    }

    pub fn with_timeout(mut self, timeout: Duration) -> Self {
        self.options.timeout = Some(timeout);
        // Rebuild client with new timeout
        let builder = Client::builder().timeout(timeout);
        self.client = builder.build().unwrap_or_else(|_| Client::new());
        self
    }

    pub fn with_client(mut self, client: Client) -> Self {
        self.client = client;
        self.options.client = Some(self.client.clone());
        self
    }

    fn generate_token(&self, secret: &str, url: &str) -> String {
        format!("{:x}", md5::compute(format!("{secret}{url}")))
    }

    fn to_query_string(&self, options: &RequestOptions) -> String {
        let mut params = Vec::new();

        for (key, value) in options {
            let value_str = match value {
                serde_json::Value::String(s) => s.clone(),
                serde_json::Value::Number(n) => n.to_string(),
                serde_json::Value::Bool(b) => b.to_string(),
                _ => continue,
            };

            if !value_str.is_empty() {
                params.push(format!(
                    "{}={}",
                    urlencoding::encode(key),
                    urlencoding::encode(&value_str)
                ));
            }
        }

        params.join("&")
    }

    fn build_url(
        &self,
        request_type: RequestType,
        url: &str,
        request_options: Option<&RequestOptions>,
    ) -> Result<String> {
        if self.key.is_empty() || self.secret.is_empty() {
            return Err(CaptureError::MissingCredentials);
        }

        if url.is_empty() {
            return Err(CaptureError::MissingUrl);
        }

        let mut options = request_options.cloned().unwrap_or_default();
        options.insert(
            "url".to_string(),
            serde_json::Value::String(url.to_string()),
        );

        let query = self.to_query_string(&options);
        let token = self.generate_token(&self.secret, &query);

        let base_url = if self.options.use_edge {
            Self::EDGE_URL
        } else {
            Self::API_URL
        };

        Ok(format!(
            "{}/{}/{}/{}?{}",
            base_url,
            self.key,
            token,
            request_type.as_str(),
            query
        ))
    }

    pub fn build_image_url(&self, url: &str, options: Option<&RequestOptions>) -> Result<String> {
        self.build_url(RequestType::Image, url, options)
    }

    pub fn build_pdf_url(&self, url: &str, options: Option<&RequestOptions>) -> Result<String> {
        self.build_url(RequestType::Pdf, url, options)
    }

    pub fn build_content_url(&self, url: &str, options: Option<&RequestOptions>) -> Result<String> {
        self.build_url(RequestType::Content, url, options)
    }

    pub fn build_metadata_url(
        &self,
        url: &str,
        options: Option<&RequestOptions>,
    ) -> Result<String> {
        self.build_url(RequestType::Metadata, url, options)
    }

    pub fn build_animated_url(&self, url: &str, options: Option<&RequestOptions>) -> Result<String> {
        self.build_url(RequestType::Animated, url, options)
    }

    // Structured options methods
    pub fn build_screenshot_url(
        &self,
        url: &str,
        options: Option<&ScreenshotOptions>,
    ) -> Result<String> {
        let request_options = options.map(|o| o.to_request_options());
        self.build_url(RequestType::Image, url, request_options.as_ref())
    }

    pub fn build_pdf_url_structured(
        &self,
        url: &str,
        options: Option<&PdfOptions>,
    ) -> Result<String> {
        let request_options = options.map(|o| o.to_request_options());
        self.build_url(RequestType::Pdf, url, request_options.as_ref())
    }

    pub fn build_content_url_structured(
        &self,
        url: &str,
        options: Option<&ContentOptions>,
    ) -> Result<String> {
        let request_options = options.map(|o| o.to_request_options());
        self.build_url(RequestType::Content, url, request_options.as_ref())
    }

    pub fn build_metadata_url_structured(
        &self,
        url: &str,
        options: Option<&MetadataOptions>,
    ) -> Result<String> {
        let request_options = options.map(|o| o.to_request_options());
        self.build_url(RequestType::Metadata, url, request_options.as_ref())
    }

    pub async fn fetch_image(
        &self,
        url: &str,
        options: Option<&RequestOptions>,
    ) -> Result<Vec<u8>> {
        let capture_url = self.build_image_url(url, options)?;
        let response = self.client.get(&capture_url).send().await?;
        let bytes = response.bytes().await?;
        Ok(bytes.to_vec())
    }

    pub async fn fetch_pdf(&self, url: &str, options: Option<&RequestOptions>) -> Result<Vec<u8>> {
        let capture_url = self.build_pdf_url(url, options)?;
        let response = self.client.get(&capture_url).send().await?;
        let bytes = response.bytes().await?;
        Ok(bytes.to_vec())
    }

    pub async fn fetch_content(
        &self,
        url: &str,
        options: Option<&RequestOptions>,
    ) -> Result<ContentResponse> {
        let capture_url = self.build_content_url(url, options)?;
        let response = self.client.get(&capture_url).send().await?;
        let content = response.json::<ContentResponse>().await?;
        Ok(content)
    }

    pub async fn fetch_metadata(
        &self,
        url: &str,
        options: Option<&RequestOptions>,
    ) -> Result<MetadataResponse> {
        let capture_url = self.build_metadata_url(url, options)?;
        let response = self.client.get(&capture_url).send().await?;
        let metadata = response.json::<MetadataResponse>().await?;
        Ok(metadata)
    }

    pub async fn fetch_animated(
        &self,
        url: &str,
        options: Option<&RequestOptions>,
    ) -> Result<Vec<u8>> {
        let capture_url = self.build_animated_url(url, options)?;
        let response = self.client.get(&capture_url).send().await?;
        let bytes = response.bytes().await?;
        Ok(bytes.to_vec())
    }

    // Structured options fetch methods
    pub async fn fetch_screenshot(
        &self,
        url: &str,
        options: Option<&ScreenshotOptions>,
    ) -> Result<Vec<u8>> {
        let capture_url = self.build_screenshot_url(url, options)?;
        let response = self.client.get(&capture_url).send().await?;
        let bytes = response.bytes().await?;
        Ok(bytes.to_vec())
    }

    pub async fn fetch_pdf_structured(
        &self,
        url: &str,
        options: Option<&PdfOptions>,
    ) -> Result<Vec<u8>> {
        let capture_url = self.build_pdf_url_structured(url, options)?;
        let response = self.client.get(&capture_url).send().await?;
        let bytes = response.bytes().await?;
        Ok(bytes.to_vec())
    }

    pub async fn fetch_content_structured(
        &self,
        url: &str,
        options: Option<&ContentOptions>,
    ) -> Result<ContentResponse> {
        let capture_url = self.build_content_url_structured(url, options)?;
        let response = self.client.get(&capture_url).send().await?;
        let content = response.json::<ContentResponse>().await?;
        Ok(content)
    }

    pub async fn fetch_metadata_structured(
        &self,
        url: &str,
        options: Option<&MetadataOptions>,
    ) -> Result<MetadataResponse> {
        let capture_url = self.build_metadata_url_structured(url, options)?;
        let response = self.client.get(&capture_url).send().await?;
        let metadata = response.json::<MetadataResponse>().await?;
        Ok(metadata)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_capture_new() {
        let capture = Capture::new("test_key".to_string(), "test_secret".to_string());
        assert_eq!(capture.key, "test_key");
        assert_eq!(capture.secret, "test_secret");
        assert!(!capture.options.use_edge);
    }

    #[test]
    fn test_capture_with_edge() {
        let options = CaptureOptions::new().with_edge();
        let capture =
            Capture::with_options("test_key".to_string(), "test_secret".to_string(), options);
        assert!(capture.options.use_edge);
    }

    #[test]
    fn test_build_image_url() {
        let capture = Capture::new("test_key".to_string(), "test_secret".to_string());
        let url = capture
            .build_image_url("https://example.com", None)
            .unwrap();
        assert!(url.contains("test_key"));
        assert!(url.contains("image"));
        assert!(url.contains("https://cdn.capture.page"));
    }

    #[test]
    fn test_build_image_url_with_edge() {
        let options = CaptureOptions::new().with_edge();
        let capture =
            Capture::with_options("test_key".to_string(), "test_secret".to_string(), options);
        let url = capture
            .build_image_url("https://example.com", None)
            .unwrap();
        assert!(url.contains("https://edge.capture.page"));
    }

    #[test]
    fn test_missing_credentials() {
        let capture = Capture::new("".to_string(), "".to_string());
        let result = capture.build_image_url("https://example.com", None);
        assert!(matches!(result, Err(CaptureError::MissingCredentials)));
    }

    #[test]
    fn test_missing_url() {
        let capture = Capture::new("test_key".to_string(), "test_secret".to_string());
        let result = capture.build_image_url("", None);
        assert!(matches!(result, Err(CaptureError::MissingUrl)));
    }
}
