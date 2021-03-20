use askama::Template;
use serde::Serialize;
use sqlx::{Error, PgPool};

#[derive(Serialize, Template)]
#[template(path = "games/index.html")]
pub struct GamesOrchestrator;

impl GamesOrchestrator {
    pub async fn find(_pool: &PgPool) -> Result<GamesOrchestrator, Error> {
        Ok(GamesOrchestrator)
    }
}
