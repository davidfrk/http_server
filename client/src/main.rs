#![deny(warnings)]
#![warn(rust_2018_idioms)]
use std::env;

use hyper::{body::HttpBody as _, Client};
use tokio::io::{self, AsyncWriteExt as _};
use hyper::{Body, Method, Request, Uri};

// A simple type alias so as to DRY.
type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

#[tokio::main]
async fn main() -> Result<()> {
    // Some simple CLI args requirements...
    let url = match env::args().nth(1) {
        Some(url) => url,
        None => {
            println!("Usage: client <url>");
            return Ok(());
        }
    };

    //Post request on local host
    println!("Post request on local host.");

    let req = Request::builder()
        .method(Method::POST)
        .uri("http://localhost:3000/echo")
        .header("content-type", "application/json")
        .body(Body::from(r#"{"library":"hyper"}"#))?;

    let client = Client::new();

    // POST it...
    let resp = client.request(req).await?;
    
    println!("Response: {}.\n\n", resp.status());
    dbg!(hyper::body::to_bytes(resp.into_body()).await?);


    //Multiple Requests Segment
    println!("Multiple Requests");

    let client = Client::new();

    let ip_fut = async {
        let resp = client.get(Uri::from_static("http://httpbin.org/ip")).await?;
        hyper::body::to_bytes(resp.into_body()).await
    };
    let headers_fut = async {
        let resp = client.get(Uri::from_static("http://httpbin.org/headers")).await?;
        hyper::body::to_bytes(resp.into_body()).await
    };
    
    // Wait on both them at the same time:
    let (ip, headers) = futures::try_join!(ip_fut, headers_fut)?;

    dbg!("ip: {}, header: {}", ip, headers);
    println!("\n\n");

    println!("Get by argument.");
    // HTTPS requires picking a TLS implementation, so give a better
    // warning if the user tries to request an 'https' URL.
    let url = url.parse::<hyper::Uri>().unwrap();
    if url.scheme_str() != Some("http") {
        println!("This example only works with 'http' URLs.");
        return Ok(());
    }

    fetch_url(url).await
}

async fn fetch_url(url: hyper::Uri) -> Result<()> {
    let client = Client::new();

    let mut res = client.get(url).await?;

    println!("Response: {}", res.status());
    println!("Headers: {:#?}\n", res.headers());

    // Stream the body, writing each chunk to stdout as we get it
    // (instead of buffering and printing at the end).
    while let Some(next) = res.data().await {
        let chunk = next?;
        io::stdout().write_all(&chunk).await?;
    }

    println!("\n\nDone!");

    Ok(())
}