use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Serialize, Deserialize, FromRow, Debug)]
pub struct Result {
    pub id: i32,
    pub url: String,
    pub job_title: String,
    pub job_location: String,
    pub scrape_date: NaiveDateTime,
    pub count: i32,
}

#[derive(Serialize, Deserialize, FromRow, Debug, Clone)]
pub struct Target {
    pub id: i32,
    pub url: String,
    pub job_title: String,
    pub job_location: String,
}

#[derive(Deserialize, Debug)]
pub struct DbSecrets {
    pub username: String,
    pub password: String,
}
