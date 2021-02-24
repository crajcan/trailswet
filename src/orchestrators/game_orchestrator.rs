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
    pub async fn find(game_id: i32, pool: &PgPool) -> GameOrchestrator {
        let game = Game::find(game_id, pool).await.unwrap();
        let home_team = Team::find(game.home_team_id, pool).await.unwrap();
        let away_team = Team::find(game.away_team_id, pool).await.unwrap();

        GameOrchestrator {
            game,
            home_team,
            away_team,
        }
    }
}