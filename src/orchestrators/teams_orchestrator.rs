use askama::Template;
use serde::Serialize;
use sqlx::{Error};

#[derive(Serialize, Template)]
#[template(path = "teams/index.html")]
pub struct TeamsOrchestrator;

impl TeamsOrchestrator {
    pub async fn find() -> Result<TeamsOrchestrator, Error> {
        Ok(TeamsOrchestrator)
    }
}
