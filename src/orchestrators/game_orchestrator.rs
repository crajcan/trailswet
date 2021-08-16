use crate::game::Game;
use crate::team::Team;
use askama::Template;
use serde::Serialize;
use sqlx::Error;

#[derive(Serialize, Template)]
#[template(path = "games/show.html")]
pub struct GameOrchestrator {
    game: Game,
    home_team: Team,
    away_team: Team,
}

impl GameOrchestrator {
    pub async fn find(game_id: i32) -> Result<GameOrchestrator, Error> {
        let game = Game::find(game_id).await?;
        let home_team = Team::find(game.home_team_id).await.unwrap();
        let away_team = Team::find(game.away_team_id).await.unwrap();

        Ok(GameOrchestrator {
            game,
            home_team,
            away_team,
        })
    }
}
