use diesel::{Queryable, Insertable};
use crate::schema::{experiences, projects};
use rocket::serde::{Serialize, Deserialize};

#[derive(Queryable, Serialize, Debug)]
pub struct Project {
    pub id: i32,
    pub title: String,
    pub description: String,
    pub link: String
}

#[derive(Insertable, Deserialize)]
#[table_name="projects"]
pub struct NewProject {
    pub title: String,
    pub description: String,
    pub link: String
}

#[derive(Queryable, Serialize)]
pub struct Experience {
    pub id: i32,
    pub title: String,
    pub description: String,
    pub company: String,
    pub year: String,
    pub org_link: String
}

#[derive(Insertable, Deserialize)]
#[table_name="experiences"]
pub struct NewExperience {
    pub title: String,
    pub description: String,
    pub company: String,
    pub year: String,
    pub org_link: String
}