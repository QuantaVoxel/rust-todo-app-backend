use sea_orm_migration::{prelude::*, schema::*};

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20261116_122446_create_user_settings"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table("user_settings")
                    .if_not_exists()
                    .col(pk_auto("setting_id"))
                    .col(integer("user_id"))
                    .col(string_len("setting_key", 255))
                    .col(string_len_null("setting_value", 255))
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_user_settings_user_id")
                            .from("user_settings", "user_id")
                            .to("users", "user_id")
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .index(
                        Index::create()
                            .name("idx_user_settings_user_id_key")
                            .col("user_id")
                            .col("setting_key")
                            .unique(),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table("user_settings").if_exists().to_owned())
            .await
    }
}
