// https://auth0.com/blog/build-an-api-in-rust-with-jwt-authentication-using-actix-web/
#[macro_use]
extern crate diesel;

mod auth;
mod controller;
mod errors;
mod models;
mod schema;

use actix_web::{dev::ServiceRequest, web, App, Error, HttpServer};
use actix_web_httpauth::extractors::bearer::{BearerAuth, Config};
use actix_web_httpauth::extractors::AuthenticationError;
use actix_web_httpauth::middleware::HttpAuthentication;
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};

pub type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;

async fn validator(req: ServiceRequest, credentials: BearerAuth) -> Result<ServiceRequest, Error> {
    let config = req
        .app_data::<Config>()
        .cloned()
        .unwrap_or_else(Default::default);
    match auth::validate_token(credentials.token()) {
        Ok(res) => {
            if res {
                Ok(req)
            } else {
                Err(AuthenticationError::from(config).into())
            }
        }
        Err(_) => Err(AuthenticationError::from(config).into()),
    }
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    std::env::set_var("RUST_LOG", "actix_web=debug");
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL not set");

    let manager = ConnectionManager::<PgConnection>::new(database_url);
    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.");

    HttpServer::new(move || {
        let auth = HttpAuthentication::bearer(validator);
        App::new()
            .wrap(auth)
            .data(web::PayloadConfig::new(1 << 25))
            .data(pool.clone())
            .route("/locations", web::get().to(controller::get_locations))
            .route(
                "/locations/{id}",
                web::get().to(controller::get_location_by_id),
            )
            .route("/readings", web::get().to(controller::get_readings))
            .route(
                "/readings/{id}",
                web::get().to(controller::get_readings_by_id),
            )
    })
    .bind("127.0.0.1:8090")?
    .run()
    .await
}
