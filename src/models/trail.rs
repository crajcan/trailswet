use crate::DB_POOL;
use serde::Serialize;
use sqlx::Error;

#[derive(Serialize)]
pub struct Trail {
    pub id: i32,
    pub name: String,
}

impl Trail {
    pub async fn find(id: i32) -> Result<Self, Error> {
        sqlx::query_as!(
            Trail,
            r#"
        SELECT *
        FROM trails
        WHERE id = $1 
                "#,
            id
        )
        .fetch_one(&*DB_POOL)
        .await
    }
}
