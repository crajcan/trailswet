use crate::game::Game;
use crate::team::Team;
use serde::Serialize;
use sqlx::{Error, PgPool};

#[derive(Serialize)]
pub struct GameOrchestrator {
    game: Game,
    home_team: Team,
    away_team: Team,
}

impl GameOrchestrator {
    pub async fn find(game_id: u32, pool: &PgPool) -> GameOrchestrator {
        let home_team = Team::find(1, pool).await.unwrap();
        let away_team = Team::find(2, pool).await.unwrap();

        GameOrchestrator {
            game: Game {
                id: game_id,
                home_team_id: 1,
                away_team_id: 2,
            },
            home_team,
            away_team,
        }
    }
}
