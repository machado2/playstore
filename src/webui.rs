use crate::repository::Repository;
use crate::schema::*;
use actix_web;
use async_graphql;
use async_graphql_actix_web;
use env_logger;


use actix_web::{
    error::Error, get, guard, web, App as ActixApp, HttpResponse, HttpServer, Responder, Result,
};
use async_graphql::{http::GraphiQLSource, EmptyMutation, EmptySubscription, Schema};
use async_graphql_actix_web::GraphQL;
use log::error;

#[derive(Debug, Clone)]
struct AppState {
    repo: Repository,
}

#[derive(Debug, Clone, serde::Serialize)]
struct StatsResponse {
    count: u64,
}

async fn stats(data: web::Data<AppState>) -> Result<HttpResponse, Error> {
    let response = data.repo.get_stats().await.map_err(|e| {
        error!("Failed to get stats: {}", e);
        actix_web::error::ErrorInternalServerError(e)
    })?;
    Ok(HttpResponse::Ok().json(response))
}

async fn index_graphiql() -> Result<HttpResponse> {
    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(GraphiQLSource::build().endpoint("/").finish()))
}

pub async fn run_web_ui(repo: Repository) -> std::io::Result<()> {
    HttpServer::new(move || {
        let schema = Schema::build(QueryRoot, EmptyMutation, EmptySubscription)
            .data(repo.db.clone())
            .finish();
        let state = AppState { repo: repo.clone() };
        ActixApp::new()
            .app_data(web::Data::new(state))
            .service(
                web::resource("/")
                    .guard(guard::Post())
                    .to(GraphQL::new(schema)),
            )
            .service(web::resource("/").guard(guard::Get()).to(index_graphiql))
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
