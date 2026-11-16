use sea_orm_migration::{prelude::*, schema::*};

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20261116_122448_create_user_notifications"
    }
}

#[derive(DeriveIden)]
enum NotificationTypeEnum {
    #[sea_orm(iden = "notification_type_enum")]
    Enum,
    #[sea_orm(iden = "reminder")]
    Reminder,
    #[sea_orm(iden = "comment")]
    Comment,
    #[sea_orm(iden = "share")]
    Share,
    #[sea_orm(iden = "system")]
    System,
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table("user_notifications")
                    .if_not_exists()
                    .col(pk_auto("notification_id"))
                    .col(integer("user_id"))
                    .col(text("message"))
                    .col(enumeration(
                        "notification_type",
                        NotificationTypeEnum::Enum,
                        [
                            NotificationTypeEnum::Reminder,
                            NotificationTypeEnum::Comment,
                            NotificationTypeEnum::Share,
                            NotificationTypeEnum::System,
                        ],
                    ))
                    .col(boolean("is_read").default(false))
                    .col(timestamp_with_time_zone("created_at").default(Expr::current_timestamp()))
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_user_notifications_user_id")
                            .from("user_notifications", "user_id")
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
                    .name("idx_user_notifications_user_read")
                    .table("user_notifications")
                    .col("user_id")
                    .col("is_read")
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table("user_notifications").if_exists().to_owned())
            .await
    }
}
