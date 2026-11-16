use scrutiny::Validate;
use serde::Deserialize;

#[derive(Debug, Deserialize, Validate)]
pub struct TagRequest {
    #[validate(required, min = 1, max = 255)]
    pub name: Option<String>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct CategoryRequest {
    #[validate(required, min = 1, max = 255)]
    pub name: Option<String>,
    pub description: Option<String>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct LabelRequest {
    #[validate(required, min = 1, max = 255)]
    pub name: Option<String>,
}
