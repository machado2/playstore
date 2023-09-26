#[macro_use]
extern crate error_chain;
use crawler::update_loop;
use dotenvy::dotenv;
use webui::run_web_ui;
mod constants;
mod crawler;
mod entities;
mod errors;
mod google_play_api;
mod repository;
mod webui;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    _ = dotenv();
    env_logger::init();
    tokio::spawn(update_loop());
    run_web_ui().await
}
