#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate serde_derive;
#[macro_use]
extern crate diesel;
extern crate dotenv;
#[macro_use]
extern crate dotenv_codegen;

pub mod schema;
pub mod models;

extern crate rocket_auth_login as auth;
use auth::authorization::*;

use rocket_contrib::templates::Template;
use rocket_contrib::databases::{database, diesel::PgConnection};
use rocket_contrib::databases::diesel::Connection;


use rocket::response::{NamedFile, Redirect, Flash};  
use rocket::http::{Cookie, Cookies};
use rocket::request::Form;


use std::collections::HashMap;
use std::path::{PathBuf, Path};

mod admin;
use admin::*;

mod lib;

use models::*;
use schema::*;

use diesel::prelude::*;

#[database("postgres")]
pub struct DbConn(PgConnection);

#[get("/")]
fn index(conn: DbConn) -> Template
{
    let mut projects = projects::table.load::<Project>(&*conn)
    .unwrap();
    projects.sort_by(|a, b| b.is_primary.cmp(&a.is_primary));
    let template = ProjectTemplate {projects : projects};
    Template::render("index", &template)
}

#[get("/")]
fn admin(conn: DbConn, _user: AuthCont<AdministratorCookie> ) -> Template
{
    let projects = projects::table.load::<Project>(&*conn).unwrap();
    let template = ProjectTemplate {projects : projects};
    Template::render("admin", &template)
}

#[post("/add", data = "<project>")]
fn post_project(conn: DbConn, project: Form<ProjectForm> ) -> Result<Redirect, diesel::result::Error>
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

#[post("/delete/<post_id>")]
fn delete_project(conn: DbConn, post_id: i32, _user: AuthCont<AdministratorCookie>) -> Result<Redirect, diesel::result::Error>
{
    diesel::delete(projects::table.filter(projects::id.eq(post_id))).execute(&*conn)?;

    Ok(Redirect::to("/admin"))
}

#[get("/edit/<post_id>")]
fn edit_project(conn: DbConn, post_id: i32, _user: AuthCont<AdministratorCookie>) -> Template
{
    let mut projects = projects::table
                   .filter(projects::id.eq(post_id))
                   .limit(1)
                   .load::<Project>(&*conn)
                   .unwrap();
    let project = projects.pop().unwrap();
    Template::render("project_edit", &project)
}

#[post("/update/<post_id>", data = "<project>" )]
fn update_project(conn: DbConn, post_id: i32,  project: Form<ProjectForm>, _user: AuthCont<AdministratorCookie>) -> Result<Redirect, diesel::result::Error>
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
fn make_primary(conn: DbConn, post_id: i32, _user: AuthCont<AdministratorCookie>) -> Result<Redirect, diesel::result::Error>
{
    diesel::update(projects::table.filter(projects::is_primary.eq(true)))
    .set(projects::is_primary.eq(false))
    .execute(&*conn)?;

    diesel::update(projects::table.find(post_id))
    .set(projects::is_primary.eq(true))
    .execute(&*conn)?;

    Ok(Redirect::to("/admin"))
}

#[get("/assets/<path..>")]
fn get_resource(path: PathBuf) -> Option<NamedFile>
{
    NamedFile::open(Path::new("assets/").join(path)).ok()
}

#[post("/login", data = "<form>")]
fn process_login(form: Form<LoginCont<AdministratorForm>>, mut cookies: Cookies) -> Result<Redirect, Flash<Redirect>> {
    let inner = form.into_inner();
    let login = inner.form;
    login.flash_redirect("/login", "/login", &mut cookies)
}

#[get("/login", rank = 1)]
fn logged_in(_user: AuthCont<AdministratorCookie>) -> Redirect {
    Redirect::to("/admin")
}
#[get("/login", rank = 2)]
fn login() -> Option<NamedFile> {
    NamedFile::open(Path::new("assets/static/login.html")).ok()
}

fn main() {

    dotenv::dotenv().ok();

    rocket::ignite()
    .mount("/", routes![index, get_resource, logged_in, login, process_login])
    .mount("/admin", routes![admin, post_project, delete_project, edit_project, update_project, make_primary])
    .attach(Template::fairing())
    .attach(DbConn::fairing())
    .launch();
}
