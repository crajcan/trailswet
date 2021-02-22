CREATE TABLE IF NOT EXISTS teams (
    id   SERIAL PRIMARY KEY,
    name VARCHAR(255)
);

CREATE TABLE IF NOT EXISTS games (
    id           SERIAL PRIMARY KEY,
    home_team_id INTEGER,
    away_team_id INTEGER,
    FOREIGN KEY(home_team_id) REFERENCES teams(id),
    FOREIGN KEY(away_team_id) REFERENCES teams(id)
);
