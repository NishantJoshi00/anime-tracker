use diesel::{associations::HasTable, BoolExpressionMethods, ExpressionMethods, QueryDsl};
use diesel_async::RunQueryDsl;
use error_stack::{IntoReport, ResultExt};
use types::schema;

use crate::{
    errors,
    storage::{types, AnimeUserRelInterface, Storage},
};

#[async_trait::async_trait]
impl AnimeUserRelInterface for Storage {
    type NextLayer = ();
    type Error = errors::DatabaseError;

    async fn find_anime_ids_by_user_id(
        &self,
        id: String,
    ) -> errors::EResult<Vec<String>, Self::Error> {
        let mut conn = self.get_conn().await?;

        let data: Result<Vec<types::user_anime_rel::UserAnimeFollowing>, _> = async {
            types::user_anime_rel::UserAnimeFollowing::table()
                .filter(schema::user_followed_animes::dsl::user_id.eq(id))
                .get_results(&mut conn)
                .await
        }
        .await;

        data.map(|value| value.into_iter().map(|inner| inner.anime_id).collect())
            .into_report()
            .change_context(errors::DatabaseError::FindQueryFailed)
    }
    async fn create_relation(
        &self,
        user_id: String,
        anime_id: String,
    ) -> errors::EResult<types::user_anime_rel::UserAnimeFollowing, Self::Error> {
        let mut conn = self.get_conn().await?;
        let new = types::user_anime_rel::UserAnimeFollowingNew::new(user_id, anime_id);

        let query =
            diesel::insert_into(types::user_anime_rel::UserAnimeFollowing::table()).values(new);

        query
            .get_result(&mut conn)
            .await
            .into_report()
            .change_context(errors::DatabaseError::InsertFailed)
    }
    async fn delete_relation(
        &self,
        user_id: String,
        anime_id: String,
    ) -> errors::EResult<(), Self::Error> {
        let mut conn = self.get_conn().await?;

        let query = diesel::delete(types::user_anime_rel::UserAnimeFollowing::table()).filter(
            schema::user_followed_animes::anime_id
                .eq(anime_id)
                .and(schema::user_followed_animes::user_id.eq(user_id)),
        );

        query
            .execute(&mut conn)
            .await
            .into_report()
            .change_context(errors::DatabaseError::DeleteFailed)?;

        Ok(())
    }
}
