use bigdecimal::BigDecimal;
use chrono::{DateTime, Local, Utc};
use diesel::backend::Backend;
use diesel::deserialize::Queryable;
use serde::Serialize;

#[derive(Serialize, Queryable)]
pub struct Reading {
    #[diesel(deserialize_as = "MyDateTimeWrapper")]
    pub measurement_time_default: DateTime<Local>,
    pub id: i32,
    pub data: serde_json::Value,
}

#[derive(Serialize, Queryable)]
pub struct LatestReading {
    #[diesel(deserialize_as = "MyDateTimeWrapper")]
    pub measurement_time_default: DateTime<Local>,
    pub id: i32,
    pub data: serde_json::Value,
}

#[derive(Serialize, Queryable)]
pub struct Location {
    #[diesel(deserialize_as = "MyDateTimeWrapper")]
    pub publication_time: DateTime<Local>,
    pub id: i32,
    pub name: String,
    pub latitude: BigDecimal,
    pub longitude: BigDecimal,
}

pub struct MyDateTimeWrapper(DateTime<Local>);

impl Into<DateTime<Local>> for MyDateTimeWrapper {
    fn into(self) -> DateTime<Local> {
        self.0
    }
}

impl<DB, ST> Queryable<ST, DB> for MyDateTimeWrapper
where
    DB: Backend,
    DateTime<Utc>: Queryable<ST, DB>,
{
    type Row = <DateTime<Utc> as Queryable<ST, DB>>::Row;

    fn build(row: Self::Row) -> Self {
        Self(<DateTime<Utc> as Queryable<ST, DB>>::build(row).with_timezone(&Local))
    }
}
