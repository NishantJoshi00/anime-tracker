use diesel::{AsChangeset, Identifiable, Insertable, Queryable};

use super::schema;
use crate::utils;

#[derive(Debug, Identifiable, Queryable)]
#[diesel(table_name = schema::users)]
pub struct User {
    pub id: i32,
    pub user_id: String,
    pub username: String,
    pub email: String,
    pub authenticated: bool,
    pub created_at: time::PrimitiveDateTime,
}

#[derive(Debug, Insertable)]
#[diesel(table_name = schema::users)]
pub struct UserNew {
    user_id: String,
    username: String,
    email: String,
}
impl UserNew {
    pub fn new(username: String, email: String) -> Self {
        Self {
            user_id: utils::uuid_generator("user"),
            username,
            email,
        }
    }
}

#[derive(Debug)]
pub enum UserUpdate {
    Authenticated,
    UpdateEmail { email: String },
    UpdateUsername { username: String },
}

#[derive(Debug, AsChangeset, Default)]
#[diesel(table_name = schema::users)]
pub struct PGUserUpdate {
    username: Option<String>,
    email: Option<String>,
    authenticated: Option<bool>,
}

impl From<UserUpdate> for PGUserUpdate {
    fn from(value: UserUpdate) -> Self {
        match value {
            UserUpdate::Authenticated => Self {
                authenticated: Some(true),
                ..Default::default()
            },
            UserUpdate::UpdateEmail { email } => Self {
                email: Some(email),
                ..Default::default()
            },
            UserUpdate::UpdateUsername { username } => Self {
                username: Some(username),
                ..Default::default()
            },
        }
    }
}
