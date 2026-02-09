use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(diesel_derive_enum::DbEnum, Debug, Clone, Serialize, Deserialize)]
#[db_enum(existing_type_path = "crate::schema::sql_types::UserRole")]
pub enum UserRole {
    USER,
    ADMIN,
}

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Selectable)]
#[diesel(table_name = crate::schema::users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct User {
    pub id: i32,
    pub username: String,
    pub password: String,
    pub email: String,
    pub role: UserRole,
}

#[derive(Debug, Insertable)]
#[diesel(table_name = crate::schema::users)]
pub struct NewUser {
    pub username: String,
    pub password: String,
    pub email: String,
}
