use chrono::NaiveDateTime;
use serde::Serialize;
use crate::DB_POOL;

#[derive(Serialize)]
pub struct Game {
    pub id: i32,
    pub home_team_id: i32,
    pub away_team_id: i32,
    pub date: NaiveDateTime,
    pub home_team_score: Option<i16>,
    pub away_team_score: Option<i16>,
}

impl Game {
    pub async fn find(id: i32) -> Result<Self, sqlx::Error> {
        sqlx::query_as!(
            Game,
            r#"
        SELECT *
        FROM games
        WHERE id = $1
                "#,
            id
        )
        .fetch_one(&*DB_POOL)
        .await
    }

    pub async fn find_by_date_range(
        team_a_id: i32,
        team_b_id: i32,
        min_date: NaiveDateTime,
        max_date: NaiveDateTime,
    ) -> Result<Vec<Self>, sqlx::Error> {
        sqlx::query_as!(
            Game,
            r#"
        SELECT *
        FROM games
        WHERE home_team_id IN ($1, $2)
        AND   away_team_id IN ($1, $2)
        AND   date >= $3
        AND   date <= $4 
            "#,
            team_a_id,
            team_b_id,
            min_date,
            max_date 
        )
        .fetch_all(&*DB_POOL)
        .await
    }
}
