use actix_web::{get, web, App, HttpServer, HttpResponse, Responder};
use regex::Regex;
use select::document::Document;
use select::predicate::{Class, Name, Predicate};
use serde::{Serialize};
use serde_json::Value;

#[derive(Debug, Serialize)]
struct Project {
    title: String,
    description: String,
    link: String,
    stars: String,
}

#[derive(Debug, Serialize)]
struct Results {
    projects: Vec<Project>
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(root)
            .service(trends)
            .service(lang_trends)
    })
        .bind("127.0.0.1:8080")?
        .run()
        .await
}

#[get("/")]
async fn root() -> impl Responder {
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
async fn trends() -> impl Responder {
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
async fn lang_trends(lang: web::Path<String>) -> impl Responder {
    let result = call_github(&lang).await;
    return match result {
        Ok(html_content) => {
            let projects = trending_scrapper(html_content);
            HttpResponse::Ok().json(projects)
        },
        Err(err) => {
            eprint!("{}", err);
            HttpResponse::InternalServerError()
                .body("Error trying to read trending")
        }
    }
}

/// Get trending infos from github webpage
async fn call_github(language: &String) -> Result<String, reqwest::Error> {
    let url = format!("https://github.com/trending/{lang}", lang = language);
    let body = reqwest::get(&url).await?.text().await?;
    Ok(body)
}

/// Extract trending projects from html content
fn trending_scrapper(html_content: String) -> Vec<Project>{
    let document = Document::from_read(html_content.as_bytes()).unwrap();
    let re = Regex::new(r"(\n|\s)+").unwrap();

    let mut results: Vec<Project> = Vec::new();
    for node in document.find(Class("Box-row")) {
        let title = node.find(Class("h3")).next().unwrap().text();
        let stars = node.find(Class("mt-2").descendant(Name("a"))).next().unwrap().text();
        let link: String = node.find(Class("h3").descendant(Name("a"))).next().unwrap().attr("href").unwrap().parse().unwrap();

        let mut description: String = String::from("");
        let description_node = node.find(Class("col-9")).next();
        if !description_node.is_none() {
            description = description_node.unwrap().text();
        }

        let project = Project{
            title: re.replace_all(&title, "").parse().unwrap(),
            description: description.trim().parse().unwrap(),
            link: format!("https://github.com{link}", link = link),
            stars: re.replace_all(&stars, "").parse().unwrap(),
        };

        results.push(project);
    }

    return results;
}

#[cfg(test)]
mod tests {
    use crate::trending_scrapper;

    #[test]
    fn scrape_empty_page() {
        let res = trending_scrapper(String::new());
        assert_eq!(res.len(), 0);
    }

    #[test]
    fn scrape_projects() {
        let page_content = String::from("<html><body>
        <article class=\"Box-row\"> \
          <h1 class=\"h3 lh-condensed\"> \
            <a href=\"/rust-lang/rust\"> \
              <span data-view-component=\"true\" class=\"text-normal\"> \
                rust-lang / \
              </span> \
              rust \
            </a> \
          </h1> \
          <p class=\"col-9 color-text-secondary my-1 pr-4\"> \
            Empowering everyone to build reliable and efficient software. \
          </p> \
          <div class=\"f6 color-text-secondary mt-2\"> \
           <a href=\"/rust-lang/rust/stargazers\" data-view-component=\"true\" class=\"Link--muted d-inline-block mr-3\">
             59,521
           </a>
        </article> \
        </body></html>");
        let res = trending_scrapper(page_content);
        assert_eq!(res.len(), 1);
        let project = res.get(0).unwrap();
        assert_eq!(project.title, "rust-lang/rust");
        assert_eq!(project.description, "Empowering everyone to build reliable and efficient software.");
        assert_eq!(project.stars, "59,521");
        assert_eq!(project.link, "https://github.com/rust-lang/rust");
    }

    #[test]
    fn empty_description() {
        let page_content = String::from("<html><body>
        <article class=\"Box-row\"> \
          <h1 class=\"h3 lh-condensed\"> \
            <a href=\"/rust-lang/rust\"> \
              <span data-view-component=\"true\" class=\"text-normal\"> \
                rust-lang / \
              </span> \
              rust \
            </a> \
          </h1> \
          <p class=\"col-9 color-text-secondary my-1 pr-4\"> \
          </p> \
          <div class=\"f6 color-text-secondary mt-2\"> \
           <a href=\"/rust-lang/rust/stargazers\" data-view-component=\"true\" class=\"Link--muted d-inline-block mr-3\">
             59,521
           </a>
        </article> \
        </body></html>");
        let res = trending_scrapper(page_content);
        assert_eq!(res.len(), 1);
        let project = res.get(0).unwrap();
        assert_eq!(project.title, "rust-lang/rust");
        assert_eq!(project.description, "");
    }
}

