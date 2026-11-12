use sea_orm_migration::prelude::*;

#[derive(DeriveIden)]
pub enum Users {
    Table,

    Id,
    Email,
    Password,
    FirstName,
    LastName,
    PhoneNumber,
    CreatedAt,
    UpdatedAt
}

#[derive(DeriveIden)]
pub enum Tasks {
    Table,

    Id,
    TaskName,
    TaskDescription,
    DueDate,
    Priority,
    CreatedAt,
    UpdatedAt,

    UserId,
    CategoryId,
    StatusId
}

#[derive(DeriveIden)]
pub enum TaskPriorityEnum {
    #[sea_orm(iden = "tasks_priority_enum")]
    Table,

    #[sea_orm(iden = "low")]
    Low,
    #[sea_orm(iden = "medium")]
    Medium,
    #[sea_orm(iden = "high")]
    High,
}

#[derive(DeriveIden)]
pub enum TaskStatuses {
    Table,

    Id,
    StatusName
}

#[derive(DeriveIden)]
pub enum TaskCategories {
    Table,

    Id,
    CategoryName,

    UserId
}