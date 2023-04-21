use hell_core::error::{HellErrorHelper, HellResult};
use reqwest::{Client, RequestBuilder};

use crate::{model::LangModel, context::ApiContext, auth::ApiAuth};

// https://platform.openai.com/docs/api-reference/authentication

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub enum ChatRole {
    System,
    Assistant,
    User,
}

impl ToString for ChatRole {
    fn to_string(&self) -> String {
        match self {
            ChatRole::System => "system",
            ChatRole::Assistant => "assistant",
            ChatRole::User => "user",
        }.to_string()
    }
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct ChatMessage {
    pub role: String,
    pub content: String,
}

impl ChatMessage {
    pub fn new(role: &ChatRole, content: impl Into<String>) -> Self {
        Self {
            role: role.to_string(),
            content: content.into(),
        }
    }

    pub fn new_system(content: impl Into<String>) -> Self {
        Self::new(&ChatRole::System, content)
    }

    pub fn new_assistant(content: impl Into<String>) -> Self {
        Self::new(&ChatRole::Assistant, content)
    }

    pub fn new_user(content: impl Into<String>) -> Self {
        Self::new(&ChatRole::User, content)
    }
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct ChatRequestData {
    pub model: String,
    pub messages: Vec<ChatMessage>,
    pub max_tokens: Option<u32>,
    pub temperature: f32,
}

impl ChatRequestData {
    pub fn new(model: LangModel, messages: Vec<ChatMessage>, max_tokens: Option<u32>, temperature: f32) -> HellResult<Self> {
        if let Some(max_tokens) = max_tokens {
            if max_tokens > model.token_limit() {
                return Err(HellErrorHelper::request_msg_err("Max tokens must be less than or equal to the model's token limit"));
            }
        }

        Ok(Self {
            model: model.to_string(),
            messages,
            max_tokens,
            temperature,
        })
    }
}

#[derive(Debug)]
pub struct ChatRequest {
    request: RequestBuilder,
}

impl ChatRequest {
    pub fn from_data(cx: &ApiContext, body: &ChatRequestData) -> Self {
        let request = cx.client
            .post("https://api.openai.com/v1/chat/completions")
            .bearer_auth(&cx.auth.api_key)
            .json(body);

        Self {
            request,
        }
    }

    pub async fn send(self) -> HellResult<ChatSuccessResponse> {
        let response = self.request.send().await?;

        if response.status().is_success() {
            Ok(response
                .json::<ChatSuccessResponse>()
                .await?)
        } else {
            let response = response
                .json::<ChatErrorResponse>()
                .await?;
            Err(HellErrorHelper::request_msg_err(response.error.message))
        }
    }
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct ChatUsageInfo {
    prompt_tokens: u32,
    completion_tokens: u32,
    total_tokens: u32,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct ChatChoice {
    message: ChatMessage,
    /// ## Possible Values:
    /// - **stop**: API returned complete model output
    /// - **length**: Incomplete model output due to *max_tokens* parameter or token limit
    /// - **content_filter**: Omitted content due to a flag from our content filters
    /// - **null**: API response still in progress or incomplete
    finish_reason: String,
    index: u64
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct ChatSuccessResponse {
    id: String,
    object: String,
    created: u64,
    model: String,
    usage: ChatUsageInfo,
    choices: Vec<ChatChoice>
}

// ---------

#[derive(serde::Serialize, serde::Deserialize)]
struct ChatError {
    message: String,
    #[serde(rename = "type")]
    error_type: String,
    param: Option<String>,

    /// ## API errors
    /// **401** - Invalid Authentication
    /// **401** - Incorrect API key provided
    /// **401** - You must be a member of an organization to use the API
    /// **429** - Rate limit reached for requests
    /// **429** - You exceeded your current quota, please check your plan and billing details
    /// **429** - The engine is currently overloaded, please try again later
    /// **500** - The server had an error while processing your request
    code: Option<String>,
}

#[derive(serde::Serialize, serde::Deserialize)]
struct ChatErrorResponse {
    error: ChatError
}


pub async fn list_models() -> String {
    let auth = ApiAuth::new();
    let client = Client::new();

    client.get("https://api.openai.com/v1/models")
        .bearer_auth(&auth.api_key)
        .send().await.unwrap()
        .text().await.unwrap()
}
