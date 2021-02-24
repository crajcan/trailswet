use serde::Serialize;
use sqlx::{Error, PgPool};

#[derive(Serialize)]
pub struct Game {
    pub id: i32,
    pub home_team_id: i32,
    pub away_team_id: i32,
}

impl Game {
    pub async fn find(id: i32, pool: &PgPool) -> Result<Self, Error> {
        sqlx::query_as!(
            Game,
            r#"
        SELECT *
        FROM games
        WHERE id = $1
                "#,
            id
        )
        .fetch_one(&*pool)
        .await
    }
}
