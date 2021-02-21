use actix_web::*;
use serde::Serialize;
use crate::Presenter;

#[derive(Serialize)]
struct GameOrchestrator {
    home_team_name: String,
    away_team_name: String,
}

#[get("/show{tail:.*}")]
async fn show() -> impl Responder {
    let orchestrator = GameOrchestrator {
        home_team_name: "Miami Hurricanes".into(),
        away_team_name: "Nebraska Cornhuskers".into(),
    };

    Presenter {
        resource: orchestrator,
        template: "games/show.html",
    }
}
