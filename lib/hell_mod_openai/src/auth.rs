// pub const OPENAI_ORG_ID: &str = "org-wXWnMq5GVAxD5xeJtPqGg5ir";
// pub const OPENAI_API_KEY: &str ="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";


pub struct ApiAuth {
    pub api_key: String,
    pub ord_id: String,
}

impl std::fmt::Debug for ApiAuth {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "[auth...]")
    }
}

impl Default for ApiAuth {
    fn default() -> Self {
        Self::new()
    }
}

impl ApiAuth {
    pub fn new() -> Self {
        Self {
            api_key: std::env::var("OPENAI_API_KEY").unwrap(),
            ord_id:  std::env::var("OPENAI_ORG_ID").unwrap(),
        }
    }
}
