use futures::{stream, StreamExt};
use snafu::prelude::*;
use std::time::{Duration, Instant};

const REQUEST_COUNT: usize = 10;
const CONCURRENT_REQUESTS: usize = 1_000;

#[tokio::main]
async fn main() -> Result<()> {
    let client = reqwest::Client::builder()
        .build()
        .unwrap();

    let urls = vec![String::from("http://example.com"); REQUEST_COUNT];

    // Concurrent Requests
    let bodies = stream::iter(urls)
        .map(|url| {
            let req = client.get(&url);
            tokio::spawn(async move {
                let resp = req.send().await.unwrap();
                let text = resp.text().await.unwrap();
                Result::Ok((url, text))
            })
        })
        .buffer_unordered(CONCURRENT_REQUESTS);

    bodies
        .for_each(|b| async {
            match b {
                Ok(Ok((url, b))) => {}
                Err(e) => {}
                Ok(Err(e)) => {}
            }
        })
        .await;

    Ok(())
}

#[derive(Debug, Snafu)]
struct Error;

type Result<T, E = Error> = ::core::result::Result<T, E>;