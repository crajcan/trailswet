use crate::game_orchestrator::GameOrchestrator;
use crate::Presenter;
use actix_web::*;
use actix_web::web::{Path, Data};
use serde::Deserialize;
use sqlx::{PgPool};
use derive_more::{Display, Error};
use std::convert::From;


#[derive(Deserialize)]
struct PathParams {
    id: i32,
}

#[derive(Debug, Display, Error)]
#[display(fmt = "my error: {}", name)]
struct NetflixError {
    name: String,
}

impl error::ResponseError for NetflixError {}

impl From<sqlx::Error> for NetflixError {
    fn from(e: sqlx::Error) -> Self {
        let name = e.to_string();

        NetflixError { name: name }
    }
}

#[get(r#"/show/{id:\d+}{tail:.*}"#)]
async fn show(path_params: Path<PathParams>, db_pool: Data<PgPool>) -> Result<impl Responder, NetflixError> {
    let orchestrator = GameOrchestrator::find(path_params.id, db_pool.get_ref()).await;

    match orchestrator {
        Ok(data) => {
                      Ok(Presenter {
                          resource: data,
                      })
        },
        Err(e) => {
            Err(NetflixError::from(e))
        }
    }
}

