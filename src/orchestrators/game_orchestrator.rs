use crate::game::Game;
use crate::team::Team;
use serde::Serialize;

#[derive(Serialize)]
pub struct GameOrchestrator {
    game: Game,
    home_team: Team,
    away_team: Team,
}

impl GameOrchestrator {
    pub fn find(game_id: u32) -> GameOrchestrator {
        GameOrchestrator {
            game: Game {
                id: game_id,
                home_team_id: 1,
                away_team_id: 2,
            },
            home_team: Team::find(1),
            away_team: Team::find(2)
        }
    }
}
