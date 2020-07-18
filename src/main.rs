#![deny(warnings)]

use std::env;
use std::net::Ipv4Addr;
use warp::Filter;

pub mod models;
pub mod schema;

#[macro_use]
extern crate diesel;
extern crate dotenv;

use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenv::dotenv;

mod filters;
mod handlers;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url).expect(&format!("Error connecting to {}", database_url))
}

#[tokio::main]
async fn main() {
    dotenv().ok();

    let host = env::var("HOST").expect("Host must be set");
    let port = env::var("PORT").expect("Port must be set");

    if env::var_os("RUST_LOG").is_none() {
        env::set_var("RUST_LOG", "projects=info");
    }
    pretty_env_logger::init();

    let cors = warp::cors()
        .allow_any_origin()
        .allow_methods(vec!["GET", "POST", "PUT", "DELETE"])
        .allow_header("content-type");

    let api = filters::project::project_filters();

    let routes = api.with(warp::log("projects")).with(cors);

    warp::serve(routes)
        .run((
            String::from(&host).parse::<Ipv4Addr>().unwrap(),
            port.parse::<u16>().unwrap(),
        ))
        .await;
}
