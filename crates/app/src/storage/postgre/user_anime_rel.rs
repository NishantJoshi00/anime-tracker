use diesel::{AsChangeset, Identifiable, Insertable, Queryable};

use super::schema;

#[derive(Debug, Identifiable, Queryable)]
#[diesel(table_name = schema::user_followed_animes)]
pub struct UserAnimeFollowing {
    id: i32,
    user_id: String,
    anime_id: String,
}

#[derive(Debug, Insertable)]
#[diesel(table_name = schema::user_followed_animes)]
pub struct UserAnimeFollowingNew {
    user_id: String,
    anime_id: String,
}
impl UserAnimeFollowingNew {
    pub fn new(user_id: String, anime_id: String) -> Self {
        Self { user_id, anime_id }
    }
}

