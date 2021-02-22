use serde::Serialize;

#[derive(Serialize)]
pub struct Team {
    pub id: u32,
    pub name: String,
    //pub url: String,
}

impl Team {
    pub fn find(id: i32) -> Self {
        Team {
            id: 1,
            name: "Miami Hurricanes".into(),
//          url: "http://localhost:3000/teams/1".into(),
        }
    }
}
