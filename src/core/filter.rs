use salvo::macros::Extractible;
use scrutiny::Validate;
use sea_orm::{ColumnTrait, Order};
use serde::{Deserialize, Serialize};
use std::str::FromStr;

#[derive(Deserialize, Serialize, Validate, Extractible, Debug)]
#[salvo(extract(default_source(from = "query")))]
pub struct FilterQueryDto {
    #[validate(min = 3, max = 100)]
    pub search: Option<String>,
    #[validate(nullable, min = 1)]
    pub page: Option<u64>,
    #[validate(nullable, min = 1)]
    pub limit: Option<u64>,
    pub order_by: Option<String>,
    pub order_dir: Option<String>,
}

pub fn build_dynamic_sort<C>(
    order_by: Option<&String>,
    order_dir: Option<&String>,
    default_column: C,
    allowed_columns: &[&str],
) -> (C, Order)
    where
        C: ColumnTrait + FromStr,
{
    let column = order_by
        .map(|s| s.trim())
        .filter(|s| allowed_columns.contains(s))
        .and_then(|s| C::from_str(s).ok())
        .unwrap_or(default_column);

    let direction = match order_dir.map(|s| s.trim().to_lowercase()).as_deref() {
        Some("asc") => Order::Asc,
        _ => Order::Desc,
    };

    (column, direction)
}