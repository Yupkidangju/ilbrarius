use reqwest::Url;
use anyhow::Result;

/// URL을 정규화하고 최종 리다이렉션 목적지를 반환합니다. [v0.1.0]
/// iptime 등의 주소에서 발생하는 301/302 리다이렉션을 추적합니다.
pub async fn normalize_url(input_url: &str) -> Result<String> {
    let client = reqwest::Client::builder()
        .redirect(reqwest::redirect::Policy::limited(10)) // 최대 10회 리다이렉션 허용
        .build()?;

    let response = client.get(input_url).send().await?;
    let final_url = response.url().to_string();

    log::info!("[v0.1.0] URL Normalized: {} -> {}", input_url, final_url);
    Ok(final_url)
}

pub async fn init() {
    println!("Crawler module initialized");
}
