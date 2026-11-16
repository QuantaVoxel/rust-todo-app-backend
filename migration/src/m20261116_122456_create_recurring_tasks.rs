use sea_orm_migration::{prelude::*, schema::*};

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20261116_122456_create_recurring_tasks"
    }
}

#[derive(DeriveIden)]
enum RecurrenceFrequencyEnum {
    #[sea_orm(iden = "recurrence_frequency_enum")]
    Enum,
    #[sea_orm(iden = "daily")]
    Daily,
    #[sea_orm(iden = "weekly")]
    Weekly,
    #[sea_orm(iden = "monthly")]
    Monthly,
    #[sea_orm(iden = "yearly")]
    Yearly,
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table("recurring_tasks")
                    .if_not_exists()
                    .col(pk_auto("recurring_id"))
                    .col(integer("task_id").unique_key())
                    .col(enumeration(
                        "frequency",
                        RecurrenceFrequencyEnum::Enum,
                        [
                            RecurrenceFrequencyEnum::Daily,
                            RecurrenceFrequencyEnum::Weekly,
                            RecurrenceFrequencyEnum::Monthly,
                            RecurrenceFrequencyEnum::Yearly,
                        ],
                    ))
                    .col(timestamp_with_time_zone("next_due_date"))
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_recurring_tasks_task_id")
                            .from("recurring_tasks", "task_id")
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
            .drop_table(Table::drop().table("recurring_tasks").if_exists().to_owned())
            .await
    }
}
