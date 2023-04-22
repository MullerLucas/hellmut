#[derive(Debug, serde::Serialize, serde::Deserialize, Clone, Copy)]
pub enum LlmChatRole {
    System,
    Assistant,
    User,
}

impl ToString for LlmChatRole {
    fn to_string(&self) -> String {
        match self {
            LlmChatRole::System => "system",
            LlmChatRole::Assistant => "assistant",
            LlmChatRole::User => "user",
        }.to_string()
    }
}

// ----------------------------------------------------------------------------
