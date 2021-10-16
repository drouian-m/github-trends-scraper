use std::error::Error;
use regex::Regex;
use select::document::Document;
use select::predicate::{Class, Name, Predicate};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let html_content = call_github("rust").await?;
    trending_scrapper(html_content);
    Ok(())
}

/// Get trending informations from github webpage
async fn call_github(language: &str) -> Result<String, Box<dyn Error>> {
    let url = format!("https://github.com/trending/{lang}", lang = language);
    let body = reqwest::get(&url).await?.text().await?;
    Ok(body)
}

/// Extract trending projects from html content
fn trending_scrapper(html_content: String) {
    let document = Document::from_read(html_content.as_bytes()).unwrap();
    let re = Regex::new(r"(\n|\s)+").unwrap();
    for node in document.find(Class("Box-row")) {
        let title = node.find(Class("h3")).next().unwrap().text();
        println!("title : {:?}", re.replace_all(&title, ""));

        let description_node = node.find(Class("col-9")).next();
        if !description_node.is_none() {
            let description = description_node.unwrap().text();
            println!("description : {:?}", description.trim());
        }

        let stars = node.find(Class("mt-2").descendant(Name("a"))).next().unwrap().text();
        println!("stars : {:?}", re.replace_all(&stars, ""));

        let link = node.find(Class("h3").descendant(Name("a"))).next().unwrap().attr("href").unwrap();
        println!("link : {:?}", link);
    }
}