use crate::scraper;
use actix_web::{get, web, HttpResponse, Responder};
use serde_json::Value;

#[get("/")]
pub async fn root() -> impl Responder {
    let data = r#"
    [{
        "url": "http://localhost:8080/trends",
        "title": "Github root trends"
    }, {
        "url": "http://localhost:8080/trends",
        "title": "Projects language trends results"
    }]"#;
    let v: Value = serde_json::from_str(data).unwrap();
    HttpResponse::Ok().json(v)
}

#[get("/trends")]
pub async fn trends() -> impl Responder {
    let data = r#"
    {
        "languages": {
            "values": ["rust", "javascript", "go", "python", "typescript", "..."],
            "example": "http://localhost:8080/trends/rust"
        }
    }"#;
    let v: Value = serde_json::from_str(data).unwrap();
    HttpResponse::Ok().json(v)
}

#[get("/trends/{lang}")]
pub async fn lang_trends(lang: web::Path<String>) -> impl Responder {
    let result = scraper::call_github(&lang).await;
    return match result {
        Ok(html_content) => {
            let projects = scraper::trending_scrapper(html_content);
            HttpResponse::Ok().json(projects)
        }
        Err(err) => {
            eprint!("{}", err);
            HttpResponse::InternalServerError().body("Error trying to read trending")
        }
    };
}
