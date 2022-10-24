use std::time::Duration;

use hell_error::HellResult;
use surrealdb::Response;
use surrealdb::sql::Value;



#[derive(Debug)]
pub struct HellDbResponse {
    pub sql: Option<String>,
    pub time: Duration,
    pub payload_raw: Value,
}

impl HellDbResponse {

    pub fn new(mut responses: Vec<Response>) -> HellResult<HellDbResponse> {
        if responses.len() != 1 {
            panic!("responses vec has more then 1 element - i don't know what that means :O");
        }

        let res = responses.remove(0);
        let payload_raw = res.result?;

        Ok(Self {
            sql: res.sql,
            time: res.time,
            payload_raw,
        })
    }

    pub fn parse_json(&self) -> HellResult<String> {
        Ok(serde_json::to_string(&self.payload_raw)?)
    }

    pub fn parse_all<T>(&self) -> HellResult<Vec<T>>
        where for<'de> T: serde::Deserialize<'de>
    {
        let json = self.parse_json()?;
        Ok(serde_json::from_str(&json)?)
    }
}
