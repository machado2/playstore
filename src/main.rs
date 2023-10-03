#[macro_use]
extern crate error_chain;
use crawler::update_loop;
use dotenvy::dotenv;
use log::info;
use sea_orm::prelude::*;
use webui::run_web_ui;
mod constants;
mod crawler;
mod entities;
mod errors;
mod google_play_api;
mod repository;
mod schema;
mod webui;
use std::env;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    _ = dotenv();
    env_logger::init();
    let repo = repository::Repository::new().await;
    if env::var("NOCRAWL").is_ok() {
        info!("NOCRAWL is set, not crawling");
    } else {
        tokio::spawn(update_loop(repo.clone()));
    }
    run_web_ui(repo).await
}
