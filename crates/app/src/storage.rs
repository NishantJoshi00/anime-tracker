use std::sync::Arc;

use async_trait::async_trait;
use diesel_async::{
    pooled_connection::{
        self,
        deadpool::{Object, Pool},
    },
    AsyncPgConnection,
};
use error_stack::{IntoReport, ResultExt};

use crate::errors::{self, EResult};

pub mod db;
pub(crate) mod schema;
pub mod types;

pub trait State {}

#[derive(Clone)]
pub struct Storage {
    pg_pool: Arc<Pool<AsyncPgConnection>>,
}

type DeadPoolConnType = Object<AsyncPgConnection>;

impl Storage {
    pub async fn new(config: Config) -> EResult<Self, errors::StorageError> {
        let config = pooled_connection::AsyncDieselConnectionManager::<AsyncPgConnection>::new(
            config.database_url,
        );
        let pool = Pool::builder(config).build().into_report().change_context(
            errors::StorageError::ConfigurationError {
                message: "Failed while building database pool",
            },
        )?;

        Ok(Self {
            pg_pool: Arc::new(pool),
        })
    }

    pub async fn get_conn(&self) -> EResult<DeadPoolConnType, errors::DatabaseError> {
        self.pg_pool
            .get()
            .await
            .into_report()
            .change_context(errors::DatabaseError::PoolClientFailure)
    }
}

pub struct Config {
    pub database_url: String,
}

impl State for Storage {}

/*
 *
 * ## SuperTrait Referencing
 * - Consider situation where we have a trait Alpha.
 *  ```rust
 *  trait Alpha<T>: Nexter<NextTarget = T> {}
 *  trait Nexter {
 *      type NextTarget;
 *  }
 *  ```
 *
 *  - In this scenerio we wish that the type `NextTarget` should know about the baby-trait that
 *  `Nexter` is used as a super-trait.
 *
 *  ```rust
 *  trait Nexter<T> {
 *      type NextTarget: Alpha<T> // This Alpha<T> should be auto referenced from the baby-trait
 *  }
 *  ```
 *
 *
 */

trait Tower {
    type NextTarget: Tower;
    fn next(&self) -> &Self::NextTarget;
}

impl Tower for () {
    type NextTarget = ();
    fn next(&self) -> &Self::NextTarget {
        self
    }
}

impl Tower for Storage {
    type NextTarget = ();
    fn next(&self) -> &Self::NextTarget {
        &()
    }
}

#[async_trait]
trait UserInterface: Tower<NextTarget = Self::NextLayer> {
    type NextLayer: UserInterface;
    type Error;

    async fn find_user_by_id(&self, id: String) -> EResult<types::users::User, Self::Error>;
    async fn find_user_by_username(
        &self,
        username: String,
    ) -> EResult<types::users::User, Self::Error>;
    async fn find_user_by_email(&self, email: String) -> EResult<types::users::User, Self::Error>;

    async fn add_new_user(
        &self,
        new: types::users::UserNew,
    ) -> EResult<types::users::User, Self::Error>;

    async fn update_user(
        &self,
        base: types::users::User,
        update: types::users::UserUpdate,
    ) -> EResult<types::users::User, Self::Error>;
}

#[async_trait]
trait AnimeInterface: Tower<NextTarget = Self::NextLayer> {
    type NextLayer: AnimeInterface;
    type Error;

    async fn find_anime_by_id(&self, id: String) -> EResult<types::animes::Anime, Self::Error>;
    async fn find_anime_like_name(
        &self,
        name: String,
    ) -> EResult<types::animes::Anime, Self::Error>;
    async fn add_new_anime(
        &self,
        new: types::animes::AnimeNew,
    ) -> EResult<types::animes::Anime, Self::Error>;

    async fn update_anime(
        &self,
        base: types::animes::Anime,
        update: types::animes::AnimeUpdate,
    ) -> EResult<types::animes::Anime, Self::Error>;
}

#[async_trait]
trait AnimeUserRelInterface: Tower<NextTarget = Self::NextLayer> {
    type NextLayer: AnimeUserRelInterface;
    type Error;

    async fn find_anime_ids_by_user_id(&self, id: String) -> EResult<Vec<String>, Self::Error>;
    async fn create_relation(
        &self,
        user_id: String,
        anime_id: String,
    ) -> EResult<types::user_anime_rel::UserAnimeFollowing, Self::Error>;
    async fn delete_relation(&self, user_id: String, anime_id: String) -> EResult<(), Self::Error>;
}

mod terminal;
