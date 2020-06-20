use diesel::{Queryable, Insertable};
use crate::schema::projects;

#[derive(Queryable, Serialize)]
pub struct Project
{
    pub id: i32,
    pub title: String,
    pub description: String,
    pub link: String,
    pub img_path: String,
    pub is_primary: bool
}

#[derive(Insertable, Deserialize)]
#[table_name="projects"]
pub struct NewProject
{
    pub title: String,
    pub description: String,
    pub link: String,
    pub img_path: String
}

#[derive(FromForm)]
pub struct ProjectForm
{
    pub title: String,
    pub description: String,
    pub link: String,
    pub img_path: String
}

#[derive(Serialize)]
pub struct ProjectTemplate
{
    pub projects: Vec<Project>
}