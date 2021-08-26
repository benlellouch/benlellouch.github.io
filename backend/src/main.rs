#[macro_use] extern crate rocket;
#[macro_use] extern crate diesel;

use crate::diesel::RunQueryDsl;
use rocket::serde::json::Json;

// type Result<T, E = Debug<diesel::result::Error>> = std::result::Result<T, E>;


mod schema;
mod models;
mod database;

use crate::models::*;
use crate::schema::*;
use crate::database::DbConn;

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
async fn add_project(conn: DbConn, project: Json<NewProject>) -> Option<Json<Project>> {

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
async fn add_experience(conn: DbConn, experience: Json<NewExperience>) -> Option<Json<Experience>> {

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


#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![get_projects, get_experiences, add_project, add_experience])
    .attach(DbConn::fairing())
}
