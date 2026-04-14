use futures::future::join_all;
use indicatif::{ProgressBar, ProgressStyle};
use scraper::{Html, Selector};
use serde::Serialize;
use std::collections::{HashSet, VecDeque};
use std::fs::File;
use std::io::Write;
use std::sync::{Arc, Mutex};
use std::time::Duration;
use tokio::sync::Semaphore;
use url::Url;

#[derive(Serialize, Clone)]
struct PageData {
    url: String,
    title: String,
    description: String,
    links: Vec<String>,
}

async fn fetch(url: &str) -> Option<PageData> {
    let client = reqwest::Client::builder()
        .timeout(Duration::from_secs(10))
        .build()
        .ok()?;

    let response = client
        .get(url)
        .header("User-Agent", "Mozilla/5.0")
        .send()
        .await
        .ok()?;

    let body = response.text().await.ok()?;
    let document = Html::parse_document(&body);

    // title
    let title_selector = Selector::parse("title").unwrap();
    let title = document
        .select(&title_selector)
        .next()
        .map(|t| t.inner_html().trim().to_string())
        .unwrap_or("No title".to_string());

    // description
    let meta_selector = Selector::parse("meta[name=\"description\"]").unwrap();
    let description = document
        .select(&meta_selector)
        .next()
        .and_then(|m| m.value().attr("content"))
        .unwrap_or("")
        .to_string();

    // links
    let link_selector = Selector::parse("a").unwrap();
    let links: Vec<String> = document
        .select(&link_selector)
        .filter_map(|l| l.value().attr("href"))
        .take(20)
        .map(|s| s.to_string())
        .collect();

    Some(PageData {
        url: url.to_string(),
        title,
        description,
        links,
    })
}

#[tokio::main]
async fn main() {
    let seeds = vec![
        "https://example.com",
        "https://www.rust-lang.org",
    ];

    let max_pages: usize = 50;
    let concurrency = 5;

    let visited = Arc::new(Mutex::new(HashSet::<String>::new()));
    let queue = Arc::new(Mutex::new(VecDeque::<String>::new()));

    for seed in seeds {
        queue.lock().unwrap().push_back(seed.to_string());
    }

    let pb = ProgressBar::new(max_pages as u64);
    pb.set_style(
        ProgressStyle::default_bar()
            .template("[{elapsed_precise}] {bar:40.cyan/blue} {pos}/{len}")
            .unwrap(),
    );

    let semaphore = Arc::new(Semaphore::new(concurrency));
    let mut results = vec![];

    while results.len() < max_pages {
        let mut batch = vec![];

        {
            let mut q = queue.lock().unwrap();
            while batch.len() < concurrency && !q.is_empty() {
                batch.push(q.pop_front().unwrap());
            }
        }

        if batch.is_empty() {
            break;
        }

        let tasks = batch.into_iter().map(|url| {
            let visited = visited.clone();
            let queue = queue.clone();
            let semaphore = semaphore.clone();
            let pb = pb.clone();

            async move {
                let _permit = semaphore.acquire().await.unwrap();

                {
                    let mut v = visited.lock().unwrap();
                    if v.contains(&url) {
                        return None;
                    }
                    v.insert(url.clone());
                }

                let page = fetch(&url).await;

                if let Some(ref p) = page {
                    if let Ok(base) = Url::parse(&p.url) {
                        for link in &p.links {
                            if let Ok(next) = base.join(link) {
                                queue
                                    .lock()
                                    .unwrap()
                                    .push_back(next.to_string());
                            }
                        }
                    }
                }

                pb.inc(1);
                page
            }
        });

        let batch_results = join_all(tasks).await;

        for r in batch_results.into_iter().flatten() {
            if results.len() < max_pages {
                results.push(r);
            }
        }
    }

    pb.finish_with_message("Crawling complete");

    let json = serde_json::to_string_pretty(&results).unwrap();
    let mut file = File::create("results.json").unwrap();
    file.write_all(json.as_bytes()).unwrap();

    println!("Saved {} pages", results.len());
}