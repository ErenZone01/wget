use anyhow::{Context, Result};
use futures_util::{stream, StreamExt};
use reqwest::Client;
use std::io::Write;
use std::sync::{Arc, Mutex};

use crate::untils::file::create_directory;

pub async fn download_files_from_list(urls: Vec<String>) -> Result<()> {
    let total_sizes = Arc::new(Mutex::new(Vec::new()));
    let finished_files = Arc::new(Mutex::new(Vec::new()));

    let futures = stream::iter(urls.clone().into_iter().map(|url| {
        let total_sizes = Arc::clone(&total_sizes);
        let finished_files = Arc::clone(&finished_files);
        {
            async move {
                let result = download_file_async(url.clone(), url.clone().split("/").last().unwrap().to_owned()).await;
                match result {
                    Ok(size) => {
                        total_sizes.lock().unwrap().push(size);
                        finished_files.lock().unwrap().push(url);
                    }
                    Err(e) => {
                        eprintln!("Error: {:?}", e);
                    }
                }
            }
        }
    }))
    .buffer_unordered(10);

    futures.for_each(|_| async {}).await;

    let total_sizes = total_sizes.lock().unwrap();
    let finished_files = finished_files.lock().unwrap();

    println!("content size: {:?}", *total_sizes);
    for url in urls {
        if let Some(filename) = url.split('/').last() {
            println!("finished {}", filename);
        }
    }
    println!("\nDownload finished:  {:?}", *finished_files);

    Ok(())
}

pub async fn download_file_async(url: String, file: String) -> Result<u64> {
    let client = Client::builder()
        .user_agent("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/91.0.4472.124 Safari/537.36")
        .build()?;

    let response = client
        .get(&url)
        .send()
        .await
        .context("Failed to send request")?;

    if !response.status().is_success() {
        return Err(anyhow::anyhow!(
            "Server returned non-OK status: {}",
            response.status()
        ));
    }

    let total_size = response.content_length().unwrap_or(0);

    let mut stream = response.bytes_stream();

    let files = create_directory(file);
    match files {
        Ok(mut f) => {
            while let Some(chunk) = stream.next().await {
                let chunk = chunk.context("Failed to download chunk")?;
                f.0.write_all(&chunk)
                    .context("Failed to write chunk to file")?;
            }
        }
        Err(_) => {}
    }

    Ok(total_size)
}
