#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
use rocket::serde::json::{json, Value};

#[macro_use]
extern crate rocket_sync_db_pools;

extern crate rocket_cors;
use rocket_cors::{Cors, CorsOptions};

#[macro_use]
extern crate diesel;

#[macro_use]
extern crate validator_derive;

use dotenv::dotenv;

mod config;
mod controllers;
mod errors;
mod jwt_auth;
mod models;
mod redis_connections;
mod schema;
mod services;
mod ws_server;

#[catch(404)]
fn not_found() -> Value {
    json!({
        "status": "error",
        "reason": "Resource was not found."
    })
}

fn cors_fairing() -> Cors {
    CorsOptions::default()
        .to_cors()
        .expect("Cors fairing cannot be created")
}

#[launch]
pub fn rocket() -> _ {
    dotenv().ok();
    ws_server::launch();
    rocket::custom(config::from_env())
        .mount(
            "/api/v1",
            [
                controllers::users::routes(),
                controllers::auth::routes(),
                controllers::friends::routes(),
            ]
            .concat(),
        )
        .attach(services::Db::fairing())
        .attach(cors_fairing())
        .attach(config::AppState::manage())
        .attach(redis_connections::RedisClient::manage())
        .register("/", catchers![not_found])
}
