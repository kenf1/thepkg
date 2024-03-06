use reqwest;
use select::document::Document;

//get + return raw html for parsing
pub async fn get_html(url: &str) -> Result<Document,reqwest::Error>{
    let res = reqwest::get(url)
        .await?
        .text()
        .await?;

    let raw_html = Document::from(res.as_str());

    Ok(raw_html)
}