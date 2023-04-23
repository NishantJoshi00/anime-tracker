use std::sync::Arc;

use diesel_async::{
    pooled_connection::{self, deadpool::Pool},
    AsyncPgConnection,
};
use error_stack::{IntoReport, ResultExt};

use crate::errors::{self, EResult};

pub mod postgre;
pub(self) mod schema;

pub trait State {}

#[derive(Clone)]
pub struct Storage {
    pg_pool: Arc<Pool<AsyncPgConnection>>,
}

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

trait UserInterface {}

trait Tower {
    type NextTarget;
}
