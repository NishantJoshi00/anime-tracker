use actix_web::{web, HttpResponse};

use super::UserCreate;

#[derive(Clone)]
pub struct PrimaryUserContract;

#[async_trait::async_trait]
impl super::UserRegistryInterface for PrimaryUserContract {
    type AppState = crate::storage::Storage;

    async fn user_create(
        state: web::Data<Self::AppState>,
        json_payload: web::Json<UserCreate>,
    ) -> HttpResponse {
        HttpResponse::Ok().body("Bitch!")
    }
    async fn subscribe_anime(
        state: web::Data<Self::AppState>,
        anime_id: web::Path<String>,
    ) -> HttpResponse {
        HttpResponse::Ok().body("Hehe!")
    }
    async fn delete_user(
        state: web::Data<Self::AppState>,
        user_id: web::Path<String>,
    ) -> HttpResponse {
        HttpResponse::Ok().body("Sus!")
    }
}
