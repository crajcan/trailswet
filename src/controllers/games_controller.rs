use crate::game_orchestrator::GameOrchestrator;
use crate::Presenter;
use actix_web::*;
use sqlx::PgPool;

#[get("/show{tail:.*}")]
async fn show(db_pool: web::Data<PgPool>) -> impl Responder {
    let orchestrator = GameOrchestrator::find(1, db_pool.get_ref()).await;

    Presenter {
        resource: orchestrator,
        template: "games/show.html",
    }
}
