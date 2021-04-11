use bigdecimal::BigDecimal;
use chrono::{DateTime, Utc};
use diesel::deserialize::Queryable;
use serde::Serialize;

#[derive(Serialize, Queryable)]
pub struct Reading {
    pub measurement_time_default: DateTime<Utc>,
    pub id: i32,
    pub index: i32,
    pub field_description: String,
    pub measurement: BigDecimal,
}

#[derive(Serialize, Queryable)]
pub struct Location {
    pub publication_time: DateTime<Utc>,
    pub id: i32,
    pub name: String,
    pub latitude: BigDecimal,
    pub longitude: BigDecimal,
}
