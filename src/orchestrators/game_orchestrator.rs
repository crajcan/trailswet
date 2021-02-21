use serde::Serialize;

#[derive(Serialize)]
struct Game {
    id:           u32,
    home_team_id: u32,
    away_team_id: u32,
}

#[derive(Serialize)]
struct Team {
    id: u32,
    name: String,
    url: String,
}

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
                id:           game_id,
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
        }
    }
}
