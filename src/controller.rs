use super::Pool;
use crate::models::{Location, Reading};
use crate::schema::locations::dsl::locations;
use crate::schema::readings::columns::measurement_time_default;
use crate::schema::readings::dsl::readings;
use actix_web::Responder;
use actix_web::{web, Error, HttpResponse};
use chrono::{DateTime, Local};
use diesel::dsl::{delete, insert_into};
use diesel::{QueryDsl, RunQueryDsl};
use serde::{Deserialize, Serialize};
use std::vec::Vec;

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

pub async fn get_location_by_id(
    db: web::Data<Pool>,
    location_id: web::Path<i32>,
) -> Result<HttpResponse, Error> {
    Ok(
        web::block(move || db_get_location_by_id(db, location_id.into_inner()))
            .await
            .map(|location| HttpResponse::Ok().json(location))
            .map_err(|_| HttpResponse::InternalServerError())?,
    )
}

fn db_get_location_by_id(
    pool: web::Data<Pool>,
    location_id: i32,
) -> Result<Location, diesel::result::Error> {
    let conn = pool.get().unwrap();
    locations.find(location_id).get_result::<Location>(&conn)
}

pub async fn get_readings(db: web::Data<Pool>) -> Result<HttpResponse, Error> {
    Ok(web::block(move || get_all_readings(db))
        .await
        .map(|reading| HttpResponse::Ok().json(reading))
        .map_err(|_| HttpResponse::InternalServerError())?)
}

fn get_all_readings(pool: web::Data<Pool>) -> Result<Vec<Reading>, diesel::result::Error> {
    let conn = pool.get().unwrap();
    let items = readings
        .order_by(measurement_time_default)
        .limit(256)
        .load::<Reading>(&conn)?;
    Ok(items)
}
