use crate::core::errors::ApiError;
use sea_orm::{ConnectionTrait, Paginator, SelectorTrait};
use serde::{Deserialize, Serialize};

// 1. Struct Meta Data
#[derive(Debug, Serialize, Deserialize)]
pub struct PaginationMeta {
    pub current_page: u64,
    pub per_page: u64,
    pub total_pages: u64,
    pub total_items: u64,
}

// 2. Struct Generic Response
#[derive(Debug, Serialize, Deserialize)]
pub struct PaginatedResponse<T> {
    pub meta: PaginationMeta,
    pub data: Vec<T>,
}

// 3. Core Function (Constructor) pembungkus Response
impl<T> PaginatedResponse<T> {
    pub fn new(
        current_page: u64,
        per_page: u64,
        total_pages: u64,
        total_items: u64,
        data: Vec<T>,
    ) -> Self {
        Self {
            meta: PaginationMeta {
                current_page,
                per_page,
                total_pages,
                total_items,
            },
            data,
        }
    }
}

pub async fn fetch_pagination<'db, C, S>(
    paginator: Paginator<'db, C, S>,
    page: u64,
    limit: u64,
) -> Result<(Vec<S::Item>, PaginationMeta), ApiError>
where
    C: ConnectionTrait,
    S: SelectorTrait + 'db,
{
    // 1. Eksekusi num_items_and_pages()
    let items_and_pages = paginator.num_items_and_pages().await.map_err(|_| {
        ApiError::internal_server_error("Gagal menghitung data paginasi dari database")
    })?;

    let total_items = items_and_pages.number_of_items;
    let total_pages = items_and_pages.number_of_pages;

    // 2. Cegah crash jika request halaman melebihi data (opsional, tapi disarankan)
    let actual_page = if page > 0 { page } else { 1 };

    // 3. Ambil data halamannya
    let data = paginator.fetch_page(actual_page - 1).await.map_err(|_| {
        ApiError::internal_server_error("Gagal mengambil data halaman dari database")
    })?;

    // 4. Susun Meta
    let meta = PaginationMeta {
        current_page: actual_page,
        per_page: limit,
        total_pages,
        total_items,
    };

    Ok((data, meta))
}


