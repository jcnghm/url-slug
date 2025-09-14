use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct CreateSlugRequest {
    pub url: String,
}

#[derive(Serialize)]
pub struct SlugResponse {
    pub slug: String,
    pub short_url: String,
}

#[derive(Serialize)]
pub struct SlugStatsResponse {
    pub original_url: String,
    pub slug: String,
    pub click_count: u64,
}