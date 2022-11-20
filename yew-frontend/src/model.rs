use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct Project {
    pub id: i32,
    pub title: String,
    pub description: String,
    pub link: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct Experience {
    pub id: i32,
    pub title: String,
    pub company: String,
    pub description: String,
    pub year: String,
    pub org_link: String,
}

#[derive(Serialize)]
pub struct Login {
    pub username: String,
    pub password: String,
}

#[derive(Deserialize)]

pub struct LoginOutcome {
    pub success: bool,
}
