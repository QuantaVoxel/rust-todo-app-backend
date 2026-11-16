use sea_orm_migration::prelude::{sea_query::extension::postgres::Type, *};
use sea_orm_migration::sea_orm::{ConnectionTrait, DbBackend};

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20261116_122436_create_enums"
    }
}

#[derive(DeriveIden)]
pub enum VoteTypeEnum {
    #[sea_orm(iden = "vote_type_enum")]
    Enum,
    #[sea_orm(iden = "up")]
    Up,
    #[sea_orm(iden = "down")]
    Down,
}

#[derive(DeriveIden)]
pub enum RecurrenceFrequencyEnum {
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

#[derive(DeriveIden)]
pub enum SharePermissionEnum {
    #[sea_orm(iden = "share_permission_enum")]
    Enum,
    #[sea_orm(iden = "view")]
    View,
    #[sea_orm(iden = "comment")]
    Comment,
    #[sea_orm(iden = "edit")]
    Edit,
}

#[derive(DeriveIden)]
pub enum ConnectionStatusEnum {
    #[sea_orm(iden = "connection_status_enum")]
    Enum,
    #[sea_orm(iden = "pending")]
    Pending,
    #[sea_orm(iden = "accepted")]
    Accepted,
    #[sea_orm(iden = "blocked")]
    Blocked,
}

#[derive(DeriveIden)]
pub enum NotificationTypeEnum {
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
        let db = manager.get_connection();
        if db.get_database_backend() == DbBackend::Postgres {
            manager
                .create_type(
                    Type::create()
                        .as_enum(VoteTypeEnum::Enum)
                        .values([VoteTypeEnum::Up, VoteTypeEnum::Down])
                        .to_owned(),
                )
                .await?;

            manager
                .create_type(
                    Type::create()
                        .as_enum(RecurrenceFrequencyEnum::Enum)
                        .values([
                            RecurrenceFrequencyEnum::Daily,
                            RecurrenceFrequencyEnum::Weekly,
                            RecurrenceFrequencyEnum::Monthly,
                            RecurrenceFrequencyEnum::Yearly,
                        ])
                        .to_owned(),
                )
                .await?;

            manager
                .create_type(
                    Type::create()
                        .as_enum(SharePermissionEnum::Enum)
                        .values([
                            SharePermissionEnum::View,
                            SharePermissionEnum::Comment,
                            SharePermissionEnum::Edit,
                        ])
                        .to_owned(),
                )
                .await?;

            manager
                .create_type(
                    Type::create()
                        .as_enum(ConnectionStatusEnum::Enum)
                        .values([
                            ConnectionStatusEnum::Pending,
                            ConnectionStatusEnum::Accepted,
                            ConnectionStatusEnum::Blocked,
                        ])
                        .to_owned(),
                )
                .await?;

            manager
                .create_type(
                    Type::create()
                        .as_enum(NotificationTypeEnum::Enum)
                        .values([
                            NotificationTypeEnum::Reminder,
                            NotificationTypeEnum::Comment,
                            NotificationTypeEnum::Share,
                            NotificationTypeEnum::System,
                        ])
                        .to_owned(),
                )
                .await?;
        }
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();
        if db.get_database_backend() == DbBackend::Postgres {
            manager
                .drop_type(Type::drop().name(NotificationTypeEnum::Enum).if_exists().cascade().to_owned())
                .await?;
            manager
                .drop_type(Type::drop().name(ConnectionStatusEnum::Enum).if_exists().cascade().to_owned())
                .await?;
            manager
                .drop_type(Type::drop().name(SharePermissionEnum::Enum).if_exists().cascade().to_owned())
                .await?;
            manager
                .drop_type(Type::drop().name(RecurrenceFrequencyEnum::Enum).if_exists().cascade().to_owned())
                .await?;
            manager
                .drop_type(Type::drop().name(VoteTypeEnum::Enum).if_exists().cascade().to_owned())
                .await?;
        }
        Ok(())
    }
}
