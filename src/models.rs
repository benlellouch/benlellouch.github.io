use diesel::{Queryable, Insertable};
use crate::schema::{projects,
     skills, about_me, education, experience};

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

#[derive(Queryable, Serialize)]
pub struct Skill
{
    pub id: i32,
    pub name: String,
    pub origin: String,
    pub yoxp: i32
}

#[derive(Insertable, Deserialize, FromForm)]
#[table_name="skills"]
pub struct NewSkill
{
    pub name: String,
    pub origin: String,
    pub yoxp: i32
}


#[derive(Queryable, Serialize)]
pub struct AboutMe
{
    pub id: i32,
    pub description: String
}

#[derive(Insertable, Deserialize, FromForm)]
#[table_name="about_me"]
pub struct NewAboutMe
{
    pub description: String
}

#[derive(Queryable, Serialize)]
pub struct Experience
{
    pub id: i32,
    pub title: String,
    pub company: String,
    pub year: String,
    pub description: String,
    pub org_link: String
}

#[derive(Insertable, Deserialize, FromForm)]
#[table_name="experience"]
pub struct NewExperience
{
    pub title: String,
    pub company: String,
    pub year: String,
    pub description: String,
    pub org_link: String
}

#[derive(Queryable, Serialize)]
pub struct Education
{
    pub id: i32,
    pub major: String,
    pub institution: String,
    pub year: String,
}

#[derive(Insertable, Deserialize, FromForm)]
#[table_name="education"]
pub struct NewEducation
{
    pub major: String,
    pub institution: String,
    pub year: String,
}

#[derive(Serialize)]
pub struct MainTemplate
{
    pub projects: Vec<Project>,
    pub skills: Vec<Skill>,
    pub experience: Vec<Experience>,
    pub about_me: AboutMe,
    pub education: Vec<Education>
}