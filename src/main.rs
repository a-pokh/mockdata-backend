#![deny(warnings)]

use std::env;
use warp::Filter;

pub mod schema;
pub mod models;

#[macro_use]
extern crate diesel;
extern crate dotenv;

use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenv::dotenv;

mod filters;
mod handlers;
mod services;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}


#[tokio::main]
async fn main() {
    if env::var_os("RUST_LOG").is_none() {
        env::set_var("RUST_LOG", "projects=info");
    }
    pretty_env_logger::init();

    let api = filters::project::project_filters();

    let routes = api.with(warp::log("projects"));
    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}

