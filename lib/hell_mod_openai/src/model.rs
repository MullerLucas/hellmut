#[derive(Debug)]
pub enum LangModel {
    Gpt35Turbo,
}

impl ToString for LangModel {
    fn to_string(&self) -> String {
        match self {
            LangModel::Gpt35Turbo => "gpt-3.5-turbo",
        }.to_string()
    }
}

impl LangModel {
    pub fn token_limit(&self) -> u32 {
        match self {
            LangModel::Gpt35Turbo => 4096,
        }
    }
}

