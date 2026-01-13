use surrealdb::engine::local::RocksDb;
use surrealdb::Surreal;
use serde::{Serialize, Deserialize};
use std::sync::Arc;
use tokio::sync::OnceCell;

static DB: OnceCell<Surreal<RocksDb>> = OnceCell::const_new();

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Page {
    pub url: String,
    pub title: String,
    pub content: String,
    pub depth: u32,
    pub parent_url: Option<String>,
    pub timestamp: u64,
}

/// SurrealDB를 임베디드 모드로 초기화합니다. [v0.1.2]
pub async fn get_db() -> anyhow::Result<&'static Surreal<RocksDb>> {
    DB.get_or_try_init(|| async {
        let db = Surreal::new::<RocksDb>("librarius.db").await?;
        db.use_ns("librarius").use_db("archive").await?;
        Ok(db)
    }).await
}

/// 수집된 페이지를 DB에 즉시 저장합니다. (Checkpoint System) [v0.1.2]
pub async fn save_page(page: Page) -> anyhow::Result<()> {
    let db = get_db().await?;
    let _: Option<Page> = db.create(("pages", &page.url)).content(page).await?;
    Ok(())
}

/// 이미 탐색된 URL인지 확인합니다. [v0.1.2]
pub async fn is_visited(url: &str) -> anyhow::Result<bool> {
    let db = get_db().await?;
    let page: Option<Page> = db.select(("pages", url)).await?;
    Ok(page.is_some())
}

/// 모든 수집된 페이지를 가져옵니다. [v0.1.3]
pub async fn get_all_pages() -> anyhow::Result<Vec<Page>> {
    let db = get_db().await?;
    let mut pages: Vec<Page> = db.select("pages").await?;
    // Depth와 Timestamp 순으로 정렬
    pages.sort_by(|a, b| a.depth.cmp(&b.depth).then(a.timestamp.cmp(&b.timestamp)));
    Ok(pages)
}

pub async fn init() {
    if let Err(e) = get_db().await {
        eprintln!("Failed to initialize SurrealDB: {}", e);
    } else {
        println!("SurrealDB (RocksDB) initialized successfully");
    }
}
