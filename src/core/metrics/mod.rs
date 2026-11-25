use std::sync::LazyLock;
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::time::Instant;
use serde::Serialize;

use crate::core::cache::{cache_hits_total, cache_invalidations_total, cache_misses_total};
use crate::core::db::connection::try_get_db_connection;
use crate::core::realtime::notification_hub;

static START_TIME: LazyLock<Instant> = LazyLock::new(Instant::now);
static SHUTTING_DOWN: AtomicBool = AtomicBool::new(false);
static TOTAL_HTTP_REQUESTS: AtomicU64 = AtomicU64::new(0);
static WORKER_TICKS_TOTAL: AtomicU64 = AtomicU64::new(0);
static WORKER_REMINDERS_PROCESSED_TOTAL: AtomicU64 = AtomicU64::new(0);
static WORKER_RECURRING_SPAWNED_TOTAL: AtomicU64 = AtomicU64::new(0);

pub fn init_start_time() {
    let _ = *START_TIME;
}

pub fn uptime_seconds() -> u64 {
    START_TIME.elapsed().as_secs()
}

pub fn is_shutting_down() -> bool {
    SHUTTING_DOWN.load(Ordering::Relaxed)
}

pub fn set_shutting_down(val: bool) {
    SHUTTING_DOWN.store(val, Ordering::Relaxed);
}

pub fn inc_http_requests() -> u64 {
    TOTAL_HTTP_REQUESTS.fetch_add(1, Ordering::Relaxed) + 1
}

pub fn total_http_requests() -> u64 {
    TOTAL_HTTP_REQUESTS.load(Ordering::Relaxed)
}

pub fn record_worker_tick(reminders_processed: usize, recurring_spawned: usize) {
    WORKER_TICKS_TOTAL.fetch_add(1, Ordering::Relaxed);
    if reminders_processed > 0 {
        WORKER_REMINDERS_PROCESSED_TOTAL.fetch_add(reminders_processed as u64, Ordering::Relaxed);
    }
    if recurring_spawned > 0 {
        WORKER_RECURRING_SPAWNED_TOTAL.fetch_add(recurring_spawned as u64, Ordering::Relaxed);
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct SystemMetricsSnapshot {
    pub uptime_seconds: u64,
    pub shutting_down: bool,
    pub total_http_requests: u64,
    pub active_realtime_connections: usize,
    pub worker_ticks_total: u64,
    pub worker_reminders_processed_total: u64,
    pub worker_recurring_spawned_total: u64,
    pub cache_hits_total: u64,
    pub cache_misses_total: u64,
    pub cache_invalidations_total: u64,
    pub database_connected: bool,
}

pub async fn collect_metrics_snapshot() -> SystemMetricsSnapshot {
    let db_connected = if let Some(db) = try_get_db_connection() {
        db.ping().await.is_ok()
    } else {
        false
    };

    SystemMetricsSnapshot {
        uptime_seconds: uptime_seconds(),
        shutting_down: is_shutting_down(),
        total_http_requests: total_http_requests(),
        active_realtime_connections: notification_hub().active_connections(),
        worker_ticks_total: WORKER_TICKS_TOTAL.load(Ordering::Relaxed),
        worker_reminders_processed_total: WORKER_REMINDERS_PROCESSED_TOTAL.load(Ordering::Relaxed),
        worker_recurring_spawned_total: WORKER_RECURRING_SPAWNED_TOTAL.load(Ordering::Relaxed),
        cache_hits_total: cache_hits_total(),
        cache_misses_total: cache_misses_total(),
        cache_invalidations_total: cache_invalidations_total(),
        database_connected: db_connected,
    }
}

impl SystemMetricsSnapshot {
    pub fn to_prometheus_text(&self) -> String {
        format!(
            "# HELP todo_app_uptime_seconds Server uptime in seconds\n\
             # TYPE todo_app_uptime_seconds gauge\n\
             todo_app_uptime_seconds {}\n\
             # HELP todo_app_shutting_down Whether the server is draining for shutdown (1 = yes, 0 = no)\n\
             # TYPE todo_app_shutting_down gauge\n\
             todo_app_shutting_down {}\n\
             # HELP todo_app_http_requests_total Total number of HTTP requests processed\n\
             # TYPE todo_app_http_requests_total counter\n\
             todo_app_http_requests_total {}\n\
             # HELP todo_app_realtime_connections_active Active SSE and WebSocket subscriber connections\n\
             # TYPE todo_app_realtime_connections_active gauge\n\
             todo_app_realtime_connections_active {}\n\
             # HELP todo_app_worker_ticks_total Total background worker cycles executed\n\
             # TYPE todo_app_worker_ticks_total counter\n\
             todo_app_worker_ticks_total {}\n\
             # HELP todo_app_worker_reminders_processed_total Total due reminders delivered by worker\n\
             # TYPE todo_app_worker_reminders_processed_total counter\n\
             todo_app_worker_reminders_processed_total {}\n\
             # HELP todo_app_worker_recurring_spawned_total Total recurring task instances spawned by worker\n\
             # TYPE todo_app_worker_recurring_spawned_total counter\n\
             todo_app_worker_recurring_spawned_total {}\n\
             # HELP todo_app_cache_hits_total Total in-memory data cache hits\n\
             # TYPE todo_app_cache_hits_total counter\n\
             todo_app_cache_hits_total {}\n\
             # HELP todo_app_cache_misses_total Total in-memory data cache misses\n\
             # TYPE todo_app_cache_misses_total counter\n\
             todo_app_cache_misses_total {}\n\
             # HELP todo_app_cache_invalidations_total Total in-memory data cache invalidations on write\n\
             # TYPE todo_app_cache_invalidations_total counter\n\
             todo_app_cache_invalidations_total {}\n\
             # HELP todo_app_database_connected Database connectivity status (1 = connected, 0 = disconnected)\n\
             # TYPE todo_app_database_connected gauge\n\
             todo_app_database_connected {}\n",
            self.uptime_seconds,
            if self.shutting_down { 1 } else { 0 },
            self.total_http_requests,
            self.active_realtime_connections,
            self.worker_ticks_total,
            self.worker_reminders_processed_total,
            self.worker_recurring_spawned_total,
            self.cache_hits_total,
            self.cache_misses_total,
            self.cache_invalidations_total,
            if self.database_connected { 1 } else { 0 },
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_metrics_counters_and_prometheus_format() {
        init_start_time();
        let before = total_http_requests();
        let after = inc_http_requests();
        assert!(after > before);

        record_worker_tick(2, 1);
        let snapshot = SystemMetricsSnapshot {
            uptime_seconds: 10,
            shutting_down: false,
            total_http_requests: after,
            active_realtime_connections: 3,
            worker_ticks_total: 1,
            worker_reminders_processed_total: 2,
            worker_recurring_spawned_total: 1,
            cache_hits_total: 5,
            cache_misses_total: 2,
            cache_invalidations_total: 1,
            database_connected: true,
        };

        let prom = snapshot.to_prometheus_text();
        assert!(prom.contains("todo_app_http_requests_total"));
        assert!(prom.contains("todo_app_realtime_connections_active 3"));
        assert!(prom.contains("todo_app_cache_hits_total 5"));
        assert!(prom.contains("todo_app_database_connected 1"));
    }
}
