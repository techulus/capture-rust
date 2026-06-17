use capture_rust::{Capture, CreateSessionOptions};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let api_key =
        std::env::var("CAPTURE_KEY").expect("CAPTURE_KEY environment variable is required");
    let api_secret =
        std::env::var("CAPTURE_SECRET").expect("CAPTURE_SECRET environment variable is required");

    let capture = Capture::new(api_key, api_secret);

    let created = capture
        .create_session(Some(&CreateSessionOptions {
            cdp: Some(true),
            max_ttl_seconds: Some(300),
            ..Default::default()
        }))
        .await?;

    let session_id = created["session"]["id"]
        .as_str()
        .expect("created session id");
    let connect_url = created["session"]["connectUrl"]
        .as_str()
        .expect("CDP connectUrl");

    println!("Session ID: {session_id}");
    println!("CDP connect URL: {connect_url}");

    capture.close_session(session_id).await?;

    Ok(())
}
