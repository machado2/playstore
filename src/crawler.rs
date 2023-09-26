use crate::errors::Error;
use crate::google_play_api::{self, GooglePlayApi};
use crate::repository::Repository;
use log::{error, info};
use std::time::Duration;

async fn search_category(repo: &Repository, api: &GooglePlayApi) -> Result<(), Error> {
    let category = repo.get_category_for_crawl().await?;
    let apps = api.get_category(&category).await?;
    repo.insert_apps(apps).await;
    info!("Crawled category {}", category);
    Ok(())
}

async fn search_similar(repo: &Repository, api: &GooglePlayApi) -> Result<(), Error> {
    let app_id = repo.get_app_for_similar_search().await?;
    let apps = api.get_similar(&app_id).await?;
    repo.insert_apps(apps).await;
    info!("Crawled similar apps for {}", app_id);
    Ok(())
}

async fn search_developer(repo: &Repository, api: &GooglePlayApi) -> Result<(), Error> {
    let dev_id = repo.get_developer_for_crawl().await?;
    let ids = api.get_from_developer(&dev_id).await?;
    repo.insert_apps(ids).await;
    info!("Crawled developer {}", dev_id);
    Ok(())
}

async fn update_one_app(repo: &Repository, api: &GooglePlayApi) -> Result<(), Error> {
    let mut app_id = repo.get_app_for_crawl().await;
    if app_id.is_err() {
        search_category(repo, api)
            .await
            .or(search_developer(repo, api).await)
            .or(search_similar(repo, api).await)
            .map_err(|_| "No search left to do")?;
        app_id = repo.get_app_for_crawl().await;
        if app_id.is_err() {
            info!("Search didn't yield any new apps");
            return Ok(());
        }
    }
    match app_id {
        Ok(id) => {
            let data = api.get_app(&id).await?;
            let r = repo.update_app(&id, data).await;
            info!("Crawled app {}", id);
            r
        }
        Err(e) => Err(e),
    }
}

pub async fn update_loop() {
    let api = google_play_api::GooglePlayApi::new();
    let repo = Repository::new().await;
    repo.populate_constants().await;
    let mut interval = tokio::time::interval(Duration::from_secs(5));
    info!("Starting scrape loop");
    loop {
        let _ = interval.tick().await;
        match update_one_app(&repo, &api).await {
            Ok(_) => {}
            Err(e) => {
                error!("Failed to update: {}", e);
            }
        }
    }
}
