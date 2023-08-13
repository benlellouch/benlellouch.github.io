#[macro_use]
extern crate rocket;

use diesel::prelude::*;
use rocket::serde::json::Json;
use std::path::PathBuf;

mod admin;
mod cors;
mod database;
mod models;
mod schema;

use crate::admin::Admin;
use crate::cors::Cors;
use crate::database::DbConn;
use crate::models::*;
use crate::schema::*;
use rocket::http::{Cookie, CookieJar, Status};
use rocket::response::Redirect;

#[get("/projects")]
async fn get_projects(conn: DbConn) -> Option<Json<Vec<Project>>> {
    conn.run(move |conn| projects::table.load(conn))
        .await
        .map(Json)
        .ok()
}

#[get("/experiences")]
async fn get_experiences(conn: DbConn) -> Option<Json<Vec<Experience>>> {
    conn.run(move |conn| experiences::table.load(conn))
        .await
        .map(Json)
        .ok()
}

#[get("/profile")]
async fn get_profiles(conn: DbConn) -> Option<Json<Vec<Profile>>> {
    conn.run(move |conn| profiles::table.load(conn))
        .await
        .map(Json)
        .ok()
}

#[post("/projects", data = "<project>")]
async fn add_project(
    conn: DbConn,
    project: Json<NewProject>,
    _admin: Admin,
) -> Option<Json<Project>> {
    let new_project: Option<Project> = conn
        .run(move |conn| {
            diesel::insert_into(projects::table)
                .values(&*project)
                .get_results(conn)
        })
        .await
        .ok()?
        .pop();

    new_project.map(Json)
}

#[post("/experiences", data = "<experience>")]
async fn add_experience(
    conn: DbConn,
    experience: Json<NewExperience>,
    _admin: Admin,
) -> Option<Json<Experience>> {
    let new_experience: Option<Experience> = conn
        .run(move |conn| {
            diesel::insert_into(experiences::table)
                .values(&*experience)
                .get_results(conn)
        })
        .await
        .ok()?
        .pop();

    new_experience.map(Json)
}

#[post("/profile", data = "<profile>")]
async fn add_profile(
    conn: DbConn,
    profile: Json<NewProfile>,
    _admin: Admin,
) -> Option<Json<Profile>> {
    let new_profile: Option<Profile> = conn
        .run(move |conn| {
            diesel::insert_into(profiles::table)
                .values(&*profile)
                .get_results(conn)
        })
        .await
        .ok()?
        .pop();

    new_profile.map(Json)
}

#[delete("/experiences/<id>")]
async fn remove_experience(conn: DbConn, id: i32, _admin: Admin) -> Option<()> {
    let result = conn
        .run(move |conn| {
            diesel::delete(experiences::table.filter(experiences::id.eq(id))).execute(conn)
        })
        .await
        .ok();

    match result {
        Some(_) => Some(()),
        None => None,
    }
}

#[delete("/projects/<id>")]
async fn remove_project(conn: DbConn, id: i32, _admin: Admin) -> Option<()> {
    let result = conn
        .run(move |conn| diesel::delete(projects::table.filter(projects::id.eq(id))).execute(conn))
        .await
        .ok();

    match result {
        Some(_) => Some(()),
        None => None,
    }
}

#[post("/login", data = "<login>")]
async fn process_login(login: Json<Login>, cookies: &CookieJar<'_>) -> Redirect {
    if login.username == "username" && login.password == "password" {
        println!("You have successfully entered uname and password");
        cookies.add_private(Cookie::new("admin", "benjamin"))
    }

    Redirect::to("/login")
}

#[get("/login", rank = 1)]
async fn logged_in(_admin: Admin) -> Json<LoginOutcome> {
    Json(LoginOutcome { success: true })
}

#[get("/login", rank = 2)]
async fn login() -> Json<LoginOutcome> {
    Json(LoginOutcome { success: false })
}

#[post("/logout")]
fn logout(cookies: &CookieJar<'_>) {
    cookies.remove_private(Cookie::named("admin"));
}

#[options("/<_path..>")]
async fn catch_preflight<'a>(_path: PathBuf) {
    //  Catches preflight OPTIONS requests which want CORS headers.
    //  Since our CORS fairing takes care of injecting those necessary headers,
    //  this handler doesn't need to do anything.
}

// These handle any authorized post, delete or put requests,
// since all requests asking to modify the database contain the Admin guard.

#[post("/<_path..>")]
async fn catch_unauth_posts<'a>(_path: PathBuf) -> Status {
    Status::Unauthorized
}

#[delete("/<_path..>")]
async fn catch_unauth_deletes<'a>(_path: PathBuf) -> Status {
    Status::Unauthorized
}

#[put("/<_path..>")]
async fn catch_unauth_puts<'a>(_path: PathBuf) -> Status {
    Status::Unauthorized
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount(
            "/",
            routes![
                get_projects,
                get_experiences,
                get_profiles,
                add_project,
                add_experience,
                add_profile,
                remove_experience,
                remove_project,
                process_login,
                login,
                logged_in,
                logout,
                catch_unauth_posts,
                catch_unauth_deletes,
                catch_unauth_puts,
                catch_preflight
            ],
        )
        .attach(DbConn::fairing())
        .attach(Cors)
}
