use sea_orm_migration::{prelude::*, schema::*};

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20261116_122503_create_task_label_assignment"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table("task_label_assignment")
                    .if_not_exists()
                    .col(integer("task_id"))
                    .col(integer("label_id"))
                    .primary_key(Index::create().col("task_id").col("label_id"))
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_task_label_assignment_task_id")
                            .from("task_label_assignment", "task_id")
                            .to("tasks", "task_id")
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_task_label_assignment_label_id")
                            .from("task_label_assignment", "label_id")
                            .to("task_labels", "label_id")
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table("task_label_assignment").if_exists().to_owned())
            .await
    }
}
