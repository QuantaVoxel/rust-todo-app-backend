use sea_orm_migration::{prelude::*, schema::*};

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20261116_122449_create_activity_logs"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table("activity_logs")
                    .if_not_exists()
                    .col(pk_auto("log_id"))
                    .col(integer("user_id"))
                    .col(string_len("action", 255))
                    .col(string_len_null("entity_type", 255))
                    .col(integer_null("entity_id"))
                    .col(timestamp_with_time_zone("timestamp").default(Expr::current_timestamp()))
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_activity_logs_user_id")
                            .from("activity_logs", "user_id")
                            .to("users", "user_id")
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_activity_logs_user_timestamp")
                    .table("activity_logs")
                    .col("user_id")
                    .col("timestamp")
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_activity_logs_entity")
                    .table("activity_logs")
                    .col("entity_type")
                    .col("entity_id")
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table("activity_logs").if_exists().to_owned())
            .await
    }
}
