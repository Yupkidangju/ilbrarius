use reqwest::Url;
use anyhow::Result;
use chromiumoxide::browser::{Browser, BrowserConfig};
use chromiumoxide::page::Page as ChromePage;
use futures::StreamExt;
use std::collections::VecDeque;
use crate::store::{save_page, is_visited, Page};
use std::time::{SystemTime, UNIX_EPOCH};
use tauri::{AppHandle, Manager, Emitter};
use base64::{Engine as _, engine::general_purpose};
use serde::Serialize;

#[derive(Clone, Serialize)]
pub struct CrawlEvent {
    pub url: String,
    pub title: String,
    pub screenshot: Option<String>, // Base64 encoded
    pub status: String,
    pub depth: u32,
}

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

/// Recursive BFS Crawler 구현. [v0.1.3]
/// 실시간 화면 캡처 및 상태 이벤트를 프론트엔드로 전송합니다.
pub async fn start_bfs_crawl(app: AppHandle, start_url: String, max_depth: u32) -> Result<()> {
    let (mut browser, mut handler) = Browser::launch(
        BrowserConfig::builder()
            .no_sandbox()
            .window_size(1280, 800)
            .build()?
    ).await?;

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

        // 상태 알림: 탐색 시작
        let _ = app.emit("crawl-status", CrawlEvent {
            url: current_url.clone(),
            title: "".into(),
            screenshot: None,
            status: "Loading...".into(),
            depth,
        });

        let page = browser.new_page(&current_url).await?;
        
        // 페이지 로딩 대기
        tokio::time::sleep(tokio::time::Duration::from_millis(1000)).await;

        let title = page.get_title().await?.unwrap_or_default();
        let content = page.get_content().await?;
        
        // 화면 캡처 (Live Viewport용)
        let screenshot_bytes = page.screenshot(chromiumoxide::page::ScreenshotParams::builder().build()).await?;
        let screenshot_base64 = general_purpose::STANDARD.encode(screenshot_bytes);

        // 상태 알림: 캡처 완료 및 데이터 수집
        let _ = app.emit("crawl-status", CrawlEvent {
            url: current_url.clone(),
            title: title.clone(),
            screenshot: Some(screenshot_base64),
            status: "Captured".into(),
            depth,
        });

        // 데이터 즉시 저장
        let page_data = Page {
            url: current_url.clone(),
            title,
            content,
            depth,
            parent_url: None,
            timestamp: SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs(),
        };
        
        save_page(page_data).await?;

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
    let _ = app.emit("crawl-finished", ());
    Ok(())
}

pub async fn init() {
    println!("Crawler module initialized");
}
