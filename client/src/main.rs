use hyper::Client;
use hyper::body::HttpBody as _;
use tokio::io::{stdout, AsyncWriteExt as _};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    // This is where we will setup our HTTP client requests.
    // Still inside `async fn main`...
    let client = Client::new();

    // Parse an `http::Uri`...
    let uri = "http://localhost:3000/echo/uppercase".parse()?;

    // Await the response...
    let mut resp = client.get(uri).await?;

    println!("Response: {}", resp.status());

    while let Some(chunk) = resp.body_mut().data().await {
        stdout().write_all(&chunk?).await?;
    }
    
    Ok(())
}