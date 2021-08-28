#[macro_use] extern crate rocket;
#[macro_use] extern crate diesel;

use diesel::prelude::*;
use rocket::serde::json::Json;
use std::path::PathBuf;

mod schema;
mod models;
mod database;
mod cors;
mod admin;

use crate::models::*;
use crate::schema::*;
use crate::admin::Admin;
use crate::database::DbConn;
use crate::cors::CORS;
use rocket::http::{Cookie, CookieJar, Status};
use rocket::response::Redirect;


#[get("/projects")]
async fn get_projects(conn: DbConn) -> Option<Json<Vec<Project>>> {

    conn.run(move |conn| {
        projects::table
        .load(conn)
    }
    ).await.map(Json).ok()

}

#[get("/experiences")]
async fn get_experiences(conn: DbConn) -> Option<Json<Vec<Experience>>> {
    conn.run(move |conn| {
        experiences::table
        .load(conn)
    }
    ).await.map(Json).ok()

}

#[post("/projects", data="<project>")]
async fn add_project(conn: DbConn, project: Json<NewProject>, _admin: Admin) -> Option<Json<Project>> {

        let new_project: Option<Project> = conn.run( move |conn| {
            diesel::insert_into(projects::table)
            .values(&*project)
            .get_results(conn)
        }).await.ok()?.pop();

        match new_project
        {
            Some(x) => Some(Json(x)),
            None => None
        }
  
}

#[post("/experiences", data="<experience>")]
async fn add_experience(conn: DbConn, experience: Json<NewExperience>, _admin: Admin) -> Option<Json<Experience>> {


        let new_experience: Option<Experience> = conn.run( move |conn| {
            diesel::insert_into(experiences::table)
            .values(&*experience)
            .get_results(conn)
        }).await.ok()?.pop();

        match new_experience
        {
            Some(x) => Some(Json(x)),
            None => None
        }
  
}

#[delete("/experiences/<id>")]
async fn remove_experience(conn: DbConn, id: i32, _admin: Admin) -> Option<()>
{
    let result = conn.run( move |conn| {
        diesel::delete(experiences::table.filter(experiences::id.eq(id)))
        .execute(conn)
    }).await.ok();  

    match result{
        Some(_) => Some(()),
        None => None
    }
}

#[delete("/projects/<id>")]
async fn remove_project(conn: DbConn, id: i32, _admin: Admin) -> Option<()>
{
    let result = conn.run( move |conn| {
        diesel::delete(projects::table.filter(projects::id.eq(id)))
        .execute(conn)
    }).await.ok();  

    match result{
        Some(_) => Some(()),
        None => None
    }
}

#[post("/login", data = "<login>")]
async fn process_login(login: Json<Login>, cookies: &CookieJar<'_>) -> Redirect{
    
    if login.username == "username" && login.password == "password"
    {
        println!("You have successfully entered uname and password");
        cookies.add_private(Cookie::new("admin", "benjamin"))
    }

    Redirect::to("/login")
}

#[get("/login", rank = 1)]
async fn logged_in(_admin: Admin) -> Json<LoginOutcome> {
    Json(
        LoginOutcome {
            success: true
        }
    )
}

#[get("/login", rank = 2)]
async fn login() -> Json<LoginOutcome> {
    Json(
        LoginOutcome {
            success: false
        }
    )
}

#[post("/logout")]
fn logout(cookies: &CookieJar<'_>)
{
    cookies.remove_private(Cookie::named("admin"));
}


#[options("/<_path..>")]
async fn catch_preflight<'a>(_path: PathBuf){ 
    //  Catches preflight OPTIONS requests which want CORS headers.
    //  Since our CORS fairing takes care of injecting those necessary headers,
    //  this handler doesn't need to do anything.
}


// These handle any authorized post, delete or put requests,
// since all requests asking to modify the database contain the Admin guard.

#[post("/<_path..>")]
async fn catch_unauth_posts<'a>(_path: PathBuf) -> Status{
    Status::Unauthorized
}

#[delete("/<_path..>")]
async fn catch_unauth_deletes<'a>(_path: PathBuf) -> Status{
    Status::Unauthorized
}

#[put("/<_path..>")]
async fn catch_unauth_puts<'a>(_path: PathBuf) -> Status{
    Status::Unauthorized
}


#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![
        get_projects, get_experiences,
        add_project, add_experience,
        remove_experience, remove_project,
        process_login, login, logged_in, logout,
        catch_unauth_posts, catch_unauth_deletes, catch_unauth_puts,
        catch_preflight
        ])

    .attach(DbConn::fairing())
    .attach(CORS)
}
