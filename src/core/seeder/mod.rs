use bcrypt::{DEFAULT_COST, hash};
use chrono::{Duration, Utc};
use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, PaginatorTrait, QueryFilter, Set};
use serde::Serialize;

use crate::entity::sea_orm_active_enums::{
    ConnectionStatusEnum, NotificationTypeEnum, RecurrenceFrequencyEnum, SharePermissionEnum,
    VoteTypeEnum,
};
use crate::entity::{
    activity_logs, locations, notes, profiles, recurring_tasks, reminders, subtasks, tags,
    task_categories, task_category_assignment, task_comments, task_label_assignment, task_labels,
    task_location, task_sharing, task_tags, task_votes, tasks, user_connections,
    user_notifications, user_settings, users,
};
use crate::error::api_error::ApiError;

pub const DEMO_USERNAME: &str = "demo_user";
pub const DEMO_EMAIL: &str = "demo@todoapp.local";
pub const DEMO_PASSWORD: &str = "Password123!";

pub const COLLAB_USERNAME: &str = "collab_user";
pub const COLLAB_EMAIL: &str = "collab@todoapp.local";
pub const COLLAB_PASSWORD: &str = "Password123!";

#[derive(Debug, Clone, Serialize)]
pub struct DemoAccountInfo {
    pub user_id: i32,
    pub username: String,
    pub email: String,
    pub password: &'static str,
}

#[derive(Debug, Clone, Serialize, Default)]
pub struct SeededCounts {
    pub tasks: u64,
    pub subtasks: u64,
    pub tags: u64,
    pub categories: u64,
    pub labels: u64,
    pub locations: u64,
    pub notifications: u64,
}

#[derive(Debug, Clone, Serialize)]
pub struct DemoSeedSummary {
    pub already_seeded: bool,
    pub message: String,
    pub demo_account: DemoAccountInfo,
    pub collaborator_account: DemoAccountInfo,
    pub counts: SeededCounts,
}

async fn ensure_tag(db: &DatabaseConnection, name: &str) -> Result<tags::Model, ApiError> {
    if let Some(existing) = tags::Entity::find()
        .filter(tags::Column::Name.eq(name))
        .one(db)
        .await?
    {
        return Ok(existing);
    }
    let active = tags::ActiveModel {
        name: Set(name.to_string()),
        ..Default::default()
    };
    Ok(active.insert(db).await?)
}

async fn ensure_category(
    db: &DatabaseConnection,
    name: &str,
    description: &str,
) -> Result<task_categories::Model, ApiError> {
    if let Some(existing) = task_categories::Entity::find()
        .filter(task_categories::Column::Name.eq(name))
        .one(db)
        .await?
    {
        return Ok(existing);
    }
    let active = task_categories::ActiveModel {
        name: Set(name.to_string()),
        description: Set(Some(description.to_string())),
        ..Default::default()
    };
    Ok(active.insert(db).await?)
}

async fn ensure_label(db: &DatabaseConnection, name: &str) -> Result<task_labels::Model, ApiError> {
    if let Some(existing) = task_labels::Entity::find()
        .filter(task_labels::Column::Name.eq(name))
        .one(db)
        .await?
    {
        return Ok(existing);
    }
    let active = task_labels::ActiveModel {
        name: Set(name.to_string()),
        ..Default::default()
    };
    Ok(active.insert(db).await?)
}

async fn compute_counts(db: &DatabaseConnection, demo_user_id: i32) -> Result<SeededCounts, ApiError> {
    let tasks = tasks::Entity::find()
        .filter(tasks::Column::UserId.eq(demo_user_id))
        .count(db)
        .await?;
    let subtasks = subtasks::Entity::find().count(db).await?;
    let tags = tags::Entity::find().count(db).await?;
    let categories = task_categories::Entity::find().count(db).await?;
    let labels = task_labels::Entity::find().count(db).await?;
    let locations = locations::Entity::find().count(db).await?;
    let notifications = user_notifications::Entity::find()
        .filter(user_notifications::Column::UserId.eq(demo_user_id))
        .count(db)
        .await?;

    Ok(SeededCounts {
        tasks,
        subtasks,
        tags,
        categories,
        labels,
        locations,
        notifications,
    })
}

pub async fn seed_demo_data(db: &DatabaseConnection) -> Result<DemoSeedSummary, ApiError> {
    // Check if demo_user already exists (idempotent check)
    if let Some(existing_demo) = users::Entity::find()
        .filter(users::Column::Username.eq(DEMO_USERNAME))
        .one(db)
        .await?
    {
        let existing_collab = users::Entity::find()
            .filter(users::Column::Username.eq(COLLAB_USERNAME))
            .one(db)
            .await?;

        let collab_id = existing_collab.as_ref().map(|u| u.user_id).unwrap_or(0);
        let counts = compute_counts(db, existing_demo.user_id).await?;

        return Ok(DemoSeedSummary {
            already_seeded: true,
            message: "Data demo sudah tersedia (idempotent skip)".to_string(),
            demo_account: DemoAccountInfo {
                user_id: existing_demo.user_id,
                username: existing_demo.username,
                email: existing_demo.email,
                password: DEMO_PASSWORD,
            },
            collaborator_account: DemoAccountInfo {
                user_id: collab_id,
                username: COLLAB_USERNAME.to_string(),
                email: COLLAB_EMAIL.to_string(),
                password: COLLAB_PASSWORD,
            },
            counts,
        });
    }

    let now: sea_orm::entity::prelude::DateTimeWithTimeZone = Utc::now().into();
    let password_hash = hash(DEMO_PASSWORD, DEFAULT_COST)
        .map_err(|e| ApiError::Internal(format!("Gagal hash password demo: {e}")))?;

    // 1. Create Demo User & Collaborator User
    let demo_user = (users::ActiveModel {
        username: Set(DEMO_USERNAME.to_string()),
        email: Set(DEMO_EMAIL.to_string()),
        password_hash: Set(password_hash.clone()),
        created_at: Set(now),
        updated_at: Set(now),
        ..Default::default()
    })
    .insert(db)
    .await?;

    let collab_user = match users::Entity::find()
        .filter(users::Column::Username.eq(COLLAB_USERNAME))
        .one(db)
        .await?
    {
        Some(u) => u,
        None => {
            (users::ActiveModel {
                username: Set(COLLAB_USERNAME.to_string()),
                email: Set(COLLAB_EMAIL.to_string()),
                password_hash: Set(password_hash),
                created_at: Set(now),
                updated_at: Set(now),
                ..Default::default()
            })
            .insert(db)
            .await?
        }
    };

    // 2. Profiles & Settings
    let _ = (profiles::ActiveModel {
        user_id: Set(demo_user.user_id),
        first_name: Set(Some("Budi".to_string())),
        last_name: Set(Some("Santoso (Demo)".to_string())),
        avatar_url: Set(Some("https://api.dicebear.com/7.x/avataaars/svg?seed=demo".to_string())),
        phone_number: Set(Some("+6281122334455".to_string())),
        ..Default::default()
    })
    .insert(db)
    .await;

    let _ = (profiles::ActiveModel {
        user_id: Set(collab_user.user_id),
        first_name: Set(Some("Siti".to_string())),
        last_name: Set(Some("Rahma (Collaborator)".to_string())),
        avatar_url: Set(Some("https://api.dicebear.com/7.x/avataaars/svg?seed=collab".to_string())),
        phone_number: Set(Some("+6289988776655".to_string())),
        ..Default::default()
    })
    .insert(db)
    .await;

    for (k, v) in [("theme", "dark"), ("language", "id"), ("realtime_push", "enabled")] {
        let _ = (user_settings::ActiveModel {
            user_id: Set(demo_user.user_id),
            setting_key: Set(k.to_string()),
            setting_value: Set(Some(v.to_string())),
            ..Default::default()
        })
        .insert(db)
        .await;
    }

    // 3. Connection between Demo User and Collaborator
    let (u1, u2) = if demo_user.user_id < collab_user.user_id {
        (demo_user.user_id, collab_user.user_id)
    } else {
        (collab_user.user_id, demo_user.user_id)
    };
    let _ = (user_connections::ActiveModel {
        user_id_1: Set(u1),
        user_id_2: Set(u2),
        status: Set(ConnectionStatusEnum::Accepted),
        created_at: Set(now),
        ..Default::default()
    })
    .insert(db)
    .await;

    // 4. Classifications (Tags, Categories, Labels) & Location
    let tag_backend = ensure_tag(db, "Backend").await?;
    let tag_frontend = ensure_tag(db, "Frontend").await?;
    let tag_devops = ensure_tag(db, "DevOps").await?;

    let cat_eng = ensure_category(db, "Engineering", "Pengembangan perangkat lunak & arsitektur").await?;
    let cat_prod = ensure_category(db, "Product", "Manajemen produk & pengalaman pengguna").await?;

    let lbl_urgent = ensure_label(db, "Urgent").await?;
    let lbl_release = ensure_label(db, "Release-v2").await?;

    let loc = match locations::Entity::find()
        .filter(locations::Column::LocationName.eq("Kantor Pusat Jakarta"))
        .one(db)
        .await?
    {
        Some(l) => l,
        None => {
            (locations::ActiveModel {
                location_name: Set("Kantor Pusat Jakarta".to_string()),
                latitude: Set(-6.2255),
                longitude: Set(106.8095),
                ..Default::default()
            })
            .insert(db)
            .await?
        }
    };

    // 5. Demo Tasks
    let due_soon: sea_orm::entity::prelude::DateTimeWithTimeZone = (Utc::now() + Duration::days(3)).into();
    let due_next_week: sea_orm::entity::prelude::DateTimeWithTimeZone = (Utc::now() + Duration::days(7)).into();

    let task1 = (tasks::ActiveModel {
        user_id: Set(demo_user.user_id),
        title: Set("Integrasi Real-time SSE & WebSocket pada Dashboard".to_string()),
        description: Set(Some(
            "Menghubungkan EventSource dan WebSocket client ke endpoint /api/v1/notifications/stream dan /ws."
                .to_string(),
        )),
        status_id: Set(2),         // In Progress
        priority_id: Set(Some(3)), // High
        progress_percentage: Set(65),
        due_date: Set(Some(due_soon)),
        archived_at: Set(None),
        created_at: Set(now),
        updated_at: Set(now),
        ..Default::default()
    })
    .insert(db)
    .await?;

    let task2 = (tasks::ActiveModel {
        user_id: Set(demo_user.user_id),
        title: Set("Implementasi Graceful Shutdown, Compression & Metrics".to_string()),
        description: Set(Some(
            "Menambahkan kompresi HTTP otomatis, request timeout, dan endpoint /api/v1/metrics."
                .to_string(),
        )),
        status_id: Set(3),         // Completed
        priority_id: Set(Some(4)), // Critical/Urgent
        progress_percentage: Set(100),
        due_date: Set(Some(now)),
        archived_at: Set(None),
        created_at: Set(now),
        updated_at: Set(now),
        ..Default::default()
    })
    .insert(db)
    .await?;

    let task3 = (tasks::ActiveModel {
        user_id: Set(demo_user.user_id),
        title: Set("Review UI/UX Komponen Todo App v2".to_string()),
        description: Set(Some(
            "Validasi desain responsif dan aksesibilitas bersama tim produk.".to_string(),
        )),
        status_id: Set(1),         // Pending
        priority_id: Set(Some(2)), // Medium
        progress_percentage: Set(20),
        due_date: Set(Some(due_next_week)),
        archived_at: Set(None),
        created_at: Set(now),
        updated_at: Set(now),
        ..Default::default()
    })
    .insert(db)
    .await?;

    // 6. Mount Tags, Categories, Labels, and Location
    for (tid, tag_id) in [
        (task1.task_id, tag_backend.tag_id),
        (task1.task_id, tag_frontend.tag_id),
        (task2.task_id, tag_devops.tag_id),
        (task3.task_id, tag_frontend.tag_id),
    ] {
        let _ = (task_tags::ActiveModel {
            task_id: Set(tid),
            tag_id: Set(tag_id),
        })
        .insert(db)
        .await;
    }

    for (tid, cid) in [
        (task1.task_id, cat_eng.category_id),
        (task2.task_id, cat_eng.category_id),
        (task3.task_id, cat_prod.category_id),
    ] {
        let _ = (task_category_assignment::ActiveModel {
            task_id: Set(tid),
            category_id: Set(cid),
        })
        .insert(db)
        .await;
    }

    for (tid, lid) in [
        (task1.task_id, lbl_urgent.label_id),
        (task2.task_id, lbl_release.label_id),
    ] {
        let _ = (task_label_assignment::ActiveModel {
            task_id: Set(tid),
            label_id: Set(lid),
        })
        .insert(db)
        .await;
    }

    let _ = (task_location::ActiveModel {
        task_id: Set(task1.task_id),
        location_id: Set(loc.location_id),
    })
    .insert(db)
    .await;

    // 7. Subtasks, Notes, Comments, Reminders, Recurrence, Sharing, Votes
    for (title, status_id) in [
        ("Setup koneksi EventSource di halaman utama", 3),
        ("Tambahkan toast notifikasi real-time", 2),
    ] {
        let _ = (subtasks::ActiveModel {
            task_id: Set(task1.task_id),
            title: Set(title.to_string()),
            status_id: Set(status_id),
            created_at: Set(now),
            ..Default::default()
        })
        .insert(db)
        .await;
    }

    let _ = (notes::ActiveModel {
        task_id: Set(task1.task_id),
        content: Set("Gunakan query parameter ?token=<jwt> saat menginisialisasi EventSource atau WebSocket di browser.".to_string()),
        created_at: Set(now),
        ..Default::default()
    })
    .insert(db)
    .await;

    let _ = (task_comments::ActiveModel {
        task_id: Set(task1.task_id),
        user_id: Set(collab_user.user_id),
        comment: Set("Endpoint SSE dan WebSocket sudah saya uji di Postman dan berjalan lancar!".to_string()),
        created_at: Set(now),
        ..Default::default()
    })
    .insert(db)
    .await;

    let _ = (reminders::ActiveModel {
        task_id: Set(task1.task_id),
        reminder_time: Set((Utc::now() + Duration::hours(2)).into()),
        notification_sent: Set(false),
        ..Default::default()
    })
    .insert(db)
    .await;

    let _ = (recurring_tasks::ActiveModel {
        task_id: Set(task3.task_id),
        frequency: Set(RecurrenceFrequencyEnum::Weekly),
        next_due_date: Set(due_next_week),
        ..Default::default()
    })
    .insert(db)
    .await;

    let _ = (task_sharing::ActiveModel {
        task_id: Set(task1.task_id),
        user_id: Set(collab_user.user_id),
        permission: Set(SharePermissionEnum::Edit),
    })
    .insert(db)
    .await;

    let _ = (task_votes::ActiveModel {
        task_id: Set(task1.task_id),
        user_id: Set(collab_user.user_id),
        vote_type: Set(VoteTypeEnum::Up),
        ..Default::default()
    })
    .insert(db)
    .await;

    // 8. Notifications & Activity Log
    for (msg, ntype) in [
        (
            "Selamat datang di Todo App v2! Data demo telah siap digunakan.",
            NotificationTypeEnum::System,
        ),
        (
            "Siti Rahma mengomentari task \"Integrasi Real-time SSE & WebSocket pada Dashboard\"",
            NotificationTypeEnum::Comment,
        ),
    ] {
        if let Ok(notif) = (user_notifications::ActiveModel {
            user_id: Set(demo_user.user_id),
            message: Set(msg.to_string()),
            notification_type: Set(ntype),
            is_read: Set(false),
            created_at: Set(now),
            ..Default::default()
        })
        .insert(db)
        .await
        {
            crate::core::realtime::notification_hub().publish_model(&notif);
        }
    }

    let _ = (activity_logs::ActiveModel {
        user_id: Set(demo_user.user_id),
        action: Set("system.seed_demo".to_string()),
        entity_type: Set(Some("task".to_string())),
        entity_id: Set(Some(task1.task_id)),
        timestamp: Set(now),
        ..Default::default()
    })
    .insert(db)
    .await;

    let counts = compute_counts(db, demo_user.user_id).await?;

    Ok(DemoSeedSummary {
        already_seeded: false,
        message: "Data demo berhasil dibuat dan siap digunakan untuk pengujian".to_string(),
        demo_account: DemoAccountInfo {
            user_id: demo_user.user_id,
            username: demo_user.username,
            email: demo_user.email,
            password: DEMO_PASSWORD,
        },
        collaborator_account: DemoAccountInfo {
            user_id: collab_user.user_id,
            username: collab_user.username,
            email: collab_user.email,
            password: COLLAB_PASSWORD,
        },
        counts,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_demo_seed_constants() {
        assert_eq!(DEMO_USERNAME, "demo_user");
        assert_eq!(COLLAB_USERNAME, "collab_user");
        assert!(DEMO_PASSWORD.len() >= 8);
    }
}
