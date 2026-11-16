use sea_orm_migration::{prelude::*, schema::*};

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20261116_122505_create_task_priority_history"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table("task_priority_history")
                    .if_not_exists()
                    .col(pk_auto("history_id"))
                    .col(integer("task_id"))
                    .col(integer_null("old_priority_id"))
                    .col(integer_null("new_priority_id"))
                    .col(timestamp_with_time_zone("changed_at").default(Expr::current_timestamp()))
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_task_priority_history_task_id")
                            .from("task_priority_history", "task_id")
                            .to("tasks", "task_id")
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_task_priority_history_old_priority_id")
                            .from("task_priority_history", "old_priority_id")
                            .to("task_priorities", "priority_id")
                            .on_delete(ForeignKeyAction::SetNull)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_task_priority_history_new_priority_id")
                            .from("task_priority_history", "new_priority_id")
                            .to("task_priorities", "priority_id")
                            .on_delete(ForeignKeyAction::SetNull)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table("task_priority_history").if_exists().to_owned())
            .await
    }
}
