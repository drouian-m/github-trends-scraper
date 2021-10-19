use serde::{Serialize};

#[derive(Debug, Serialize)]
pub struct Project {
    pub title: String,
    pub description: String,
    pub link: String,
    pub stars: String,
}
