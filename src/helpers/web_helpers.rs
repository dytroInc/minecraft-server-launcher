use std::str::from_utf8;
use follow_redirects::ClientExt;
use hyper::body::HttpBody;
use tokio::fs::{File};
use tokio::io::{AsyncWriteExt, BufWriter};
use hyper::Client;
use hyper_tls::HttpsConnector;

// Took reference from https://github.com/dolphin2410/server-script/blob/master/src/web.rs#L56-L74
pub async fn download_file_from_url(dir: String, url: String) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let mut buffer = BufWriter::new(File::create(&dir).await.expect("An error has occurred!"));

    let https = HttpsConnector::new();
    let client = Client::builder()
        .build::<_, hyper::Body>(https);

    let uri = url.parse()?;

    let mut response = client.follow_redirects().get(uri).await?;

    while let Some(chunk) = response.body_mut().data().await {
        buffer.write_all(&chunk?).await?;
    }

    buffer.flush().await?;

    Ok(())
}

pub async fn get_request(url: &str) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
    let https = HttpsConnector::new();
    let client = Client::builder()
        .build::<_, hyper::Body>(https);

    let uri = url.parse()?;

    let response = client.follow_redirects().get(uri).await?;

    let bytes = hyper::body::to_bytes(response).await?;

    Ok(from_utf8(&bytes)?.to_owned())
}