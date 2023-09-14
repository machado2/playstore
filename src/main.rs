#[macro_use]
extern crate error_chain;

use std::time::Duration;

use dotenvy::dotenv;
use google_play_api::GooglePlayApi;
use repository::Repository;

mod entities;

mod constants;
mod errors;

use errors::Error;
mod google_play_api;
mod repository;

async fn search_category(repo: &Repository, api: &GooglePlayApi) -> Result<(), Error> {
    let category = repo.get_category_for_crawl().await?;
    let apps = api.get_category(&category).await?;
    repo.insert_apps(apps).await;
    Ok(())
}

async fn search_similar(repo: &Repository, api: &GooglePlayApi) -> Result<(), Error> {
    let app_id = repo.get_app_for_similar_search().await?;
    let apps = api.get_similar(&app_id).await?;
    repo.insert_apps(apps).await;
    Ok(())
}

async fn search_developer(repo: &Repository, api: &GooglePlayApi) -> Result<(), Error> {
    let dev_id = repo.get_developer_for_crawl().await?;
    let ids = api.get_from_developer(&dev_id).await?;
    repo.insert_apps(ids).await;
    Ok(())
}

async fn update_one_app(repo: &Repository, api: &GooglePlayApi) -> Result<(), Error> {
    let mut app_id = repo.get_app_for_crawl().await;
    if app_id.is_err() {
        let searches = search_category(repo, api)
            .await
            .or(search_developer(repo, api).await)
            .or(search_similar(repo, api).await);
        if searches.is_ok() {
            app_id = repo.get_app_for_crawl().await;
        }
    }
    match app_id {
        Ok(id) => {
            let data = api.get_app(&id).await?;
            repo.update_app(id, data).await
        }
        Err(e) => Err(e),
    }
}

async fn update_loop(repo: &Repository, api: &GooglePlayApi) {
    let mut interval = tokio::time::interval(Duration::from_secs(5));
    loop {
        let _ = interval.tick().await;
        match update_one_app(repo, api).await {
            Ok(_) => {}
            Err(e) => {
                println!("Failed to update: {}", e);
            }
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    _ = dotenv();
    let api = google_play_api::GooglePlayApi::new();
    let repo = Repository::new().await;
    repo.populate_constants().await;
    update_loop(&repo, &api).await;
    Ok(())
}
