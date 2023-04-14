use super::storage::State;
use actix_web::{web, HttpResponse};

pub trait ServerBuilder<S: State + 'static> {
    fn build(self, state: S) -> actix_web::Scope;
}

#[derive(Clone)]
pub struct Application<U, A>
where
    U: UserRegistryInterface + Clone,
    A: AnimeRegistryInterface + Clone,
{
    user: U,
    admin: A,
}

impl<U, A> Application<U, A>
where
    U: UserRegistryInterface + Clone,
    A: AnimeRegistryInterface + Clone,
{
    pub fn new(user_interface: U, registry_interface: A) -> Self {
        Self {
            user: user_interface,
            admin: registry_interface,
        }
    }
}

#[async_trait::async_trait]
pub trait UserRegistryInterface {
    type AppState: State + 'static;
    async fn user_create(
        state: web::Data<Self::AppState>,
        json_payload: web::Json<UserCreate>,
    ) -> HttpResponse;
    async fn subscribe_anime(
        state: web::Data<Self::AppState>,
        anime_id: web::Path<String>,
    ) -> HttpResponse;
    async fn delete_user(
        state: web::Data<Self::AppState>,
        user_id: web::Path<String>,
    ) -> HttpResponse;
}

#[derive(serde::Deserialize)]
pub struct UserCreate {
    username: String,
    email: String,
}

#[derive(serde::Deserialize)]
pub struct AnimeList {
    query: Option<String>,
    subscribed: bool,
}

#[async_trait::async_trait]
pub trait AnimeRegistryInterface {
    type AppState: State + 'static;
    async fn add_anime(
        state: web::Data<Self::AppState>,
        json_payload: web::Json<AnimeCreate>,
    ) -> HttpResponse;
    async fn update_anime(
        state: web::Data<Self::AppState>,
        path: web::Path<String>,
        json_payload: web::Json<AnimeUpdate>,
    ) -> HttpResponse;
    async fn delete_anime(
        state: web::Data<Self::AppState>,
        path: web::Path<String>,
    ) -> HttpResponse;
    async fn list_anime(
        state: web::Data<Self::AppState>,
        json_payload: web::Json<AnimeList>,
    ) -> HttpResponse;
}

#[derive(serde::Deserialize)]
pub struct AnimeCreate {
    reference_url: String,
    name: String,
    analyzer: Analyzer,
}

#[derive(serde::Deserialize)]
pub struct AnimeUpdate {
    reference_url: Option<String>,
    name: String,
    analyzer: Option<Analyzer>,
}

#[derive(serde::Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Analyzer {
    NoAnalyzer,
}

impl<
        T: UserRegistryInterface<AppState = S> + Clone + 'static,
        U: AnimeRegistryInterface<AppState = S> + Clone + 'static,
        S: State + 'static,
    > ServerBuilder<S> for Application<T, U>
{
    fn build(self, state: S) -> actix_web::Scope {
        web::scope("/v1")
            .app_data(web::Data::new(state))
            .service(
                web::scope("/user")
                    .service(web::resource("/create").route(web::post().to(T::user_create)))
                    .service(
                        web::resource("/subscribe/{anime_id}")
                            .route(web::get().to(T::subscribe_anime)),
                    )
                    .service(
                        web::resource("/delete/{user_id}").route(web::delete().to(T::delete_user)),
                    ),
            )
            .service(
                web::scope("/anime")
                    .service(
                        web::resource("/")
                            .route(web::post().to(U::add_anime))
                            .route(web::get().to(U::list_anime)),
                    )
                    .service(
                        web::resource("/{anime_id}")
                            .route(web::post().to(U::update_anime))
                            .route(web::delete().to(U::delete_anime)),
                    ),
            )
    }
}

pub mod admin;
pub mod users;
