use loco_rs::schema::*;
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, m: &SchemaManager) -> Result<(), DbErr> {
        let table_name = "objects";
        create_table(m, table_name, &[], &[]).await?;

        let nz_ref_name = "contained_by";
        let bk = m.get_database_backend();
        let col = integer_null(Alias::new(nz_ref_name));
        let fk = TableForeignKey::new()
            .name(format!("fk-{table_name}-{nz_ref_name}-to-{table_name}"))
            .from_tbl(Alias::new(table_name))
            .from_col(Alias::new(nz_ref_name))
            .to_tbl(Alias::new(table_name))
            .to_col(Alias::new("id"))
            .on_delete(ForeignKeyAction::Cascade)
            .on_update(ForeignKeyAction::Cascade)
            .to_owned();
        match bk {
            sea_orm::DatabaseBackend::MySql | sea_orm::DatabaseBackend::Postgres => {
                m.alter_table(
                    alter(Alias::new(table_name))
                        .add_column(col.clone()) // XXX fix, table_name_id
                        .add_foreign_key(&fk)
                        .to_owned(),
                )
                .await?;
            }
            sea_orm::DatabaseBackend::Sqlite => {
                m.alter_table(
                    alter(Alias::new(table_name))
                        .add_column(col.clone()) // XXX fix, table_name_id
                        .to_owned(),
                )
                .await?;
            }
        }
        Ok(())
    }

    async fn down(&self, m: &SchemaManager) -> Result<(), DbErr> {
        drop_table(m, "objects").await
    }
}
