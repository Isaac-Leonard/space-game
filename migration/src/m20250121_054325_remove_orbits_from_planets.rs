use loco_rs::schema::*;
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, m: &SchemaManager) -> Result<(), DbErr> {
        remove_column(m, "planets", "orbits").await
    }

    async fn down(&self, m: &SchemaManager) -> Result<(), DbErr> {
        Ok(())
    }
}
