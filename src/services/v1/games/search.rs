use askama::Template;
use serde::Serialize;
use sqlx::Error;

#[derive(Serialize, Template)]
#[template(path = "trails/index.html")]
pub struct Search;

pub async fn search() -> Result<Search, Error> {
    Ok(Search)
}
