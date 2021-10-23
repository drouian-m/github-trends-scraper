mod handlers;
mod models;
mod scraper;
mod cli;

use actix_web::{App, HttpServer};
use structopt::StructOpt;

/// Search for a pattern in a file and display the lines that contain it.
#[derive(StructOpt)]
struct Cli {
    /// The path to the file to read
    #[structopt(short, long)]
    runtime: Option<String>,

    #[structopt(short, long, required_if("runtime", "cli"))]
    language: Option<String>,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let args = Cli::from_args();
    let runtime = args.runtime.unwrap();

    match runtime.as_str() {
        "web" => {
            HttpServer::new(|| {
                App::new()
                    .service(handlers::root)
                    .service(handlers::trends)
                    .service(handlers::lang_trends)
            })
            .bind("127.0.0.1:8080")?
            .run()
            .await
        }
        "cli" => {
            let language = args.language.unwrap();
            cli::get_projects(&language).await;
            Ok(())
        }
        "bot" => {
            println!("WIP...");
            Ok(())
        }
        _ => {
            println!("Invalid {:?} option (available : web, cli)", runtime);
            Ok(())
        }
    }
}
