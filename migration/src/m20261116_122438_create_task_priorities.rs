use sea_orm_migration::{prelude::*, schema::*};

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20261116_122438_create_task_priorities"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table("task_priorities")
                    .if_not_exists()
                    .col(pk_auto("priority_id"))
                    .col(string_len_uniq("name", 50))
                    .col(integer("rank_order").unique_key())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table("task_priorities").if_exists().to_owned())
            .await
    }
}
