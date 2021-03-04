use crate::game::Game;
use crate::team::Team;
use askama::Template;
use serde::Serialize;
use sqlx::{Error, PgPool};

#[derive(Serialize, Template)]
#[template(path = "games/show.html")]
pub struct GameOrchestrator {
    game: Game,
    home_team: Team,
    away_team: Team,
}

impl GameOrchestrator {
    pub async fn find(game_id: i32, pool: &PgPool) -> Result<GameOrchestrator, Error> {
        let game = Game::find(game_id, pool).await?;
        let home_team = Team::find(game.home_team_id, pool).await.unwrap();
        let away_team = Team::find(game.away_team_id, pool).await.unwrap();

        Ok(GameOrchestrator {
            game,
            home_team,
            away_team,
        })
    }
}
