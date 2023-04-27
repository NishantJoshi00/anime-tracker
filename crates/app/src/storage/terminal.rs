use async_trait::async_trait;
use error_stack::IntoReport;

use crate::errors::{self, EResult};

use super::{types, UserInterface};

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
