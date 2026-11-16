pub use sea_orm_migration::prelude::*;

mod m20261116_122436_create_enums;
mod m20261116_122437_create_task_statuses;
mod m20261116_122438_create_task_priorities;
mod m20261116_122439_create_external_apps;
mod m20261116_122440_create_users;
mod m20261116_122441_create_locations;
mod m20261116_122442_create_tags;
mod m20261116_122443_create_task_categories;
mod m20261116_122444_create_task_labels;
mod m20261116_122445_create_profiles;
mod m20261116_122446_create_user_settings;
mod m20261116_122447_create_user_connections;
mod m20261116_122448_create_user_notifications;
mod m20261116_122449_create_activity_logs;
mod m20261116_122450_create_tasks;
mod m20261116_122451_create_subtasks;
mod m20261116_122452_create_notes;
mod m20261116_122453_create_task_comments;
mod m20261116_122454_create_task_attachments;
mod m20261116_122455_create_reminders;
mod m20261116_122456_create_recurring_tasks;
mod m20261116_122457_create_task_location;
mod m20261116_122458_create_task_external_integration;
mod m20261116_122459_create_task_sharing;
mod m20261116_122500_create_task_votes;
mod m20261116_122501_create_task_tags;
mod m20261116_122502_create_task_category_assignment;
mod m20261116_122503_create_task_label_assignment;
mod m20261116_122504_create_task_history;
mod m20261116_122505_create_task_priority_history;
mod m20261116_122506_create_task_labels_history;
mod m20261116_122507_create_task_notes_history;
mod m20261116_122508_seed_lookups;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20261116_122436_create_enums::Migration),
            Box::new(m20261116_122437_create_task_statuses::Migration),
            Box::new(m20261116_122438_create_task_priorities::Migration),
            Box::new(m20261116_122439_create_external_apps::Migration),
            Box::new(m20261116_122440_create_users::Migration),
            Box::new(m20261116_122441_create_locations::Migration),
            Box::new(m20261116_122442_create_tags::Migration),
            Box::new(m20261116_122443_create_task_categories::Migration),
            Box::new(m20261116_122444_create_task_labels::Migration),
            Box::new(m20261116_122445_create_profiles::Migration),
            Box::new(m20261116_122446_create_user_settings::Migration),
            Box::new(m20261116_122447_create_user_connections::Migration),
            Box::new(m20261116_122448_create_user_notifications::Migration),
            Box::new(m20261116_122449_create_activity_logs::Migration),
            Box::new(m20261116_122450_create_tasks::Migration),
            Box::new(m20261116_122451_create_subtasks::Migration),
            Box::new(m20261116_122452_create_notes::Migration),
            Box::new(m20261116_122453_create_task_comments::Migration),
            Box::new(m20261116_122454_create_task_attachments::Migration),
            Box::new(m20261116_122455_create_reminders::Migration),
            Box::new(m20261116_122456_create_recurring_tasks::Migration),
            Box::new(m20261116_122457_create_task_location::Migration),
            Box::new(m20261116_122458_create_task_external_integration::Migration),
            Box::new(m20261116_122459_create_task_sharing::Migration),
            Box::new(m20261116_122500_create_task_votes::Migration),
            Box::new(m20261116_122501_create_task_tags::Migration),
            Box::new(m20261116_122502_create_task_category_assignment::Migration),
            Box::new(m20261116_122503_create_task_label_assignment::Migration),
            Box::new(m20261116_122504_create_task_history::Migration),
            Box::new(m20261116_122505_create_task_priority_history::Migration),
            Box::new(m20261116_122506_create_task_labels_history::Migration),
            Box::new(m20261116_122507_create_task_notes_history::Migration),
            Box::new(m20261116_122508_seed_lookups::Migration),
        ]
    }
}
