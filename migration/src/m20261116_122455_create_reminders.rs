use sea_orm_migration::{prelude::*, schema::*};

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20261116_122455_create_reminders"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table("reminders")
                    .if_not_exists()
                    .col(pk_auto("reminder_id"))
                    .col(integer("task_id"))
                    .col(timestamp_with_time_zone("reminder_time"))
                    .col(boolean("notification_sent").default(false))
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_reminders_task_id")
                            .from("reminders", "task_id")
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
            .drop_table(Table::drop().table("reminders").if_exists().to_owned())
            .await
    }
}
