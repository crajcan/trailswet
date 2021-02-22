use serde::Serialize;

#[derive(Serialize)]
pub struct Team {
    pub id: u32,
    pub name: String,
    pub url: String,
}
