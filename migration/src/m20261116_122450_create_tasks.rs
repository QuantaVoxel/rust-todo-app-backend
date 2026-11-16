use sea_orm_migration::{prelude::*, schema::*};

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20261116_122450_create_tasks"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table("tasks")
                    .if_not_exists()
                    .col(pk_auto("task_id"))
                    .col(integer("user_id"))
                    .col(string_len("title", 255))
                    .col(text_null("description"))
                    .col(integer("status_id"))
                    .col(integer_null("priority_id"))
                    .col(
                        integer("progress_percentage")
                            .default(0)
                            .check(Expr::cust("progress_percentage >= 0 AND progress_percentage <= 100")),
                    )
                    .col(timestamp_with_time_zone_null("due_date"))
                    .col(timestamp_with_time_zone_null("archived_at"))
                    .col(timestamp_with_time_zone("created_at").default(Expr::current_timestamp()))
                    .col(timestamp_with_time_zone("updated_at").default(Expr::current_timestamp()))
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_tasks_user_id")
                            .from("tasks", "user_id")
                            .to("users", "user_id")
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_tasks_status_id")
                            .from("tasks", "status_id")
                            .to("task_statuses", "status_id")
                            .on_delete(ForeignKeyAction::Restrict)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_tasks_priority_id")
                            .from("tasks", "priority_id")
                            .to("task_priorities", "priority_id")
                            .on_delete(ForeignKeyAction::SetNull)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_tasks_user_id")
                    .table("tasks")
                    .col("user_id")
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_tasks_status_id")
                    .table("tasks")
                    .col("status_id")
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_tasks_due_date")
                    .table("tasks")
                    .col("due_date")
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table("tasks").if_exists().to_owned())
            .await
    }
}
