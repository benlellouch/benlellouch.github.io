#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate serde_derive;
#[macro_use]
extern crate diesel;
extern crate dotenv;
#[macro_use]
extern crate dotenv_codegen;
extern crate bcrypt;
extern crate rocket_multipart_form_data;
extern crate image;

pub mod schema;
pub mod models;
pub mod add_component;
pub mod remove_component;
pub mod edit_component;
pub mod upload_content;


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
use upload_content::*;

use diesel::prelude::*;
use diesel::dsl::max;



#[get("/")]
fn index(conn: DbConn) -> Template
{
    let mut projects = projects::table.load::<Project>(&*conn)
    .unwrap();
    projects.sort_by(|a, b| b.is_primary.cmp(&a.is_primary));
    let skills = skills::table.load::<Skill>(&*conn).unwrap();
    let education = education::table.load::<Education>(&*conn).unwrap();
    let experience = experience::table.load::<Experience>(&*conn).unwrap();
    let about_me_id: Option<i32> = about_me::table.select(max(about_me::id)).first(&*conn).unwrap();
    let about_me_maybe = about_me::table
    .filter(about_me::id.eq(about_me_id.unwrap()))
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
    .mount("/admin", routes![admin, make_primary])
    .mount("/admin/add", routes![add_project, add_skill, add_experience, add_education, add_about_me])
    .mount("/admin/delete", routes![delete_project, delete_education, delete_experience, delete_skill])
    .mount("/admin/edit", routes![edit_project, edit_skill, edit_education, edit_experience])
    .mount("/admin/update", routes![update_education, update_project, update_skill, update_experience])
    .mount("/admin/upload", routes![upload])
    .attach(Template::fairing())
    .attach(DbConn::fairing())
    .launch();
}
