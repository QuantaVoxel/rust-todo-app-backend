use sea_orm_migration::{prelude::*, schema::*};

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20261116_122504_create_task_history"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table("task_history")
                    .if_not_exists()
                    .col(pk_auto("history_id"))
                    .col(integer("task_id"))
                    .col(integer_null("old_status_id"))
                    .col(integer("new_status_id"))
                    .col(timestamp_with_time_zone("changed_at").default(Expr::current_timestamp()))
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_task_history_task_id")
                            .from("task_history", "task_id")
                            .to("tasks", "task_id")
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_task_history_old_status_id")
                            .from("task_history", "old_status_id")
                            .to("task_statuses", "status_id")
                            .on_delete(ForeignKeyAction::SetNull)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_task_history_new_status_id")
                            .from("task_history", "new_status_id")
                            .to("task_statuses", "status_id")
                            .on_delete(ForeignKeyAction::Restrict)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table("task_history").if_exists().to_owned())
            .await
    }
}
