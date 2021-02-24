use serde::Serialize;
use sqlx::{Error, PgPool};

#[derive(Serialize)]
pub struct Team {
    pub id: i32,
    pub name: String,
    //pub url: String,
}

impl Team {
    pub async fn find(id: i32, pool: &PgPool) -> Result<Self, Error> {
        sqlx::query_as!(
            Team,
            r#"
        SELECT *
        FROM teams
        WHERE id = $1 
                "#,
            id
        )
        .fetch_one(&*pool)
        .await
    }
}
