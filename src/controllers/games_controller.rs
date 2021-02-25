use crate::game_orchestrator::GameOrchestrator;
use crate::Presenter;
use actix_web::*;
use sqlx::PgPool;
use serde::Deserialize;

#[derive(Deserialize)]
struct PathParams {
    id: i32
}

#[get(r#"/show/{id:\d+}{tail:.*}"#)]
async fn show(path_params: web::Path<PathParams>, db_pool: web::Data<PgPool>) -> impl Responder {
    let orchestrator = GameOrchestrator::find(path_params.id, db_pool.get_ref()).await;

    Presenter {
        resource: orchestrator,
        template: "games/show.html",
    }
}
