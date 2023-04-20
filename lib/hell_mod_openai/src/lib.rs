mod auth;

use reqwest::{Client, header::{HeaderMap, CONTENT_TYPE}};


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
struct OpenAiMsg {
    role: String,
    content: String,
}

#[derive(serde::Serialize, serde::Deserialize)]
struct OpenAiRequest {
    model: String,
    messages: Vec<OpenAiMsg>,
    temperature: f32,
}



pub async fn send_request() -> String {
    let client = Client::new();

    let body = OpenAiRequest {
        model: "gpt-3.5-turbo".to_string(),
        messages: vec![OpenAiMsg {
            role: "user".to_string(),
            content: "Tell a very funny joke!".to_string(),
        }],
        temperature: 0.7,
    };

    let auth = auth::OpenAiAuth::new();
    let mut headers = HeaderMap::new();
    headers.insert("Content-Type", "application/json".parse().unwrap());

    let request = client
        .post("https://api.openai.com/v1/chat/completions")
        .bearer_auth(&auth.api_key)
        .headers(headers)
        .json(&body);

    println!("test: {:?}", request);
    let response = request.send().await.unwrap()
        .text().await.unwrap();

    println!("request send");

    response
}

// {
//     "error": {
//         "message": "you must provide a model parameter",
//         "type": "invalid_request_error",
//         "param": null,
//         "code": null
//     }
// }


pub async fn list_models() -> String {
    let auth = auth::OpenAiAuth::new();
    let client = Client::new();

    client.get("https://api.openai.com/v1/models")
        .bearer_auth(&auth.api_key)
        .send().await.unwrap()
        .text().await.unwrap()
}
