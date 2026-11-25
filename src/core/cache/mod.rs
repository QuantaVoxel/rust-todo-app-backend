use std::sync::LazyLock;
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::{Duration, Instant};
use salvo::cache::{Cache, MokaStore, RequestIssuer};
use salvo::caching_headers::CachingHeaders;
use salvo::prelude::Handler;
use tokio::sync::RwLock;

use crate::entity::{
    external_apps, tags, task_categories, task_labels, task_priorities, task_statuses,
};

const DATA_CACHE_TTL: Duration = Duration::from_secs(300);

static CACHE_HITS_TOTAL: AtomicU64 = AtomicU64::new(0);
static CACHE_MISSES_TOTAL: AtomicU64 = AtomicU64::new(0);
static CACHE_INVALIDATIONS_TOTAL: AtomicU64 = AtomicU64::new(0);

pub fn cache_hits_total() -> u64 {
    CACHE_HITS_TOTAL.load(Ordering::Relaxed)
}

pub fn cache_misses_total() -> u64 {
    CACHE_MISSES_TOTAL.load(Ordering::Relaxed)
}

pub fn cache_invalidations_total() -> u64 {
    CACHE_INVALIDATIONS_TOTAL.load(Ordering::Relaxed)
}

/// Salvo HTTP Response Cache backed by `MokaStore` (5-minute TTL) for static reference endpoints.
pub fn lookup_response_cache() -> impl Handler {
    Cache::new(
        MokaStore::builder()
            .time_to_live(DATA_CACHE_TTL)
            .build(),
        RequestIssuer::default(),
    )
}

/// Salvo ETag & Last-Modified (`304 Not Modified`) caching headers middleware.
pub fn etag_caching_headers() -> CachingHeaders {
    CachingHeaders::new()
}

struct CachedEntry<T> {
    value: T,
    inserted_at: Instant,
}

impl<T: Clone> CachedEntry<T> {
    fn new(value: T) -> Self {
        Self {
            value,
            inserted_at: Instant::now(),
        }
    }

    fn get_if_fresh(&self, ttl: Duration) -> Option<T> {
        if self.inserted_at.elapsed() < ttl {
            Some(self.value.clone())
        } else {
            None
        }
    }
}

pub struct AppDataCache {
    tags: RwLock<Option<CachedEntry<Vec<tags::Model>>>>,
    categories: RwLock<Option<CachedEntry<Vec<task_categories::Model>>>>,
    labels: RwLock<Option<CachedEntry<Vec<task_labels::Model>>>>,
    statuses: RwLock<Option<CachedEntry<Vec<task_statuses::Model>>>>,
    priorities: RwLock<Option<CachedEntry<Vec<task_priorities::Model>>>>,
    external_apps: RwLock<Option<CachedEntry<Vec<external_apps::Model>>>>,
}

impl AppDataCache {
    pub fn new() -> Self {
        Self {
            tags: RwLock::new(None),
            categories: RwLock::new(None),
            labels: RwLock::new(None),
            statuses: RwLock::new(None),
            priorities: RwLock::new(None),
            external_apps: RwLock::new(None),
        }
    }

    // --- Tags ---
    pub async fn get_tags(&self) -> Option<Vec<tags::Model>> {
        let guard = self.tags.read().await;
        if let Some(entry) = guard.as_ref() {
            if let Some(val) = entry.get_if_fresh(DATA_CACHE_TTL) {
                CACHE_HITS_TOTAL.fetch_add(1, Ordering::Relaxed);
                return Some(val);
            }
        }
        CACHE_MISSES_TOTAL.fetch_add(1, Ordering::Relaxed);
        None
    }

    pub async fn set_tags(&self, items: Vec<tags::Model>) {
        let mut guard = self.tags.write().await;
        *guard = Some(CachedEntry::new(items));
    }

    pub async fn invalidate_tags(&self) {
        let mut guard = self.tags.write().await;
        *guard = None;
        CACHE_INVALIDATIONS_TOTAL.fetch_add(1, Ordering::Relaxed);
    }

    // --- Categories ---
    pub async fn get_categories(&self) -> Option<Vec<task_categories::Model>> {
        let guard = self.categories.read().await;
        if let Some(entry) = guard.as_ref() {
            if let Some(val) = entry.get_if_fresh(DATA_CACHE_TTL) {
                CACHE_HITS_TOTAL.fetch_add(1, Ordering::Relaxed);
                return Some(val);
            }
        }
        CACHE_MISSES_TOTAL.fetch_add(1, Ordering::Relaxed);
        None
    }

    pub async fn set_categories(&self, items: Vec<task_categories::Model>) {
        let mut guard = self.categories.write().await;
        *guard = Some(CachedEntry::new(items));
    }

    pub async fn invalidate_categories(&self) {
        let mut guard = self.categories.write().await;
        *guard = None;
        CACHE_INVALIDATIONS_TOTAL.fetch_add(1, Ordering::Relaxed);
    }

    // --- Labels ---
    pub async fn get_labels(&self) -> Option<Vec<task_labels::Model>> {
        let guard = self.labels.read().await;
        if let Some(entry) = guard.as_ref() {
            if let Some(val) = entry.get_if_fresh(DATA_CACHE_TTL) {
                CACHE_HITS_TOTAL.fetch_add(1, Ordering::Relaxed);
                return Some(val);
            }
        }
        CACHE_MISSES_TOTAL.fetch_add(1, Ordering::Relaxed);
        None
    }

    pub async fn set_labels(&self, items: Vec<task_labels::Model>) {
        let mut guard = self.labels.write().await;
        *guard = Some(CachedEntry::new(items));
    }

    pub async fn invalidate_labels(&self) {
        let mut guard = self.labels.write().await;
        *guard = None;
        CACHE_INVALIDATIONS_TOTAL.fetch_add(1, Ordering::Relaxed);
    }

    // --- Statuses ---
    pub async fn get_statuses(&self) -> Option<Vec<task_statuses::Model>> {
        let guard = self.statuses.read().await;
        if let Some(entry) = guard.as_ref() {
            if let Some(val) = entry.get_if_fresh(DATA_CACHE_TTL) {
                CACHE_HITS_TOTAL.fetch_add(1, Ordering::Relaxed);
                return Some(val);
            }
        }
        CACHE_MISSES_TOTAL.fetch_add(1, Ordering::Relaxed);
        None
    }

    pub async fn set_statuses(&self, items: Vec<task_statuses::Model>) {
        let mut guard = self.statuses.write().await;
        *guard = Some(CachedEntry::new(items));
    }

    // --- Priorities ---
    pub async fn get_priorities(&self) -> Option<Vec<task_priorities::Model>> {
        let guard = self.priorities.read().await;
        if let Some(entry) = guard.as_ref() {
            if let Some(val) = entry.get_if_fresh(DATA_CACHE_TTL) {
                CACHE_HITS_TOTAL.fetch_add(1, Ordering::Relaxed);
                return Some(val);
            }
        }
        CACHE_MISSES_TOTAL.fetch_add(1, Ordering::Relaxed);
        None
    }

    pub async fn set_priorities(&self, items: Vec<task_priorities::Model>) {
        let mut guard = self.priorities.write().await;
        *guard = Some(CachedEntry::new(items));
    }

    // --- External Apps ---
    pub async fn get_external_apps(&self) -> Option<Vec<external_apps::Model>> {
        let guard = self.external_apps.read().await;
        if let Some(entry) = guard.as_ref() {
            if let Some(val) = entry.get_if_fresh(DATA_CACHE_TTL) {
                CACHE_HITS_TOTAL.fetch_add(1, Ordering::Relaxed);
                return Some(val);
            }
        }
        CACHE_MISSES_TOTAL.fetch_add(1, Ordering::Relaxed);
        None
    }

    pub async fn set_external_apps(&self, items: Vec<external_apps::Model>) {
        let mut guard = self.external_apps.write().await;
        *guard = Some(CachedEntry::new(items));
    }

    /// Invalidates all cached classifications and lookups.
    pub async fn invalidate_all(&self) {
        *self.tags.write().await = None;
        *self.categories.write().await = None;
        *self.labels.write().await = None;
        *self.statuses.write().await = None;
        *self.priorities.write().await = None;
        *self.external_apps.write().await = None;
        CACHE_INVALIDATIONS_TOTAL.fetch_add(1, Ordering::Relaxed);
    }
}

static GLOBAL_DATA_CACHE: LazyLock<AppDataCache> = LazyLock::new(AppDataCache::new);

pub fn app_cache() -> &'static AppDataCache {
    &GLOBAL_DATA_CACHE
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_app_data_cache_hit_miss_and_invalidation() {
        let cache = AppDataCache::new();

        // Miss initially
        assert!(cache.get_tags().await.is_none());

        // Populate
        let sample = vec![tags::Model {
            tag_id: 1,
            name: "Rust".to_string(),
        }];
        cache.set_tags(sample.clone()).await;

        // Hit
        let cached = cache.get_tags().await;
        assert_eq!(cached, Some(sample));

        // Invalidate on write
        cache.invalidate_tags().await;
        assert!(cache.get_tags().await.is_none());
    }
}
