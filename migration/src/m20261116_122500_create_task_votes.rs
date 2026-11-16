use sea_orm_migration::{prelude::*, schema::*};

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20261116_122500_create_task_votes"
    }
}

#[derive(DeriveIden)]
enum VoteTypeEnum {
    #[sea_orm(iden = "vote_type_enum")]
    Enum,
    #[sea_orm(iden = "up")]
    Up,
    #[sea_orm(iden = "down")]
    Down,
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table("task_votes")
                    .if_not_exists()
                    .col(pk_auto("vote_id"))
                    .col(integer("task_id"))
                    .col(integer("user_id"))
                    .col(enumeration(
                        "vote_type",
                        VoteTypeEnum::Enum,
                        [VoteTypeEnum::Up, VoteTypeEnum::Down],
                    ))
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_task_votes_task_id")
                            .from("task_votes", "task_id")
                            .to("tasks", "task_id")
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_task_votes_user_id")
                            .from("task_votes", "user_id")
                            .to("users", "user_id")
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .index(
                        Index::create()
                            .name("idx_task_votes_task_user")
                            .col("task_id")
                            .col("user_id")
                            .unique(),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table("task_votes").if_exists().to_owned())
            .await
    }
}
