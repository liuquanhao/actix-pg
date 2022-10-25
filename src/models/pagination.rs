use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Pagination {
    pub page: Option<usize>,
    pub per_page: Option<usize>,
}
