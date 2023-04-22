use diesel::{AsChangeset, Identifiable, Insertable, Queryable};

use super::schema;
use crate::utils;

#[derive(Debug, Identifiable, Queryable)]
#[diesel(table_name = schema::users)]
pub struct User {
    id: i32,
    user_id: String,
    username: String,
    email: String,
    authenticated: bool,
    created_at: time::PrimitiveDateTime,
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
struct PGUserUpdate {
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
