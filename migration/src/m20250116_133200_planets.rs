use loco_rs::schema::*;
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, m: &SchemaManager) -> Result<(), DbErr> {
        create_table(
            m,
            "planets",
            &[
                ("x", ColType::Double),
                ("y", ColType::Double),
                ("z", ColType::Double),
                ("mass", ColType::Double),
                ("radius", ColType::Double),
                ("type", ColType::String),
            ],
            &[],
        )
        .await
    }

    async fn down(&self, m: &SchemaManager) -> Result<(), DbErr> {
        drop_table(m, "planets").await
    }
}
