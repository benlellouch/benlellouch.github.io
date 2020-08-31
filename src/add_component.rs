use crate::admin::*;
use crate::models::*;
use crate::schema::*;
use auth::authorization::*;
use rocket::request::Form;
use rocket_contrib::databases::{database, diesel::PgConnection};
use rocket::response::{Redirect};  
use diesel::prelude::*;




#[database("postgres")]
pub struct DbConn(PgConnection);


#[post("/project", data = "<project>")]
pub fn add_project(conn: DbConn, project: Form<ProjectForm> , _user: AuthCont<AdministratorCookie> ) -> Result<Redirect, diesel::result::Error>
{
    let new_project = NewProject
    {
        title: project.title.clone(),
        description: project.description.clone(),
        link: project.link.clone(),
        img_path: project.img_path.clone()
    };

    let result = diesel::insert_into(projects::table)
    .values(&new_project)
    .execute(&*conn);

    match result
    {
        Ok(_) => Ok(Redirect::to("/")),
        Err(p) => Err(p)
    }
}

#[post("/skill", data = "<skill>")]
pub fn add_skill(conn: DbConn, skill: Form<NewSkill>, _user: AuthCont<AdministratorCookie> ) -> Result<Redirect, diesel::result::Error>
{
    let new_skill = 
    NewSkill
    {
        name: skill.name.clone(),
        origin: skill.origin.clone(),
        yoxp: skill.yoxp.clone()
    };
    let result = diesel::insert_into(skills::table)
    .values(&new_skill)
    .execute(&*conn);

    match result
    {
        Ok(_) => Ok(Redirect::to("/")),
        Err(p) => Err(p)
    }
}


#[post("/experience", data = "<experience>")]
pub fn add_experience(conn: DbConn, experience: Form<NewExperience>, _user: AuthCont<AdministratorCookie> ) -> Result<Redirect, diesel::result::Error>
{
    let new_experience = 
    NewExperience
    {
        title: experience.title.clone(),
        company: experience.company.clone(),
        year: experience.year.clone(),
        description: experience.description.clone(),
        org_link: experience.org_link.clone()
    };
    let result = diesel::insert_into(experience::table)
    .values(&new_experience)
    .execute(&*conn);

    match result
    {
        Ok(_) => Ok(Redirect::to("/")),
        Err(p) => Err(p)
    }
}

#[post("/education", data = "<education>")]
pub fn add_education(conn: DbConn, education: Form<NewEducation> , _user: AuthCont<AdministratorCookie> ) -> Result<Redirect, diesel::result::Error>
{
    let new_education = 
    NewEducation
    {
        major: education.major.clone(),
        institution: education.institution.clone(),
        year: education.year.clone()
    };
    let result = diesel::insert_into(education::table)
    .values(&new_education)
    .execute(&*conn);

    match result
    {
        Ok(_) => Ok(Redirect::to("/")),
        Err(p) => Err(p)
    }
}
