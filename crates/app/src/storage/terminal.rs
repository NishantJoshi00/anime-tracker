use async_trait::async_trait;
use error_stack::IntoReport;

use crate::errors::{self, EResult};

use super::{types, AnimeInterface, AnimeUserRelInterface, UserInterface};

#[async_trait]
impl UserInterface for () {
    type NextLayer = ();
    type Error = errors::TowerError;

    async fn find_user_by_id(&self, _id: String) -> EResult<types::users::User, Self::Error> {
        Err(errors::TowerError::TowerEndError).into_report()
    }
    async fn find_user_by_username(
        &self,
        _username: String,
    ) -> EResult<types::users::User, Self::Error> {
        Err(errors::TowerError::TowerEndError).into_report()
    }
    async fn find_user_by_email(&self, _email: String) -> EResult<types::users::User, Self::Error> {
        Err(errors::TowerError::TowerEndError).into_report()
    }

    async fn add_new_user(
        &self,
        _new: types::users::UserNew,
    ) -> EResult<types::users::User, Self::Error> {
        Err(errors::TowerError::TowerEndError).into_report()
    }

    async fn update_user(
        &self,
        _base: types::users::User,
        _update: types::users::UserUpdate,
    ) -> EResult<types::users::User, Self::Error> {
        Err(errors::TowerError::TowerEndError).into_report()
    }
}

#[async_trait]
impl AnimeInterface for () {
    type NextLayer = ();
    type Error = errors::TowerError;

    async fn find_anime_by_id(&self, _id: String) -> EResult<types::animes::Anime, Self::Error> {
        Err(errors::TowerError::TowerEndError).into_report()
    }
    async fn find_anime_like_name(
        &self,
        _name: String,
    ) -> EResult<types::animes::Anime, Self::Error> {
        Err(errors::TowerError::TowerEndError).into_report()
    }
    async fn add_new_anime(
        &self,
        _new: types::animes::AnimeNew,
    ) -> EResult<types::animes::Anime, Self::Error> {
        Err(errors::TowerError::TowerEndError).into_report()
    }

    async fn update_anime(
        &self,
        _base: types::animes::Anime,
        _update: types::animes::AnimeUpdate,
    ) -> EResult<types::animes::Anime, Self::Error> {
        Err(errors::TowerError::TowerEndError).into_report()
    }
}

#[async_trait]
impl AnimeUserRelInterface for () {
    type NextLayer = ();
    type Error = errors::TowerError;

    async fn find_anime_ids_by_user_id(&self, _id: String) -> EResult<Vec<String>, Self::Error> {
        Err(errors::TowerError::TowerEndError).into_report()
    }
    async fn create_relation(
        &self,
        _user_id: String,
        _anime_id: String,
    ) -> EResult<types::user_anime_rel::UserAnimeFollowing, Self::Error> {
        Err(errors::TowerError::TowerEndError).into_report()
    }
    async fn delete_relation(
        &self,
        user_id: String,
        _anime_id: String,
    ) -> EResult<(), Self::Error> {
        Err(errors::TowerError::TowerEndError).into_report()
    }
}
