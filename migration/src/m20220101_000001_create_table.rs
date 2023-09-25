use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(App::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(App::Id)
                        .string()
                        .not_null()
                        .primary_key()
                    )
                    .col(ColumnDef::new(App::Data).json())
                    .col(ColumnDef::new(App::LastCrawled).timestamp())
                    .col(ColumnDef::new(App::LastUpdated).timestamp())
                    .col(ColumnDef::new(App::LastSimilarSearch).timestamp())
                    .col(ColumnDef::new(App::Priority)
                        .integer()
                        .default(0)
                        .not_null()
                    )
                    .to_owned()
            )
            .await?;

        // do the same for other models
        // ...

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(App::Table).to_owned())
            .await?;

        // do the same for other models
        // ...

        Ok(())
    }
}

#[derive(DeriveIden)]
enum App {
    Table,
    Id,
    Data,
    LastCrawled,
    LastUpdated,
    LastSimilarSearch,
    Priority,
}

// Add similar enum definitions for your `developer` and `category` models also
