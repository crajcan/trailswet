use crate::game_orchestrator::GameOrchestrator;
use crate::Presenter;
use actix_web::*;
use serde::Deserialize;
use sqlx::PgPool;

#[derive(Deserialize)]
struct PathParams {
    id: i32,
}

#[get(r#"/show/{id:\d+}{tail:.*}"#)]
async fn show(path_params: web::Path<PathParams>, db_pool: web::Data<PgPool>) -> Result<impl Responder> {
    let orchestrator = GameOrchestrator::find(path_params.id, db_pool.get_ref()).await;

    match orchestrator {
        Ok(data) => {
                      Ok(Presenter {
                          resource: data,
                      })
        },
        Err(_) => Err(error::ErrorNotFound(path_params.id))
    }
}
