mod auth;

use hell_core::error::{HellErrorHelper, HellResult};
use reqwest::Client;

// https://platform.openai.com/docs/api-reference/authentication

// curl https://api.openai.com/v1/chat/completions \
//   -H "Content-Type: application/json" \
//   -H "Authorization: Bearer $OPENAI_API_KEY" \
//   -d '{
//      "model": "gpt-3.5-turbo",
//      "messages": [{"role": "user", "content": "Tell a very funny joke!"}],
//      "temperature": 0.7
//    }'


enum OpenAiModel {
    Gpt35Turbo,
}

impl OpenAiModel {
    pub fn to_str(&self) -> &str {
        match self {
            OpenAiModel::Gpt35Turbo => "gpt-3.5-turbo",
        }
    }
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct OpenAiMsg {
    role: String,
    content: String,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct OpenAiRequest {
    model: String,
    messages: Vec<OpenAiMsg>,
    temperature: f32,
}



pub async fn send_request() -> HellResult<OpenAiChatCompletionSuccessResponse> {
    let client = Client::new();

    let body = OpenAiRequest {
        model: OpenAiModel::Gpt35Turbo.to_str().to_string(),
        messages: vec![OpenAiMsg {
            role: "user".to_string(),
            content: "Tell a very funny joke!".to_string(),
        }],
        temperature: 0.7,
    };

    let auth = auth::OpenAiAuth::new();
    // let mut headers = HeaderMap::new();
    // headers.insert("Content-Type", "application/json".parse().unwrap());

    let request = client
        .post("https://api.openai.com/v1/chat/completions")
        .bearer_auth(&auth.api_key)
        // .headers(headers)
        .json(&body);

    println!("test: {:?}", request);
    let response = request.send().await.unwrap();

    if response.status().is_success() {
        Ok(response
            .json::<OpenAiChatCompletionSuccessResponse>()
            .await
            .unwrap())
    } else {
        let r = response
            .json::<OpenAiChatCompletionErrorResponse>()
            .await
            .unwrap();
        Err(HellErrorHelper::request_msg_err(r.error.message))
    }
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct OpenaiUsageData {
    prompt_tokens: u32,
    completion_tokens: u32,
    total_tokens: u32,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct OpenAiChoice {
    message: OpenAiMsg,
    finish_reason: String,
    index: u64
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct OpenAiChatCompletionSuccessResponse {
    id: String,
    object: String,
    created: u64,
    model: String,
    usage: OpenaiUsageData,
    choices: Vec<OpenAiChoice>
}

// ---------

#[derive(serde::Serialize, serde::Deserialize)]
struct OpenaiError {
    message: String,
    #[serde(rename = "type")]
    error_type: String,
    param: Option<String>,
    code: Option<String>,
}

#[derive(serde::Serialize, serde::Deserialize)]
struct OpenAiChatCompletionErrorResponse {
    error: OpenaiError
}


pub async fn list_models() -> String {
    let auth = auth::OpenAiAuth::new();
    let client = Client::new();

    client.get("https://api.openai.com/v1/models")
        .bearer_auth(&auth.api_key)
        .send().await.unwrap()
        .text().await.unwrap()
}
