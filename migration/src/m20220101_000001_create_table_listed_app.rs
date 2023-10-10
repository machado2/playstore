use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(ListedApp::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(ListedApp::Id)
                            .string()
                            .not_null()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(ListedApp::LastUpdated)
                            .timestamp()
                            .not_null()
                            .default(SimpleExpr::Keyword(Keyword::CurrentTimestamp)),
                    )
                    .col(ColumnDef::new(ListedApp::Title).string().not_null())
                    .col(ColumnDef::new(ListedApp::Score).float().not_null())
                    .col(ColumnDef::new(ListedApp::MinInstalls).integer().not_null())
                    .col(ColumnDef::new(ListedApp::GenreId).string().not_null())
                    .col(ColumnDef::new(ListedApp::Free).boolean().not_null())
                    .col(ColumnDef::new(ListedApp::Details).json_binary().not_null())
                    .to_owned(),
            )
            .await?;

        let sql = r#"
            insert into listed_app
            select id,
                now(),
                appdata->>'title',
                (appdata->>'score')::numeric,
                (appdata->>'minInstalls')::numeric,
                appdata->>'genreId' as genreId,
                appdata->>'free' = 'true',
                appdata
            from app
            where 	appdata->>'offersIAP' = 'false'
                and appdata->>'adSupported' = 'false'
                and appdata->>'genreId' like 'GAME%'
                and (appdata->>'score')::numeric >= 3
        "#;

        manager.get_connection().execute_unprepared(sql).await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(ListedApp::Table).to_owned())
            .await?;

        Ok(())
    }
}

#[derive(DeriveIden)]
enum ListedApp {
    Table,
    Id,
    LastUpdated,
    Title,
    Score,
    MinInstalls,
    GenreId,
    Free,
    Details,
}
