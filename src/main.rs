use futures::{stream, StreamExt};

fn main() {
    let client = reqwest::Client::builder().build().unwrap();

    // Concurrent Requests
    let bodies = stream::iter([])
        .map(|url: String| {
            let req = client.get(&url);
            tokio::spawn(async move {
                let resp = req.send().await.unwrap();
                let text = resp.text().await.unwrap();
                Result::Ok((url, text))
            })
        })
        .buffer_unordered(0);

    bodies.for_each(|b| async {
        match b {
            Ok(Ok((url, b))) => {}
            Err(e) => {}
            Ok(Err(e)) => {}
        }
    });
}
