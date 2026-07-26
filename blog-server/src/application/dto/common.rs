use serde::Deserialize;

#[derive(Debug, Deserialize, Clone, PartialEq, Eq)]
#[serde(default)]
pub struct Pagination {
    pub offset: i64,
    pub limit: i64,
}

impl Default for Pagination {
    fn default() -> Self {
        Self {
            offset: 0,
            limit: 10,
        }
    }
}
