use crate::admin::*;
use crate::models::*;
use crate::schema::*;
use crate::add_component::*;

use auth::authorization::*;
use rocket::request::Form;
use rocket::response::{Redirect};  
use diesel::prelude::*;
use rocket_contrib::templates::Template;


#[get("/project/<post_id>")]
pub fn edit_project(conn: DbConn, post_id: i32, _user: AuthCont<AdministratorCookie>) -> Template
{
    let mut projects = projects::table
                   .filter(projects::id.eq(post_id))
                   .limit(1)
                   .load::<Project>(&*conn)
                   .unwrap();
    let project = projects.pop().unwrap();
    Template::render("project_edit", &project)
}

#[post("/project/<post_id>", data = "<project>" )]
pub fn update_project(conn: DbConn, post_id: i32,  project: Form<ProjectForm>, _user: AuthCont<AdministratorCookie>) -> Result<Redirect, diesel::result::Error>
{
    diesel::update(projects::table.find(post_id))
    .set(projects::title.eq(project.title.clone()))
    .execute(&*conn)?;
    
    diesel::update(projects::table.find(post_id))
    .set(projects::description.eq(project.description.clone()))
    .execute(&*conn)?;

    diesel::update(projects::table.find(post_id))
    .set(projects::link.eq(project.link.clone()))
    .execute(&*conn)?;

    diesel::update(projects::table.find(post_id))
    .set(projects::img_path.eq(project.img_path.clone()))
    .execute(&*conn)?;

    Ok(Redirect::to("/admin"))
}

#[post("/make_primary/<post_id>")]
pub fn make_primary(conn: DbConn, post_id: i32, _user: AuthCont<AdministratorCookie>) -> Result<Redirect, diesel::result::Error>
{
    diesel::update(projects::table.filter(projects::is_primary.eq(true)))
    .set(projects::is_primary.eq(false))
    .execute(&*conn)?;

    diesel::update(projects::table.find(post_id))
    .set(projects::is_primary.eq(true))
    .execute(&*conn)?;

    Ok(Redirect::to("/admin"))
}


#[get("/skill/<post_id>")]
pub fn edit_skill(conn: DbConn, post_id: i32, _user: AuthCont<AdministratorCookie>) -> Template
{
    let mut projects = skills::table
                   .filter(skills::id.eq(post_id))
                   .limit(1)
                   .load::<Skill>(&*conn)
                   .unwrap();
    let project = projects.pop().unwrap();
    Template::render("skill_edit", &project)
}

#[post("/skill/<post_id>", data = "<skill>" )]
pub fn update_skill(conn: DbConn, post_id: i32,  skill: Form<NewSkill>, _user: AuthCont<AdministratorCookie>) -> Result<Redirect, diesel::result::Error>
{

    diesel::update(skills::table.find(post_id))
    .set(skills::name.eq(skill.name.clone()))
    .execute(&*conn)?;

    diesel::update(skills::table.find(post_id))
    .set(skills::origin.eq(skill.origin.clone()))
    .execute(&*conn)?;

    diesel::update(skills::table.find(post_id))
    .set(skills::yoxp.eq(skill.yoxp.clone()))
    .execute(&*conn)?;

    Ok(Redirect::to("/admin"))
}

#[get("/education/<post_id>")]
pub fn edit_education(conn: DbConn, post_id: i32, _user: AuthCont<AdministratorCookie>) -> Template
{
    let mut projects = education::table
                   .filter(education::id.eq(post_id))
                   .limit(1)
                   .load::<Education>(&*conn)
                   .unwrap();
    let project = projects.pop().unwrap();
    Template::render("education_edit", &project)
}

#[post("/education/<post_id>", data = "<education>" )]
pub fn update_education(conn: DbConn, post_id: i32,  education: Form<NewEducation>, _user: AuthCont<AdministratorCookie>) -> Result<Redirect, diesel::result::Error>
{
    diesel::update(education::table.find(post_id))
    .set(education::major.eq(education.major.clone()))
    .execute(&*conn)?;

    diesel::update(education::table.find(post_id))
    .set(education::institution.eq(education.institution.clone()))
    .execute(&*conn)?;

    diesel::update(education::table.find(post_id))
    .set(education::year.eq(education.year.clone()))
    .execute(&*conn)?;

    Ok(Redirect::to("/admin"))
}


#[get("/experience/<post_id>")]
pub fn edit_experience(conn: DbConn, post_id: i32, _user: AuthCont<AdministratorCookie>) -> Template
{
    let mut projects = experience::table
                   .filter(experience::id.eq(post_id))
                   .limit(1)
                   .load::<Experience>(&*conn)
                   .unwrap();
    let project = projects.pop().unwrap();
    Template::render("experience_edit", &project)
}

#[post("/experience/<post_id>", data = "<experience>" )]
pub fn update_experience(conn: DbConn, post_id: i32,  experience: Form<NewExperience>, _user: AuthCont<AdministratorCookie>) -> Result<Redirect, diesel::result::Error>
{

    diesel::update(experience::table.find(post_id))
    .set(experience::title.eq(experience.title.clone()))
    .execute(&*conn)?;

    diesel::update(experience::table.find(post_id))
    .set(experience::company.eq(experience.company.clone()))
    .execute(&*conn)?;

    diesel::update(experience::table.find(post_id))
    .set(experience::year.eq(experience.year.clone()))
    .execute(&*conn)?;

    diesel::update(experience::table.find(post_id))
    .set(experience::description.eq(experience.description.clone()))
    .execute(&*conn)?;

    diesel::update(experience::table.find(post_id))
    .set(experience::org_link.eq(experience.org_link.clone()))
    .execute(&*conn)?;

    
    Ok(Redirect::to("/admin"))
}
