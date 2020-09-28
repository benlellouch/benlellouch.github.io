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
extern crate s3;

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

use s3::bucket::Bucket;
use s3::S3Error;
use s3::creds::Credentials;
use s3::region::Region;


#[get("/")]
fn index(conn: DbConn) -> Template
{
    Template::render("index", &generate_main_template(conn))
}

#[get("/")]
fn admin(conn: DbConn, _user: AuthCont<AdministratorCookie> ) -> Template
{
    Template::render("admin", &generate_main_template(conn))
}

fn generate_main_template(conn: DbConn) -> MainTemplate
{
    let mut projects = projects::table.load::<Project>(&*conn)
    .unwrap();
    projects.sort_by(|a, b| b.is_primary.cmp(&a.is_primary));
    let skills = skills::table.load::<Skill>(&*conn).unwrap();
    let education = education::table.load::<Education>(&*conn).unwrap();
    let experience = experience::table.load::<Experience>(&*conn).unwrap();
    let languages = languages::table.load::<Language>(&*conn).unwrap();
    let profile = profile::table
    .filter(profile::id.eq(1))
    .limit(1)
    .load::<Profile>(&*conn).unwrap()
    .pop().unwrap_or_else( || 
    Profile
    {
        id: 1,
        first_name: "John".to_string(),
        last_name: "AppleSeed".to_string(),
        profile_path: "assets/images/profile/default.jpg".to_string(),
        location: "Los Angeles, CA".to_string(),
        title: "Software Engineer".to_string(),
        email: "john@appleseed.com".to_string(),
        about_me: "Lorem Ipsum".to_string(),
        github_link: "https://github.com".to_string(),
        linkedin_link: "https://linkedin.com".to_string()
    }
    );

    MainTemplate
    {
        projects: projects,
        skills: skills,
        education: education,
        experience: experience,
        languages: languages,
        profile: profile,
    }
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

fn save_file(key: String, bucket: &Bucket)
{
    let (data,code) = bucket.get_object_blocking(&key).unwrap();
    match code
    {
        200 => {
            std::fs::write(key, data).unwrap();
        }

        _ => ()
    };

}

fn retrieve_dynamic_assets()
{
    let access_key = "AKIATLBBVGWPFTHNA36Z";
    let secret_key = "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";
    let bucket_name = "portfolio-lellouch";
    let region: Region = "eu-west-2".parse().unwrap();
    let credentials = Credentials::new(Some(access_key), Some(secret_key), None, None, None).unwrap();
    let bucket = Bucket::new(bucket_name, region, credentials).unwrap();
    let results = bucket.list_blocking("assets/".to_string(), None).unwrap();
    for (list, code) in results {
        assert_eq!(200, code);
        println!("{:?}", list.contents);
        for item in list.contents{
            save_file(item.key, &bucket);
        }
    }
}


fn main() {

    dotenv::dotenv().ok();
    
    retrieve_dynamic_assets();


    rocket::ignite()
    .mount("/", routes![index, get_resource, logged_in, login, process_login])
    .mount("/admin", routes![admin, make_primary])
    .mount("/admin/add", routes![add_project, add_skill, add_experience, add_education])
    .mount("/admin/delete", routes![delete_project, delete_education, delete_experience, delete_skill])
    .mount("/admin/edit", routes![edit_project, edit_skill, edit_education, edit_experience])
    .mount("/admin/update", routes![update_education, update_project, update_skill, update_experience])
    .mount("/admin/upload", routes![upload])
    .attach(Template::fairing())
    .attach(DbConn::fairing())
    .launch();
}
