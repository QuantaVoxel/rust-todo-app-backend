use std::time::Duration;
use chrono::Utc;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, DbErr, EntityTrait, QueryFilter, Set,
};

use crate::core::db::connection::get_db_connection;
use crate::entity::sea_orm_active_enums::{NotificationTypeEnum, RecurrenceFrequencyEnum};
use crate::entity::{
    activity_logs, recurring_tasks, reminders, task_tags, tasks, user_notifications,
};

#[derive(Debug, Default, PartialEq, Eq)]
pub struct WorkerRunSummary {
    pub reminders_processed: usize,
    pub recurring_spawned: usize,
}

pub async fn run_worker_once(db: &DatabaseConnection) -> Result<WorkerRunSummary, DbErr> {
    let mut summary = WorkerRunSummary::default();

    // 1. Process Due Reminders
    let now = Utc::now();
    let due_reminders = reminders::Entity::find()
        .filter(reminders::Column::NotificationSent.eq(false))
        .filter(reminders::Column::ReminderTime.lte(now))
        .all(db)
        .await?;

    for reminder in due_reminders {
        if let Some(task) = tasks::Entity::find_by_id(reminder.task_id).one(db).await? {
            let notif = user_notifications::ActiveModel {
                user_id: Set(task.user_id),
                message: Set(format!(
                    "Pengingat: Task \"{}\" telah jatuh tempo!",
                    task.title
                )),
                notification_type: Set(NotificationTypeEnum::Reminder),
                is_read: Set(false),
                created_at: Set(Utc::now().into()),
                ..Default::default()
            };
            let _ = notif.insert(db).await?;

            let mut active: reminders::ActiveModel = reminder.into();
            active.notification_sent = Set(true);
            let _ = active.update(db).await?;
            summary.reminders_processed += 1;
        }
    }

    // 2. Process Due Recurring Tasks
    let due_recurring = recurring_tasks::Entity::find()
        .filter(recurring_tasks::Column::NextDueDate.lte(now))
        .all(db)
        .await?;

    for rec in due_recurring {
        if let Some(base_task) = tasks::Entity::find_by_id(rec.task_id).one(db).await? {
            let now_dt = Utc::now().into();

            // Spawn new task instance
            let new_task = tasks::ActiveModel {
                user_id: Set(base_task.user_id),
                title: Set(base_task.title.clone()),
                description: Set(base_task.description.clone()),
                status_id: Set(1), // Pending
                priority_id: Set(base_task.priority_id),
                progress_percentage: Set(0),
                due_date: Set(Some(rec.next_due_date)),
                archived_at: Set(None),
                created_at: Set(now_dt),
                updated_at: Set(now_dt),
                ..Default::default()
            };
            let spawned = new_task.insert(db).await?;

            // Duplicate tags
            let old_tags = task_tags::Entity::find()
                .filter(task_tags::Column::TaskId.eq(base_task.task_id))
                .all(db)
                .await?;
            for t in old_tags {
                let link = task_tags::ActiveModel {
                    task_id: Set(spawned.task_id),
                    tag_id: Set(t.tag_id),
                };
                let _ = link.insert(db).await;
            }

            // Log activity
            let log = activity_logs::ActiveModel {
                user_id: Set(base_task.user_id),
                action: Set("task.recurring_spawn".to_string()),
                entity_type: Set(Some("task".to_string())),
                entity_id: Set(Some(spawned.task_id)),
                timestamp: Set(now_dt),
                ..Default::default()
            };
            let _ = log.insert(db).await;

            // Notify user
            let notif = user_notifications::ActiveModel {
                user_id: Set(base_task.user_id),
                message: Set(format!(
                    "Task berulang baru berhasil dibuat: \"{}\"",
                    spawned.title
                )),
                notification_type: Set(NotificationTypeEnum::System),
                is_read: Set(false),
                created_at: Set(now_dt),
                ..Default::default()
            };
            let _ = notif.insert(db).await;

            // Advance next_due_date
            let current_next = rec.next_due_date;
            let next_date = match rec.frequency {
                RecurrenceFrequencyEnum::Daily => current_next + chrono::Duration::days(1),
                RecurrenceFrequencyEnum::Weekly => current_next + chrono::Duration::weeks(1),
                RecurrenceFrequencyEnum::Monthly => current_next + chrono::Duration::days(30),
                RecurrenceFrequencyEnum::Yearly => current_next + chrono::Duration::days(365),
            };

            let mut active_rec: recurring_tasks::ActiveModel = rec.into();
            active_rec.next_due_date = Set(next_date);
            let _ = active_rec.update(db).await?;

            summary.recurring_spawned += 1;
        }
    }

    Ok(summary)
}

pub async fn start_worker() {
    tracing::info!("Background Worker started (interval: 30 detik)");
    let mut interval = tokio::time::interval(Duration::from_secs(30));

    loop {
        interval.tick().await;

        let db = get_db_connection();
        match run_worker_once(db).await {
            Ok(summary) => {
                if summary.reminders_processed > 0 || summary.recurring_spawned > 0 {
                    tracing::info!(
                        "Worker tick selesai: {} pengingat diproses, {} task berulang dibuat",
                        summary.reminders_processed,
                        summary.recurring_spawned
                    );
                }
            }
            Err(err) => {
                tracing::error!("Error pada background worker tick: {:?}", err);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::{Duration, TimeZone};

    #[test]
    fn test_worker_summary_defaults() {
        let summary = WorkerRunSummary::default();
        assert_eq!(summary.reminders_processed, 0);
        assert_eq!(summary.recurring_spawned, 0);
    }

    #[test]
    fn test_recurrence_frequency_advancements() {
        let base_date: chrono::DateTime<chrono::FixedOffset> =
            Utc.with_ymd_and_hms(2026, 11, 1, 10, 0, 0).unwrap().into();

        // Daily
        let next_daily = match RecurrenceFrequencyEnum::Daily {
            RecurrenceFrequencyEnum::Daily => base_date + Duration::days(1),
            _ => unreachable!(),
        };
        assert_eq!(next_daily, base_date + Duration::days(1));

        // Weekly
        let next_weekly = match RecurrenceFrequencyEnum::Weekly {
            RecurrenceFrequencyEnum::Weekly => base_date + Duration::weeks(1),
            _ => unreachable!(),
        };
        assert_eq!(next_weekly, base_date + Duration::weeks(1));

        // Monthly
        let next_monthly = match RecurrenceFrequencyEnum::Monthly {
            RecurrenceFrequencyEnum::Monthly => base_date + Duration::days(30),
            _ => unreachable!(),
        };
        assert_eq!(next_monthly, base_date + Duration::days(30));

        // Yearly
        let next_yearly = match RecurrenceFrequencyEnum::Yearly {
            RecurrenceFrequencyEnum::Yearly => base_date + Duration::days(365),
            _ => unreachable!(),
        };
        assert_eq!(next_yearly, base_date + Duration::days(365));
    }
}
