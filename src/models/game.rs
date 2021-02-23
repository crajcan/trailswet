use serde::Serialize;
use sqlx::PgPool;

#[derive(Serialize)]
pub struct Game {
    pub id: u32,
    pub home_team_id: u32,
    pub away_team_id: u32,
}
