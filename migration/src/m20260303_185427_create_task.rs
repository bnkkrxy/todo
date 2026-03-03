use sea_orm_migration::{prelude::*, schema::*};
use super::m20260303_183523_create_category::Category;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {

        manager
            .create_table(
                Table::create()
                    .table(Task::Table)
                    .if_not_exists()
                    .col(pk_auto(Task::Id))
                    .col(string(Task::Title).not_null())
                    .col(text(Task::Description))
                    .col(boolean(Task::IsDone).default(false))
                    .col(integer(Task::CategoryId).not_null())
                        .foreign_key(
                            ForeignKey::create()
                            .name("fk-task-category")
                            .from(Task::Table, Task::CategoryId)
                            .to(Category::Table, Category::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                        )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Task::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Task {
    #[sea_orm(iden = "task")]
    Table,
    Id,
    Title,
    Description,
    IsDone,
    CategoryId,
}
