use sea_orm_migration::{prelude::*, schema::*};

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20261116_122447_create_user_connections"
    }
}

#[derive(DeriveIden)]
enum ConnectionStatusEnum {
    #[sea_orm(iden = "connection_status_enum")]
    Enum,
    #[sea_orm(iden = "pending")]
    Pending,
    #[sea_orm(iden = "accepted")]
    Accepted,
    #[sea_orm(iden = "blocked")]
    Blocked,
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table("user_connections")
                    .if_not_exists()
                    .col(pk_auto("connection_id"))
                    .col(integer("user_id_1"))
                    .col(integer("user_id_2"))
                    .col(
                        enumeration(
                            "status",
                            ConnectionStatusEnum::Enum,
                            [
                                ConnectionStatusEnum::Pending,
                                ConnectionStatusEnum::Accepted,
                                ConnectionStatusEnum::Blocked,
                            ],
                        )
                        .default("pending"),
                    )
                    .col(timestamp_with_time_zone("created_at").default(Expr::current_timestamp()))
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_user_connections_user_id_1")
                            .from("user_connections", "user_id_1")
                            .to("users", "user_id")
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_user_connections_user_id_2")
                            .from("user_connections", "user_id_2")
                            .to("users", "user_id")
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .index(
                        Index::create()
                            .name("idx_user_connections_pair")
                            .col("user_id_1")
                            .col("user_id_2")
                            .unique(),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table("user_connections").if_exists().to_owned())
            .await
    }
}
