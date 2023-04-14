use app::server::{Application, ServerBuilder};

#[actix_web::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let app = Application::new(
        app::server::users::PrimaryUserContract,
        app::server::admin::PrimaryAnimeContract,
    );

    // let scopes = app.build(app::storage::Storage {});

    let server = actix_web::HttpServer::new(move || {
        actix_web::App::new().service(app.clone().build(app::storage::Storage {}))
    })
    .bind(("127.0.0.1", 8080))?
    .workers(7)
    .shutdown_timeout(120)
    .run();

    let _ = server.await;

    Ok(())
}
