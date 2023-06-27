use diesel::prelude::*;
use diesel::deserialize::FromStaticSqlRow;
use diesel::sql_types::{Integer, Text, Timestamp};
use rocket::serde::{Deserialize, Serialize};
use chrono::{NaiveDateTime, Utc};
use crate::schemas::schema::teams;
use crate::schemas::schema::users;

#[derive(Serialize, Queryable)]
#[serde(crate = "rocket::serde")]
pub struct Users {
    pub user_id: i32,
    pub name_user: String,
    pub password: String,
    pub data_register: NaiveDateTime,
    //pub date_update: Option<NaiveDateTime>,
    pub team: i32,
}

#[derive(Insertable, Deserialize)]
#[serde(crate = "rocket::serde")]
#[diesel(table_name = users)]
pub struct New_Users {
    pub name_user: Option<String>,
    pub password: Option<String>,
    //pub date_register: DateTime,
    //pub date_update: Option<String>,
    pub team: i32
}

#[derive(Serialize, Queryable)]
#[serde(crate = "rocket::serde")]
pub struct Teams {
    pub team_id: i32,
    pub type_teams: String,
    pub description: String,
}

#[derive(Insertable, Deserialize)]
#[serde(crate = "rocket::serde")]
#[diesel(table_name = teams)]
pub struct New_Teams {
    pub type_teams: Option<String>,
    pub description: Option<String>
}

#[derive(Serialize, Queryable)]
#[serde(crate = "rocket::serde")]
pub struct USUARIOWEB {
    pub usrid: i32,
    pub usrnomelogin: String,
    pub usrsenha: String,
}