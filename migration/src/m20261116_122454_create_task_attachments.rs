use sea_orm_migration::{prelude::*, schema::*};

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20261116_122454_create_task_attachments"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table("task_attachments")
                    .if_not_exists()
                    .col(pk_auto("attachment_id"))
                    .col(integer("task_id"))
                    .col(string_len("file_url", 255))
                    .col(string_len_null("file_type", 50))
                    .col(timestamp_with_time_zone("uploaded_at").default(Expr::current_timestamp()))
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_task_attachments_task_id")
                            .from("task_attachments", "task_id")
                            .to("tasks", "task_id")
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table("task_attachments").if_exists().to_owned())
            .await
    }
}
