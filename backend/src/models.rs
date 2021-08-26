use diesel::{Queryable, Insertable};
use crate::schema::{projects,
     skills,education, experience, languages, profile};

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

#[derive(Queryable, Serialize)]
pub struct Language
{
    pub id: i32,
    pub language: String,
    pub proficiency: String
}

#[derive(Insertable, Deserialize, FromForm)]
#[table_name="languages"]
pub struct NewLanguage
{
    pub language: String,
    pub proficiency: String
}

#[derive(Queryable, Serialize)]
pub struct Profile
{
    pub id: i32,
    pub first_name: String,
    pub last_name: String,
    pub title: String, 
    pub profile_picture_path: String,
    pub location: String,
    pub email: String,
    pub about_me: String,
    pub github_link: String,
    pub linkedin_link: String
}

#[derive(Insertable, Deserialize, FromForm)]
#[table_name="profile"]
pub struct NewProfile
{
    pub first_name: String,
    pub last_name: String,
    pub title: String, 
    pub profile_picture_path: String,
    pub location: String,
    pub email: String,
    pub about_me: String,
    pub github_link: String,
    pub linkedin_link: String
}



#[derive(Serialize)]
pub struct MainTemplate
{
    pub projects: Vec<Project>,
    pub skills: Vec<Skill>,
    pub experience: Vec<Experience>,
    pub languages: Vec<Language>,
    pub education: Vec<Education>,
    pub profile: Profile,
    pub image_paths: Vec<String>
}

#[derive(Serialize)]
pub struct EditProfileTemplate
{
    pub profile: Profile,
    pub image_paths: Vec<String>
}