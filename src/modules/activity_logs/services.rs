use chrono::DateTime;
use sea_orm::{
    ColumnTrait, EntityTrait, PaginatorTrait, QueryFilter, QueryOrder, QuerySelect,
};

use crate::common::response::{PageMeta, PaginatedResponse};
use crate::core::db::connection::try_get_db_connection;
use crate::entity::activity_logs;
use crate::error::api_error::ApiError;
use crate::modules::activity_logs::dtos::*;

pub struct ActivityLogService;

impl ActivityLogService {
    pub async fn list_activity_logs(
        user_id: i32,
        query: ActivityLogQuery,
    ) -> Result<PaginatedResponse<activity_logs::Model>, ApiError> {
        let db = try_get_db_connection()
            .ok_or_else(|| ApiError::Internal("Database belum terhubung".into()))?;

        let mut select = activity_logs::Entity::find()
            .filter(activity_logs::Column::UserId.eq(user_id));

        if let Some(ref et) = query.entity_type {
            select = select.filter(activity_logs::Column::EntityType.eq(et));
        }

        if let Some(eid) = query.entity_id {
            select = select.filter(activity_logs::Column::EntityId.eq(eid));
        }

        if let Some(ref act) = query.action {
            select = select.filter(activity_logs::Column::Action.eq(act));
        }

        if let Some(ref from_str) = query.from {
            if let Ok(from_dt) = DateTime::parse_from_rfc3339(from_str) {
                select = select.filter(activity_logs::Column::Timestamp.gte(from_dt));
            }
        }

        if let Some(ref to_str) = query.to {
            if let Ok(to_dt) = DateTime::parse_from_rfc3339(to_str) {
                select = select.filter(activity_logs::Column::Timestamp.lte(to_dt));
            }
        }

        select = select.order_by_desc(activity_logs::Column::Timestamp);

        let page = query.page.unwrap_or(1).max(1);
        let per_page = query.per_page.unwrap_or(20).clamp(1, 100);

        let total = select.clone().count(db).await? as u64;
        let total_pages = if total == 0 {
            0
        } else {
            (total + per_page - 1) / per_page
        };

        let offset = (page - 1) * per_page;
        let items = select.offset(offset).limit(per_page).all(db).await?;

        Ok(PaginatedResponse {
            data: items,
            meta: PageMeta {
                page,
                per_page,
                total,
                total_pages,
            },
        })
    }
}
