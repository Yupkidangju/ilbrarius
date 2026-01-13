use reqwest::Url;
use anyhow::Result;
use chromiumoxide::browser::{Browser, BrowserConfig};
use chromiumoxide::page::Page as ChromePage;
use futures::StreamExt;
use std::collections::VecDeque;
use crate::store::{save_page, is_visited, Page};
use std::time::{SystemTime, UNIX_EPOCH};

/// URL을 정규화하고 최종 리다이렉션 목적지를 반환합니다. [v0.1.0]
pub async fn normalize_url(input_url: &str) -> Result<String> {
    let client = reqwest::Client::builder()
        .redirect(reqwest::redirect::Policy::limited(10))
        .build()?;

    let response = client.get(input_url).send().await?;
    let final_url = response.url().to_string();

    log::info!("[v0.1.0] URL Normalized: {} -> {}", input_url, final_url);
    Ok(final_url)
}

/// Recursive BFS Crawler 구현. [v0.1.2]
/// 사용자가 설정한 Depth까지 하위 링크를 탐색하며 데이터를 수집합니다.
pub async fn start_bfs_crawl(start_url: String, max_depth: u32) -> Result<()> {
    let (mut browser, mut handler) = Browser::launch(
        BrowserConfig::builder()
            .no_sandbox()
            .window_size(1280, 800)
            .build()?
    ).await?;

    // 브라우저 이벤트 핸들러를 별도 태스크로 실행
    tokio::spawn(async move {
        while let Some(h) = handler.next().await {
            if let Err(e) = h {
                log::error!("Browser handler error: {}", e);
                break;
            }
        }
    });

    let mut queue = VecDeque::new();
    queue.push_back((start_url, 0));

    while let Some((current_url, depth)) = queue.pop_front() {
        if depth > max_depth { continue; }
        if is_visited(&current_url).await.unwrap_or(false) { continue; }

        log::info!("[v0.1.2] Crawling Depth {}: {}", depth, current_url);

        let page = browser.new_page(&current_url).await?;
        
        // 페이지 로딩 대기 및 데이터 추출
        let title = page.get_title().await?.unwrap_or_default();
        let content = page.get_content().await?;
        
        // 데이터 즉시 저장 (Checkpoint System)
        let page_data = Page {
            url: current_url.clone(),
            title,
            content,
            depth,
            parent_url: None, // 향후 구조 개선 시 추가
            timestamp: SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs(),
        };
        
        save_page(page_data).await?;

        // 하위 링크 추출 (Depth가 남아있는 경우)
        if depth < max_depth {
            let links: Vec<String> = page.evaluate("() => Array.from(document.querySelectorAll('a')).map(a => a.href)")
                .await?
                .into_value()?;

            for link in links {
                if link.starts_with("http") && !is_visited(&link).await.unwrap_or(false) {
                    queue.push_back((link, depth + 1));
                }
            }
        }

        page.close().await?;
    }

    browser.close().await?;
    Ok(())
}

pub async fn init() {
    println!("Crawler module initialized");
}
