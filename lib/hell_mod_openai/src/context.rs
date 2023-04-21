use reqwest::Client;
use crate::auth::ApiAuth;

#[derive(Debug, Default)]
pub struct ApiContext {
    pub auth: ApiAuth,
    pub client: Client,
}
