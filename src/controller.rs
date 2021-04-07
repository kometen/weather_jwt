use actix_web::Responder;
use actix_web::{web, Error, HttpResponse};
use super::Pool;
use diesel::dsl::{delete, insert_into};
use serde::{Deserialize, Serialize};
use std::vec::Vec;
use chrono::{Local, DateTime};
use actix_web::error::DispatchError::H2;
use crate::models::Location;
use crate::schema::locations::dsl::locations;
use diesel::{RunQueryDsl, QueryDsl};

pub async fn get_locations(db: web::Data<Pool>) -> Result<HttpResponse, Error> {
    Ok(web::block(move || get_all_locations(db))
        .await
        .map(|station| HttpResponse::Ok().json(station))
        .map_err(|_| HttpResponse::InternalServerError())?)
}

fn get_all_locations(pool: web::Data<Pool>) -> Result<Vec<Location>, diesel::result::Error> {
    let conn = pool.get().unwrap();
    let items = locations.load::<Location>(&conn)?;
    Ok(items)
}

pub async fn get_location_by_id(db: web::Data<Pool>, location_id: web::Path<i32>) -> Result<HttpResponse, Error> {
    Ok(web::block(move || db_get_location_by_id(db, location_id.into_inner()))
        .await
        .map(|location| HttpResponse::Ok().json(location))
        .map_err(|_| HttpResponse::InternalServerError())?
    )
}

fn db_get_location_by_id(pool: web::Data<Pool>, location_id: i32) -> Result<Location, diesel::result::Error> {
    let conn = pool.get().unwrap();
    locations.find(location_id).get_result::<Location>(&conn)
}
