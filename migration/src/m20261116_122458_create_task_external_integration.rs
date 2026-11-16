use sea_orm_migration::{prelude::*, schema::*};

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20261116_122458_create_task_external_integration"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table("task_external_integration")
                    .if_not_exists()
                    .col(pk_auto("integration_id"))
                    .col(integer("task_id"))
                    .col(integer("app_id"))
                    .col(string_len("external_app_task_id", 255))
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_task_external_integration_task_id")
                            .from("task_external_integration", "task_id")
                            .to("tasks", "task_id")
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_task_external_integration_app_id")
                            .from("task_external_integration", "app_id")
                            .to("external_apps", "app_id")
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .index(
                        Index::create()
                            .name("idx_external_integration_app_task")
                            .col("app_id")
                            .col("external_app_task_id")
                            .unique(),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table("task_external_integration").if_exists().to_owned())
            .await
    }
}
