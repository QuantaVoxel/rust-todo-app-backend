use crate::model::Users;
use sea_orm_migration::{prelude::*, schema::*};

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20261112_073338_create_table_users"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        manager
            .create_table(
                Table::create()
                    .table(Users::Table)
                    .if_not_exists()
                    .col(pk_uuid(Users::Id).not_null())
                    .col(string_uniq(Users::Email).not_null())
                    .col(string(Users::Password).not_null())
                    .col(string(Users::FirstName).not_null())
                    .col(string(Users::LastName).null())
                    .col(string(Users::PhoneNumber).null())
                    .col(timestamp(Users::CreatedAt).default(Expr::current_timestamp()).not_null())
                    .col(timestamp(Users::UpdatedAt).default(Expr::current_timestamp()).not_null())
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        manager
            .drop_table(Table::drop().table(Users::Table).to_owned())
            .await
    }
}
