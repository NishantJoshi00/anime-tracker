use diesel::{AsChangeset, Identifiable, Insertable, Queryable};

use crate::utils;

use super::schema;

#[derive(Debug, Identifiable, Queryable)]
#[diesel(table_name = schema::animes)]
pub struct Anime {
    id: i32,
    anime_id: String,
    name: String,
    tracking_data: Option<serde_json::Value>,
    completed: bool,
}

#[derive(Debug, Insertable)]
#[diesel(table_name = schema::animes)]
pub struct AnimeNew {
    anime_id: String,
    name: String,
    tracking_data: Option<serde_json::Value>,
    completed: bool,
}

impl AnimeNew {
    pub fn new(name: String, tracking_data: Option<serde_json::Value>, completed: bool) -> Self {
        Self {
            anime_id: utils::uuid_generator("anime"),
            name,
            tracking_data,
            completed,
        }
    }
}

#[derive(Debug)]
pub enum AnimeUpdate {
    UpdateCompleteStatus { status: bool },
    UpdateTrackingData { data: Option<serde_json::Value> },
    UpdateName { name: String },
}

#[derive(Debug, AsChangeset, Default)]
#[diesel(table_name = schema::animes)]
struct PGAnimeUpdate {
    name: Option<String>,
    tracking_data: Option<Option<serde_json::Value>>,
    completed: Option<bool>,
}

impl From<AnimeUpdate> for PGAnimeUpdate {
    fn from(value: AnimeUpdate) -> Self {
        match value {
            AnimeUpdate::UpdateCompleteStatus { status } => Self {
                completed: Some(status),
                ..Default::default()
            },
            AnimeUpdate::UpdateTrackingData { data } => Self {
                tracking_data: Some(data),
                ..Default::default()
            },
            AnimeUpdate::UpdateName { name } => Self {
                name: Some(name),
                ..Default::default()
            },
        }
    }
}
