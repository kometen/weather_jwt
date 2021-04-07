use serde::{Serialize};
use chrono::{Local, DateTime, NaiveDateTime};
use diesel::deserialize::Queryable;
use bigdecimal::BigDecimal;

#[derive(Serialize, Queryable)]
pub struct Reading {
    pub measurement_time_default: NaiveDateTime,
    pub id: i32,
    pub index: i32,
    pub field_description: String,
    pub measurement: f64,
}

#[derive(Serialize, Queryable)]
pub struct Location {
    pub publication_time: NaiveDateTime,
    pub id: i32,
    pub name: String,
    pub latitude: BigDecimal,
    pub longitude: BigDecimal,
}
