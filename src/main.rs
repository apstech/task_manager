#[macro_use]
extern crate rocket;

use rocket::{Build, Rocket};

mod database;
use database::database::*;

mod models;
use models::models::*;

mod schemas;
use schemas::schema::*;

mod controllers;
use controllers::controller;

#[launch]
fn rocket() -> Rocket<Build> {
    rocket::build()
        .mount(
            "/",
            routes![
                controllers::controller::login,
                controllers::controller::users,
                controllers::controller::teams,
                controllers::controller::new_team,
                controllers::controller::new_user,
                controllers::controller::unauthorized
            ],
        )
        .register(
            "/",
            catchers![
                controllers::controller::not_found,
                controllers::controller::not_autorized
            ],
        )
}
