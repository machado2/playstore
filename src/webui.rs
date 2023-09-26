use crate::repository::Repository;
use actix_web::{get, web, App as ActixApp, HttpResponse, HttpServer, Responder, error::Error};

use log::error;

#[derive(Debug, Clone)]
struct AppState {
    repo: Repository,
}

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[derive(Debug, Clone, serde::Serialize)]
struct StatsResponse {
    count: u64,
}

#[get("/stats")]
async fn stats(data: web::Data<AppState>) -> Result<HttpResponse, Error> {
    let response = data.repo.get_stats().await.map_err(|e| {
        error!("Failed to get stats: {}", e);
        actix_web::error::ErrorInternalServerError(e)
    })?;
    Ok(HttpResponse::Ok().json(response))
}

pub async fn run_web_ui(repo: Repository) -> std::io::Result<()> {
    HttpServer::new(move || {
        let state = AppState { repo: repo.clone() };
        ActixApp::new()
            .app_data(web::Data::new(state))
            .service(hello)
            .service(stats)
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
