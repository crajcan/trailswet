use crate::Presenter;
use actix_web::*;
use serde::Serialize;

#[derive(Serialize)]
struct Game {
    home_team_id: u16,
    away_team_id: u16,
}

#[derive(Serialize)]
struct Team {
    id: u16,
    name: String,
    url: String,
}

#[derive(Serialize)]
struct GameOrchestrator {
    game: Game,
    home_team: Team,
    away_team: Team,
}

#[get("/show{tail:.*}")]
async fn show() -> impl Responder {
    let orchestrator = GameOrchestrator {
        game: Game {
            home_team_id: 1,
            away_team_id: 2,
        },
        home_team: Team {
            id: 1,
            name: "Miami Hurricanes".into(),
            url: "http://localhost:3000/teams/1".into(),
        },
        away_team: Team {
            id: 2,
            name: "Nebraska Cornhuskers".into(),
            url: "http://localhost:3000/teams/2".into(),
        },
    };

    Presenter {
        resource: orchestrator,
        template: "games/show.html",
    }
}
