use rand::{distributions::Alphanumeric, Rng};

// Helper functions for slug generation and URL validation
pub fn generate_slug() -> String {
    rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(6)
        .map(char::from)
        .collect()
}

pub fn is_valid_url(url: &str) -> bool {
    url.starts_with("http://") || url.starts_with("https://")
}