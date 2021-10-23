use crate::scraper;
use crate::models::Project;

pub async fn get_projects(language: &str) {
  let res = scraper::get_trends(&String::from(language)).await;
  return match res {
      Ok(projects) => {
        print_results(projects);
      }
      Err(err) => {
          eprint!("{}", err);
      }
  };
}

fn print_results(projects: Vec<Project>) {
  for proj in projects {
    println!("{} - {} stars", proj.link, proj.stars);
    println!("{}", proj.description);
    println!("-------------------------------");
  }
}