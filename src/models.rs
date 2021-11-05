use crate::schema::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Queryable)]
#[table_name = "users"]
pub struct User {
    pub id: i32,
    pub name: String,
    pub email: String,
    pub created_at: chrono::NaiveDateTime,
}

#[derive(Insertable, Debug)]
#[table_name = "users"]
pub struct NewUser {
    pub name: String,
    pub email: String,
    pub created_at: chrono::NaiveDateTime,
}