use actix_web::{web, HttpResponse};

use super::{AnimeCreate, AnimeList, AnimeUpdate};

#[derive(Clone)]
pub struct PrimaryAnimeContract;

#[async_trait::async_trait]
impl super::AnimeRegistryInterface for PrimaryAnimeContract {
    type AppState = crate::storage::Storage;

    async fn add_anime(
        state: web::Data<Self::AppState>,
        json_payload: web::Json<AnimeCreate>,
    ) -> HttpResponse {
        HttpResponse::Ok().body("Fuck you!")
    }

    async fn update_anime(
        state: web::Data<Self::AppState>,
        path: web::Path<String>,
        json_payload: web::Json<AnimeUpdate>,
    ) -> HttpResponse {
        HttpResponse::Ok().body("Nope!")
    }
    async fn delete_anime(
        state: web::Data<Self::AppState>,
        path: web::Path<String>,
    ) -> HttpResponse {
        HttpResponse::Ok().body("You suck!")
    }
    async fn list_anime(
        state: web::Data<Self::AppState>,
        json_payload: web::Json<AnimeList>,
    ) -> HttpResponse {
        HttpResponse::Ok().body("pussy")
    }
}
