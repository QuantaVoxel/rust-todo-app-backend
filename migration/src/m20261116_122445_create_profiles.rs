use sea_orm_migration::{prelude::*, schema::*};

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20261116_122445_create_profiles"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table("profiles")
                    .if_not_exists()
                    .col(pk_auto("profile_id"))
                    .col(integer("user_id").unique_key())
                    .col(string_len_null("first_name", 255))
                    .col(string_len_null("last_name", 255))
                    .col(string_len_null("avatar_url", 255))
                    .col(string_len_null("phone_number", 20))
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_profiles_user_id")
                            .from("profiles", "user_id")
                            .to("users", "user_id")
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table("profiles").if_exists().to_owned())
            .await
    }
}
