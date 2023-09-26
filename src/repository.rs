use crate::{
    constants,
    entities::{prelude::*, *},
    errors::Error,
};
use chrono::Utc;
use sea_orm::entity::prelude::*;
use sea_orm::*;
use std::env;
pub struct Repository {
    db: DatabaseConnection,
}

impl Repository {
    pub async fn new() -> Self {
        let connection_string = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let db = Database::connect(connection_string)
            .await
            .expect("Failed to connect to database");
        Self { db }
    }

    pub async fn insert_app(&self, id: String) {
        let new_app = app::ActiveModel {
            id: ActiveValue::Set(id.clone()),
            appdata: ActiveValue::Set(None),
            last_crawled: ActiveValue::Set(None),
            last_updated: ActiveValue::Set(None),
            last_similar_search: ActiveValue::Set(None),
            priority: ActiveValue::Set(1),
        };
        _ = App::insert(new_app).exec(&self.db).await;
    }

    pub async fn insert_apps(&self, ids: impl IntoIterator<Item = String>) {
        for id in ids {
            self.insert_app(id).await;
        }
    }

    pub async fn insert_developer(&self, id: String) {
        let new_developer = developer::ActiveModel {
            id: ActiveValue::Set(id.clone()),
            last_crawled: ActiveValue::Set(None),
        };
        _ = Developer::insert(new_developer).exec(&self.db).await;
    }

    pub async fn update_app(&self, id: &str, data: JsonValue) -> Result<(), Error> {
        let dev_id = data["developerId"].as_str().map(String::from);
        let mut app: app::ActiveModel = App::find_by_id(id)
            .one(&self.db)
            .await?
            .ok_or_else(|| format!("App not found when updating {}", id))?
            .into();
        app.appdata = ActiveValue::Set(Some(data));
        app.last_updated = ActiveValue::Set(Some(Utc::now().naive_utc()));
        app.update(&self.db).await.expect("Failed to update app");
        match dev_id {
            Some(id) => self.insert_developer(id).await,
            None => (),
        }
        Ok(())
    }

    pub async fn get_category_for_crawl(&self) -> Result<String, Error> {
        let category = Category::find()
            .filter(category::Column::LastCrawled.is_null())
            .one(&self.db)
            .await?
            .ok_or_else(|| "No category left for crawling")?;
        let cat_id = category.id.clone();
        let mut category: category::ActiveModel = category.into();
        category.last_crawled = ActiveValue::Set(Some(Utc::now().naive_utc()));
        category.update(&self.db).await?;
        Ok(cat_id)
    }

    pub async fn get_app_for_similar_search(&self) -> Result<String, Error> {
        let app = App::find()
            .filter(app::Column::LastSimilarSearch.is_null())
            .one(&self.db)
            .await?
            .ok_or_else(|| "No app left for similar search")?;
        let app_id = app.id.clone();
        let mut app: app::ActiveModel = app.into();
        app.last_similar_search = ActiveValue::Set(Some(Utc::now().naive_utc()));
        app.update(&self.db).await?;
        Ok(app_id)
    }

    pub async fn get_app_for_crawl(&self) -> Result<String, Error> {
        let app = App::find()
            .filter(app::Column::LastCrawled.is_null())
            .one(&self.db)
            .await?
            .ok_or_else(|| "No app left for crawling")?;
        let app_id = app.id.clone();
        let mut app: app::ActiveModel = app.into();
        app.last_crawled = ActiveValue::Set(Some(Utc::now().naive_utc()));
        app.update(&self.db).await?;
        Ok(app_id)
    }

    pub async fn get_developer_for_crawl(&self) -> Result<String, Error> {
        let developer = Developer::find()
            .filter(developer::Column::LastCrawled.is_null())
            .one(&self.db)
            .await?
            .ok_or_else(|| "No developer left for crawling")?;
        let dev_id = developer.id.clone();
        let mut developer: developer::ActiveModel = developer.into();
        developer.last_crawled = ActiveValue::Set(Some(Utc::now().naive_utc()));
        developer.update(&self.db).await?;
        Ok(dev_id)
    }

    pub async fn populate_constants(&self) {
        for category in constants::CATEGORIES {
            if category.starts_with("GAME") {
                let new_category = category::ActiveModel {
                    id: ActiveValue::Set(category.to_string()),
                    last_crawled: ActiveValue::Set(None),
                };
                _ = Category::insert(new_category).exec(&self.db).await;
            }
        }
    }
}
