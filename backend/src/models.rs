use crate::schema::{experiences, projects, profiles};
use diesel::{Insertable, Queryable};
use rocket::serde::{Deserialize, Serialize};
use time::Date;

#[derive(Queryable, Serialize, Debug)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Project {
    pub id: i32,
    pub title: String,
    pub description: String,
    pub link: String,
}

#[derive(Insertable, Deserialize)]
#[diesel(table_name = projects)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct NewProject {
    pub title: String,
    pub description: String,
    pub link: String,
}

#[derive(Queryable, Serialize)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Experience {
    pub id: i32,
    pub title: String,
    pub company: String,
    pub description: String,
    pub start_date: Date,
    pub end_date: Option<Date>,
    pub org_link: String,
}

#[derive(Insertable, Deserialize)]
#[diesel(table_name = experiences)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct NewExperience {
    pub title: String,
    pub company: String,
    pub description: String,
    pub start_date: Date,
    pub end_date: Option<Date>,
    pub org_link: String,
}

#[derive(Queryable, Serialize)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Profile {
    pub id: i32,
    pub first_name: String,
    pub last_name: String,
    pub biography: Option<String>
}

#[derive(Insertable, Deserialize)]
#[diesel(table_name = profiles)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct NewProfile {
    pub first_name: String,
    pub last_name: String,
    pub biography: Option<String>
}

#[derive(Deserialize)]
pub struct Login {
    pub username: String,
    pub password: String,
}

#[derive(Serialize)]

pub struct LoginOutcome {
    pub success: bool,
}
