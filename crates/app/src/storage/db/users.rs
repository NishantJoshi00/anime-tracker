use async_trait::async_trait;
use diesel::{associations::HasTable, ExpressionMethods, QueryDsl};
use diesel_async::RunQueryDsl;
use error_stack::{IntoReport, ResultExt};
use types::schema;

use crate::{
    errors::{self, EResult},
    storage::{types, Storage, UserInterface},
};

#[async_trait]
impl UserInterface for Storage {
    type NextLayer = ();
    type Error = errors::DatabaseError;

    async fn find_user_by_id(&self, id: String) -> EResult<types::users::User, Self::Error> {
        let mut conn = self
            .pg_pool
            .get()
            .await
            .into_report()
            .change_context(errors::DatabaseError::PoolClientFailure)?;

        types::users::User::table()
            .filter(schema::users::dsl::user_id.eq(id))
            .get_result(&mut conn)
            .await
            .into_report()
            .change_context(errors::DatabaseError::FindQueryFailed)
    }
    async fn find_user_by_username(
        &self,
        username: String,
    ) -> EResult<types::users::User, Self::Error> {
        let mut conn = self
            .pg_pool
            .get()
            .await
            .into_report()
            .change_context(errors::DatabaseError::PoolClientFailure)?;

        types::users::User::table()
            .filter(schema::users::dsl::username.eq(username))
            .get_result(&mut conn)
            .await
            .into_report()
            .change_context(errors::DatabaseError::FindQueryFailed)
    }
    async fn find_user_by_email(&self, email: String) -> EResult<types::users::User, Self::Error> {
        let mut conn = self
            .pg_pool
            .get()
            .await
            .into_report()
            .change_context(errors::DatabaseError::PoolClientFailure)?;

        types::users::User::table()
            .filter(schema::users::dsl::email.eq(email))
            .get_result(&mut conn)
            .await
            .into_report()
            .change_context(errors::DatabaseError::FindQueryFailed)
    }

    async fn add_new_user(
        &self,
        new: types::users::UserNew,
    ) -> EResult<types::users::User, Self::Error> {
        let mut conn = self
            .pg_pool
            .get()
            .await
            .into_report()
            .change_context(errors::DatabaseError::PoolClientFailure)?;

        let query = diesel::insert_into(types::users::User::table()).values(new);

        query
            .get_result(&mut conn)
            .await
            .into_report()
            .change_context(errors::DatabaseError::InsertFailed)
    }

    async fn update_user(
        &self,
        base: types::users::User,
        update: types::users::UserUpdate,
    ) -> EResult<types::users::User, Self::Error> {
        let update: types::users::PGUserUpdate = update.into();

        let mut conn = self
            .pg_pool
            .get()
            .await
            .into_report()
            .change_context(errors::DatabaseError::PoolClientFailure)?;
        let update_query = diesel::update(types::users::User::table())
            .filter(schema::users::id.eq(base.id))
            .set(update);

        update_query
            .get_result(&mut conn)
            .await
            .into_report()
            .change_context(errors::DatabaseError::UpdateFailed)
    }
}
