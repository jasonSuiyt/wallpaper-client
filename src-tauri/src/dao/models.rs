use chrono::NaiveDate;
use diesel::prelude::*;
use serde::Deserialize;
use serde::Serialize;

#[derive(Queryable, Selectable, Debug, Serialize, Deserialize)]
#[diesel(table_name = crate::dao::schema::bing)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
#[derive(Clone)]
pub struct Bing {
    pub id: i32,
    pub name: String,
    pub url: String,
    pub uhd_url: String,
    pub uhd_file_path: String,
    pub normal_file_path: String,
    pub source: String,
    pub created_date: NaiveDate,
}

#[derive(Insertable, Clone, Debug)]
#[diesel(table_name = crate::dao::schema::bing)]
pub struct NewBing {
    pub name: String,
    pub url: String,
    pub uhd_url: String,
    pub uhd_file_path: String,
    pub normal_file_path: String,
    pub source: String,
    pub created_date: NaiveDate,
}

#[derive(Clone, Debug)]
pub struct Page {
    pub data: Vec<Bing>,
    pub totals: i64,
    pub current_page: i64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DownloadPayload {
    pub id: i32,
    pub process: f64,
    pub text: String,
}
