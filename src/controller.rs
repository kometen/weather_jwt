use super::Pool;
use crate::models::{LatestReading, Location, Reading};
use crate::schema::latest_readings::dsl::latest_readings;
use crate::schema::locations::dsl::locations;
use crate::schema::readings::columns::{id, measurement_time_default};
use crate::schema::readings::dsl::readings;
use actix_web::{web, Error, HttpResponse};
use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl};
use std::vec::Vec;

pub async fn get_locations(db: web::Data<Pool>) -> Result<HttpResponse, Error> {
    Ok(web::block(move || db_get_locations(db))
        .await
        .map(|station| HttpResponse::Ok().json(station))
        .map_err(|_| HttpResponse::InternalServerError())?)
}

fn db_get_locations(pool: web::Data<Pool>) -> Result<Vec<Location>, diesel::result::Error> {
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
    Ok(web::block(move || db_get_readings(db))
        .await
        .map(|reading| HttpResponse::Ok().json(reading))
        .map_err(|_| HttpResponse::InternalServerError())?)
}

fn db_get_readings(pool: web::Data<Pool>) -> Result<Vec<LatestReading>, diesel::result::Error> {
    let conn = pool.get().unwrap();
    latest_readings.load::<LatestReading>(&conn)
}

pub async fn get_readings_by_id(
    db: web::Data<Pool>,
    reading_id: web::Path<i32>,
) -> Result<HttpResponse, Error> {
    Ok(
        web::block(move || db_get_readings_by_id(db, reading_id.into_inner()))
            .await
            .map(|reading| HttpResponse::Ok().json(reading))
            .map_err(|_| HttpResponse::InternalServerError())?,
    )
}

fn db_get_readings_by_id(
    pool: web::Data<Pool>,
    reading_id: i32,
) -> Result<Vec<Reading>, diesel::result::Error> {
    let conn = pool.get().unwrap();
    readings
        .order(measurement_time_default.desc())
        .limit(256)
        .filter(id.eq(reading_id))
        .load::<Reading>(&conn)
}
