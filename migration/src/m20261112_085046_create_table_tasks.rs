use crate::extension::postgres::Type;
use crate::model::*;
use sea_orm_migration::{prelude::*, schema::*};

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20261112_085046_create_table_tasks"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts

        // Status
        manager
            .create_table(
                Table::create()
                    .table(TaskStatuses::Table)
                    .if_not_exists()
                    .col(pk_auto(TaskStatuses::Id).auto_increment().not_null())
                    .col(string_uniq(TaskStatuses::StatusName).not_null())
                    .to_owned(),
            )
            .await?;

        // Category
        manager
            .create_table(
                Table::create()
                    .table(TaskCategories::Table)
                    .if_not_exists()
                    .col(pk_auto(TaskCategories::Id).auto_increment().not_null())
                    .col(string(TaskCategories::CategoryName).not_null())
                    .col(uuid(TaskCategories::UserId).not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-task_categories-user_id")
                            .from(TaskCategories::Table, TaskCategories::UserId)
                            .to(Users::Table, Users::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        // Enum Task Priority
        manager
            .create_type(
                Type::create()
                    .as_enum(TaskPriorityEnum::Table)
                    .values([
                        TaskPriorityEnum::Low,
                        TaskPriorityEnum::Medium,
                        TaskPriorityEnum::High,
                    ])
                    .to_owned(),
            )
            .await?;

        // Task
        manager
            .create_table(
                Table::create()
                    .table(Tasks::Table)
                    .if_not_exists()
                    .col(pk_uuid(Tasks::Id).not_null())
                    .col(string(Tasks::TaskName).not_null())
                    .col(text(Tasks::TaskDescription).null())
                    .col(date(Tasks::DueDate).not_null())
                    .col(
                        ColumnDef::new(Tasks::Priority)
                            .custom(TaskPriorityEnum::Table)
                            .default(TaskPriorityEnum::Medium.to_string()),
                    )
                    .col(
                        timestamp(Users::CreatedAt)
                            .default(Expr::current_timestamp())
                            .not_null(),
                    )
                    .col(
                        timestamp(Users::UpdatedAt)
                            .default(Expr::current_timestamp())
                            .not_null(),
                    )
                    .col(uuid(Tasks::UserId).not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-tasks-user_id")
                            .from(Tasks::Table, Tasks::UserId)
                            .to(Users::Table, Users::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .col(integer(Tasks::CategoryId).not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-tasks-category_id")
                            .from(Tasks::Table, Tasks::CategoryId)
                            .to(TaskCategories::Table, TaskCategories::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .col(integer(Tasks::StatusId).not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-tasks-status-id")
                            .from(Tasks::Table, Tasks::StatusId)
                            .to(TaskStatuses::Table, TaskStatuses::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts

        manager
            .drop_table(Table::drop().table(Tasks::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(TaskCategories::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(TaskStatuses::Table).to_owned())
            .await?;

        manager
            .drop_type(
                Type::drop()
                    .name(TaskPriorityEnum::Table)
                    .if_exists()
                    .to_owned(),
            )
            .await?;

        Ok(())
    }
}
