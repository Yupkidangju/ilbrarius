use chromiumoxide::browser::{Browser, BrowserConfig};
use chromiumoxide::cdp::browser_protocol::page::PrintToPdfParams;
use futures::StreamExt;
use std::path::PathBuf;
use tokio::fs;
use crate::store::{get_all_pages, Page};

/// 수집된 데이터를 통합하여 PDF로 내보냅니다. [v0.1.3]
pub async fn assemble_pdf(output_path: PathBuf) -> anyhow::Result<()> {
    let pages = get_all_pages().await?;
    if pages.is_empty() {
        return Err(anyhow::anyhow!("수집된 데이터가 없습니다."));
    }

    // 1. 통합 HTML 생성 (목차 포함)
    let combined_html = generate_combined_html(&pages);

    // 2. 브라우저 실행 및 PDF 출력
    let (mut browser, mut handler) = Browser::launch(
        BrowserConfig::builder()
            .no_sandbox()
            .build()?
    ).await?;

    tokio::spawn(async move {
        while let Some(h) = handler.next().await {
            if let Err(e) = h {
                log::error!("PDF Exporter browser error: {}", e);
                break;
            }
        }
    });

    let page = browser.new_page("about:blank").await?;
    page.set_content(combined_html).await?;
    
    // 약간의 렌더링 대기
    tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;

    let pdf_data = page.pdf(PrintToPdfParams::default()).await?;
    fs::write(&output_path, pdf_data).await?;

    page.close().await?;
    browser.close().await?;

    Ok(())
}

fn generate_combined_html(pages: &[Page]) -> String {
    let mut toc_html = String::from("<div id='toc'><h1>Table of Contents</h1><ul>");
    let mut content_html = String::from("<div id='content'>");

    for (idx, page) in pages.iter().enumerate() {
        let anchor = format!("page-{}", idx);
        toc_html.push_str(&format!(
            "<li><a href='#{}'>Depth {}: {}</a></li>",
            anchor, page.depth, page.title
        ));

        content_html.push_str(&format!(
            "<section id='{}' style='page-break-after: always;'>
                <h1>{}</h1>
                <p><small>Source: {}</small></p>
                <hr/>
                <div>{}</div>
            </section>",
            anchor, page.title, page.url, page.content
        ));
    }

    toc_html.push_str("</ul></div>");
    content_html.push_str("</div>");

    format!(
        "<!DOCTYPE html>
        <html>
        <head>
            <meta charset='UTF-8'>
            <style>
                body {{ font-family: sans-serif; line-height: 1.6; padding: 40px; }}
                h1 {{ color: #2c3e50; }}
                #toc ul {{ list-style-type: none; padding-left: 0; }}
                #toc li {{ margin-bottom: 10px; }}
                #toc a {{ text-decoration: none; color: #3498db; }}
                section {{ margin-top: 50px; }}
                img {{ max-width: 100%; height: auto; }}
                pre {{ background: #f4f4f4; padding: 10px; border-radius: 5px; overflow-x: auto; }}
            </style>
        </head>
        <body>
            {}
            <div style='page-break-after: always;'></div>
            {}
        </body>
        </html>",
        toc_html, content_html
    )
}
