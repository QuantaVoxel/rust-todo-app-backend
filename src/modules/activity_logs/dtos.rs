use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct ActivityLogQuery {
    pub entity_type: Option<String>,
    pub entity_id: Option<i32>,
    pub action: Option<String>,
    pub from: Option<String>,
    pub to: Option<String>,
    pub page: Option<u64>,
    pub per_page: Option<u64>,
}
