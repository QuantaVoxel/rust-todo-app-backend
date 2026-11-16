use sea_orm_migration::{prelude::*, schema::*};

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20261116_122453_create_task_comments"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table("task_comments")
                    .if_not_exists()
                    .col(pk_auto("comment_id"))
                    .col(integer("task_id"))
                    .col(integer("user_id"))
                    .col(text("comment"))
                    .col(timestamp_with_time_zone("created_at").default(Expr::current_timestamp()))
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_task_comments_task_id")
                            .from("task_comments", "task_id")
                            .to("tasks", "task_id")
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_task_comments_user_id")
                            .from("task_comments", "user_id")
                            .to("users", "user_id")
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table("task_comments").if_exists().to_owned())
            .await
    }
}
