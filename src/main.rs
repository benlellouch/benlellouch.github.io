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

use s3::bucket::Bucket;
// use s3::S3Error;
use s3::creds::Credentials;
use s3::region::Region;


extern crate rocket_auth_login as auth;
use auth::authorization::*;

use dotenv::dotenv;
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
        location: "Los Angeles, CA".to_string(),
        title: "Software Engineer".to_string(),
        profile_picture_path: "path/to/picture".to_string(),
        email: "john@appleseed.com".to_string(),
        about_me: "Lorem Ipsum".to_string(),
        github_link: "https://github.com".to_string(),
        linkedin_link: "https://linkedin.com".to_string()
    }
    );

    let paths = std::fs::read_dir("assets/images/projects/").unwrap();
    let image_paths: Vec<String> = paths.map(|p| String::from(p.unwrap().path().to_str().unwrap())).collect();


    MainTemplate
    {
        projects,
        skills,
        education,
        experience,
        languages,
        profile,
        image_paths
        
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
            match std::fs::write(&key, data)
            {
                Ok(_) => (),
                Err(_) => panic!("Was not able to save file: {}", key),
            }
        }

        _ => ()
    };

}

fn retrieve_dynamic_assets()
{
    let access_key = &dotenv::var("ACCESSKEY").unwrap();
    let secret_key = &dotenv::var("SECRETKEY").unwrap();
    let bucket_name = &dotenv::var("BUCKET").unwrap();
    let region: Region = "eu-west-2".parse().unwrap();
    let credentials = Credentials::new_blocking(Some(access_key), Some(secret_key), None, None, None).unwrap();
    let bucket = Bucket::new(bucket_name, region, credentials).unwrap();
    let results = bucket.list_blocking("".to_string(), None).unwrap();
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
    .mount("/admin/edit", routes![edit_project, edit_skill, edit_education, edit_experience, edit_profile])
    .mount("/admin/update", routes![update_education, update_project, update_skill, update_experience, update_profile])
    .mount("/admin/upload", routes![upload])
    .attach(Template::fairing())
    .attach(DbConn::fairing())
    .launch();
}
