use askama::Template;
use serde::Serialize;
use sqlx::{Error, PgPool};

#[derive(Serialize, Template)]
#[template(path = "teams/index.html")]
pub struct TeamsOrchestrator;

impl TeamsOrchestrator {
    pub async fn find(_pool: &PgPool) -> Result<TeamsOrchestrator, Error> {
        Ok(TeamsOrchestrator)
    }
}
