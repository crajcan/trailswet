use serde::Serialize;
use sqlx::{Error};
use crate::DB_POOL;

#[derive(Serialize)]
pub struct Team {
    pub id: i32,
    pub name: String,
}

impl Team {
    pub async fn find(id: i32) -> Result<Self, Error> {
        sqlx::query_as!(
            Team,
            r#"
        SELECT *
        FROM teams
        WHERE id = $1 
                "#,
            id
        )
        .fetch_one(&*DB_POOL)
        .await
    }
}
