use sea_orm_migration::{prelude::*, schema::*};

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20261116_122501_create_task_tags"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table("task_tags")
                    .if_not_exists()
                    .col(integer("task_id"))
                    .col(integer("tag_id"))
                    .primary_key(Index::create().col("task_id").col("tag_id"))
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_task_tags_task_id")
                            .from("task_tags", "task_id")
                            .to("tasks", "task_id")
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_task_tags_tag_id")
                            .from("task_tags", "tag_id")
                            .to("tags", "tag_id")
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table("task_tags").if_exists().to_owned())
            .await
    }
}
