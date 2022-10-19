/// [docs: surrealdb::Datastore](https://docs.rs/surrealdb/1.0.0-beta.8/surrealdb/struct.Datastore.html)

use std::collections::BTreeMap;

use hell_error::{HellResult, HellError};
use surrealdb::sql::Query;
use surrealdb::{Datastore, Session, sql};

use crate::{HellTx, HellTxInfo, HellDbResponse};

#[derive(Debug)]
pub struct HellDbInfo {
    pub path: String,
    pub namespace: String,
    pub database: String,
    pub tx: HellTxInfo,
}

#[allow(dead_code)]
pub struct HellDb {
    info: HellDbInfo,
    datastore: Datastore,
    session: Session,
    strict: bool,
}

impl HellDb {
    pub async fn new(info: HellDbInfo) -> HellResult<Self> {
        let datastore = Datastore::new(&info.path).await?;
        let session = Session::for_db(&info.namespace, &info.database);

        println!("HellDb::new:: with info: \n{:#?}", info);

        Ok(Self {
            info,
            datastore,
            session,
            strict: false,
        })
    }

    pub fn info(&self) -> &HellDbInfo {
        &self.info
    }

    pub fn datastore(&self) -> &Datastore {
        &self.datastore
    }
}

impl HellDb {
    pub async fn create_transaction(&mut self) -> HellResult<HellTx> {
        HellTx::new(self).await
    }

    pub async fn execute(&self, txt: &str, vars: Option<BTreeMap<String, sql::Value>>) -> HellResult<()> {
        let _ = Self::execute_internal(self, txt, vars).await?;
        Ok(())
    }

    pub async fn fetch_all<T>(&self, txt: &str, vars: Option<BTreeMap<String, sql::Value>>) -> HellResult<Vec<T>>
        where for <'de> T: serde::Deserialize<'de>
    {
        Self::execute_internal(self, txt, vars).await?
            .parse_all()
    }

    pub async fn fetch_optional<T>(&self, txt: &str, vars: Option<BTreeMap<String, sql::Value>>) -> HellResult<Option<T>>
        where for <'de> T: serde::Deserialize<'de>
    {
        let mut res = Self::fetch_all(self, txt, vars).await?;
        if res.is_empty() {
            Ok(None)
        }
        else {
            Ok(Some(res.remove(0)))
        }
    }

    pub async fn fetch_one<T>(&self, txt: &str, vars: Option<BTreeMap<String, sql::Value>>) -> HellResult<T>
        where for <'de> T: serde::Deserialize<'de>
    {
        Self::fetch_optional(self, txt, vars).await?
            .ok_or_else(|| HellError::new(hell_error::HellErrorKind::GenericError, hell_error::HellErrorContent::Message("failed to fetch-one".to_string())))
    }

}

#[allow(dead_code)]
impl HellDb {
    /// Parse and execute an SQL query
    async fn execute_internal(&self, txt: &str, vars: Option<BTreeMap<String, sql::Value>>) -> HellResult<HellDbResponse> {
        HellDbResponse::new(
            self.datastore.execute(txt, &self.session, vars, self.strict).await?
        )
    }

    /// Execute a pre-parsed SQL query
    async fn process_internal(&self, ast: Query, vars: Option<BTreeMap<String, sql::Value>>) -> HellResult<HellDbResponse> {
        HellDbResponse::new(
            self.datastore.process(ast, &self.session, vars, self.strict).await?
        )
    }

    /// Ensure a SQL Value is fully computed
    async fn compute_internal(&self, val: sql::Value, vars: Option<BTreeMap<String, sql::Value>>) -> HellResult<sql::Value> {
        Ok(self.datastore.compute(val, &self.session, vars, self.strict).await?)
    }

    /// Performs a full database export as SQL
    async fn export_internal(&self, ns: String, db: String, chn: surrealdb::channel::Sender<Vec<u8>>) -> HellResult<()> {
        Ok(self.datastore.export(ns, db, chn).await?)
    }
}
