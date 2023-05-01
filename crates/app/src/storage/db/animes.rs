use diesel::{associations::HasTable, ExpressionMethods, QueryDsl, TextExpressionMethods};
use diesel_async::RunQueryDsl;
use types::schema;

use crate::{
    errors,
    storage::{self, types, AnimeInterface, Storage},
};
use error_stack::{IntoReport, ResultExt};

#[async_trait::async_trait]
impl AnimeInterface for Storage {
    type NextLayer = ();

    type Error = errors::DatabaseError;

    async fn find_anime_by_id(
        &self,
        id: String,
    ) -> errors::EResult<storage::types::animes::Anime, Self::Error> {
        let mut conn = self.get_conn().await?;

        types::animes::Anime::table()
            .filter(schema::animes::dsl::anime_id.eq(id))
            .get_result(&mut conn)
            .await
            .into_report()
            .change_context(errors::DatabaseError::FindQueryFailed)
    }

    async fn find_anime_like_name(
        &self,
        name: String,
    ) -> errors::EResult<storage::types::animes::Anime, Self::Error> {
        let mut conn = self.get_conn().await?;

        types::animes::Anime::table()
            .filter(schema::animes::dsl::name.like(name))
            .get_result(&mut conn)
            .await
            .into_report()
            .change_context(errors::DatabaseError::FindQueryFailed)
    }

    async fn add_new_anime(
        &self,
        new: storage::types::animes::AnimeNew,
    ) -> errors::EResult<storage::types::animes::Anime, Self::Error> {
        let mut conn = self.get_conn().await?;

        let query = diesel::insert_into(types::animes::Anime::table()).values(new);

        query
            .get_result(&mut conn)
            .await
            .into_report()
            .change_context(errors::DatabaseError::InsertFailed)
    }

    async fn update_anime(
        &self,
        base: storage::types::animes::Anime,
        update: storage::types::animes::AnimeUpdate,
    ) -> errors::EResult<storage::types::animes::Anime, Self::Error> {
        let update: types::animes::PGAnimeUpdate = update.into();

        let mut conn = self.get_conn().await?;
        let update_query = diesel::update(types::animes::Anime::table())
            .filter(schema::animes::id.eq(base.id))
            .set(update);

        update_query
            .get_result(&mut conn)
            .await
            .into_report()
            .change_context(errors::DatabaseError::UpdateFailed)
    }
}
