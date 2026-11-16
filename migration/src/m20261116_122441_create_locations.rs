use sea_orm_migration::{prelude::*, schema::*};

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20261116_122441_create_locations"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table("locations")
                    .if_not_exists()
                    .col(pk_auto("location_id"))
                    .col(string_len("location_name", 255))
                    .col(double("latitude"))
                    .col(double("longitude"))
                    .index(
                        Index::create()
                            .name("idx_locations_lat_long")
                            .col("latitude")
                            .col("longitude")
                            .unique(),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table("locations").if_exists().to_owned())
            .await
    }
}
