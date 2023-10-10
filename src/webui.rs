use crate::repository::Repository;
use crate::schema::*;
use async_graphql::{self, dataloader::DataLoader};
use async_graphql::{
    http::{playground_source, GraphQLPlaygroundConfig},
    EmptyMutation, EmptySubscription, Schema,
};
use poem::EndpointExt;
use poem::middleware::Cors;
use sea_orm::DatabaseConnection;

use async_graphql_poem::GraphQL;
use poem::{get, handler, listener::TcpListener, web::Html, IntoResponse, Route, Server};

#[derive(Debug, Clone, serde::Serialize)]
struct StatsResponse {
    count: u64,
}

#[handler]
async fn graphql_playground() -> impl IntoResponse {
    Html(playground_source(GraphQLPlaygroundConfig::new("/")))
}

pub struct OrmDataloader {
    pub db: DatabaseConnection,
}

pub async fn run_web_ui(repo: Repository) -> std::io::Result<()> {
    let orm_dataloader: DataLoader<OrmDataloader> = DataLoader::new(
        OrmDataloader {
            db: repo.db.clone(),
        },
        tokio::spawn,
    );
    let schema = Schema::build(QueryRoot, EmptyMutation, EmptySubscription)
        .data(repo.db)
        .data(orm_dataloader)
        .finish();
    let app = Route::new()
        .at("/", get(graphql_playground).post(GraphQL::new(schema)))
        .with(Cors::new());
    Server::new(TcpListener::bind(":::8080")).run(app).await
}
