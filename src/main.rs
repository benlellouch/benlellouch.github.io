#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate serde_derive;
#[macro_use]
extern crate diesel;
extern crate dotenv;
#[macro_use]
extern crate dotenv_codegen;
extern crate bcrypt;

pub mod schema;
pub mod models;
pub mod add_component;
pub mod remove_component;
pub mod edit_component;


extern crate rocket_auth_login as auth;
use auth::authorization::*;

use rocket_contrib::templates::Template;

use rocket::response::{NamedFile, Redirect, Flash};  
use rocket::http::{Cookies};
use rocket::request::Form;


use std::path::{PathBuf, Path};

mod admin;
use admin::*;

use models::*;
use schema::*;

use add_component::*;
use remove_component::*;
use edit_component::*;

use diesel::prelude::*;



#[get("/")]
fn index(conn: DbConn) -> Template
{
    let mut projects = projects::table.load::<Project>(&*conn)
    .unwrap();
    projects.sort_by(|a, b| b.is_primary.cmp(&a.is_primary));
    let skills = skills::table.load::<Skill>(&*conn).unwrap();
    let education = education::table.load::<Education>(&*conn).unwrap();
    let experience = experience::table.load::<Experience>(&*conn).unwrap();
    let about_me_maybe = about_me::table
    .limit(1)
    .load::<AboutMe>(&*conn).unwrap()
    .pop();

    let about_me = match about_me_maybe
    {
        Some(abt_me) => abt_me,
        None => {
            AboutMe{id: 0, description: "".to_string()}
        }
    };

    let template = MainTemplate 
    {
        projects : projects,
        skills: skills,
        education: education,
        experience: experience,
        about_me: about_me
    };
    Template::render("index", &template)
}

#[get("/")]
fn admin(conn: DbConn, _user: AuthCont<AdministratorCookie> ) -> Template
{
    let projects = projects::table.load::<Project>(&*conn).unwrap();
    let skills = skills::table.load::<Skill>(&*conn).unwrap();
    let education = education::table.load::<Education>(&*conn).unwrap();
    let experience = experience::table.load::<Experience>(&*conn).unwrap();
    let about_me_maybe = about_me::table
    .limit(1)
    .load::<AboutMe>(&*conn).unwrap()
    .pop();

    let about_me = match about_me_maybe
    {
        Some(abt_me) => abt_me,
        None => {
            AboutMe{id: 0, description: "".to_string()}
        }
    };
    let template = MainTemplate 
    {
        projects : projects,
        skills: skills,
        education: education,
        experience: experience,
        about_me: about_me
    };
    Template::render("admin", &template)
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
    .mount("/admin", routes![admin, add_project, add_skill, add_experience, add_education, add_about_me, delete_project, delete_education, delete_experience, delete_skill,  edit_project, update_project, make_primary, edit_skill, update_skill])
    .attach(Template::fairing())
    .attach(DbConn::fairing())
    .launch();
}
