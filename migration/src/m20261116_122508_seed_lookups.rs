use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let insert_statuses = Query::insert()
            .into_table(TaskStatuses::Table)
            .columns([TaskStatuses::Name])
            .values_panic(["pending".into()])
            .values_panic(["in_progress".into()])
            .values_panic(["completed".into()])
            .values_panic(["cancelled".into()])
            .to_owned();

        manager.exec_stmt(insert_statuses).await?;

        let insert_priorities = Query::insert()
            .into_table(TaskPriorities::Table)
            .columns([TaskPriorities::Name, TaskPriorities::RankOrder])
            .values_panic(["low".into(), 1.into()])
            .values_panic(["medium".into(), 2.into()])
            .values_panic(["high".into(), 3.into()])
            .values_panic(["urgent".into(), 4.into()])
            .to_owned();

        manager.exec_stmt(insert_priorities).await?;

        let insert_apps = Query::insert()
            .into_table(ExternalApps::Table)
            .columns([ExternalApps::Name])
            .values_panic(["Trello".into()])
            .values_panic(["Jira".into()])
            .values_panic(["GitHub".into()])
            .values_panic(["Slack".into()])
            .to_owned();

        manager.exec_stmt(insert_apps).await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let delete_statuses = Query::delete()
            .from_table(TaskStatuses::Table)
            .to_owned();
        manager.exec_stmt(delete_statuses).await?;

        let delete_priorities = Query::delete()
            .from_table(TaskPriorities::Table)
            .to_owned();
        manager.exec_stmt(delete_priorities).await?;

        let delete_apps = Query::delete()
            .from_table(ExternalApps::Table)
            .to_owned();
        manager.exec_stmt(delete_apps).await?;

        Ok(())
    }
}

#[derive(DeriveIden)]
enum TaskStatuses {
    Table,
    Name,
}

#[derive(DeriveIden)]
enum TaskPriorities {
    Table,
    Name,
    RankOrder,
}

#[derive(DeriveIden)]
enum ExternalApps {
    Table,
    Name,
}
