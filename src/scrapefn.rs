use reqwest;
use select::document::Document;

//get + return raw html for parsing
pub async fn get_html(
    url: &str
) -> Result<Document,reqwest::Error>{
    let res = reqwest::get(url)
        .await?
        .text()
        .await?;

    Ok(
        Document::from(res.as_str())
    )
}