use crate::admin::*;
use crate::schema::*;
use auth::authorization::*;
use rocket::response::{Redirect};  
use diesel::prelude::*;
use crate::add_component::*;

#[post("/project/<post_id>")]
pub fn delete_project(conn: DbConn, post_id: i32, _user: AuthCont<AdministratorCookie>) -> Result<Redirect, diesel::result::Error>
{
    diesel::delete(projects::table.filter(projects::id.eq(post_id))).execute(&*conn)?;

    Ok(Redirect::to("/admin"))
}

#[post("/skill/<post_id>")]
pub fn delete_skill(conn: DbConn, post_id: i32, _user: AuthCont<AdministratorCookie>) -> Result<Redirect, diesel::result::Error>
{
    diesel::delete(skills::table.filter(skills::id.eq(post_id))).execute(&*conn)?;

    Ok(Redirect::to("/admin"))
}

#[post("/experience/<post_id>")]
pub fn delete_experience(conn: DbConn, post_id: i32, _user: AuthCont<AdministratorCookie>) -> Result<Redirect, diesel::result::Error>
{
    diesel::delete(experience::table.filter(experience::id.eq(post_id))).execute(&*conn)?;

    Ok(Redirect::to("/admin"))
}

#[post("/education/<post_id>")]
pub fn delete_education(conn: DbConn, post_id: i32, _user: AuthCont<AdministratorCookie>) -> Result<Redirect, diesel::result::Error>
{
    diesel::delete(education::table.filter(education::id.eq(post_id))).execute(&*conn)?;

    Ok(Redirect::to("/admin"))
}