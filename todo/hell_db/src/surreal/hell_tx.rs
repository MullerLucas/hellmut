use hell_error::{HellResult, ErrToHellErr};
use surrealdb::Transaction;

use crate::HellDb;

#[derive(Debug)]
pub struct HellTxInfo {
    pub lock: bool,
    pub write: bool,
}

#[allow(dead_code)]
pub struct HellTx<'a> {
    db: &'a mut HellDb,
    tx: Transaction,
}

impl<'a> HellTx<'a> {
    pub async fn new(db: &'a mut HellDb) -> HellResult<HellTx> {
        let info = db.info();
        let tx = db.datastore().transaction(info.tx.write, info.tx.lock).await.to_hell_err(hell_error::HellErrorKind::DbError)?;

        Ok(Self {
            db,
            tx
        })
    }


    /// Check if the transaction is finished
    /// If the transaction has been cancelled or committed, then this function will return true, and any further calls to functions on this transaction will result in a Error::TxFinished error.
    pub async fn is_closed(&self) -> bool {
        self.tx.closed().await
    }

    /// Cancel a transaction.
    /// This reverses all changes made within the transaction.
    pub async fn cancel(mut self) -> HellResult<()> {
        Ok(self.tx.cancel().await.to_hell_err(hell_error::HellErrorKind::DbError)?)
    }

    /// Commit a transaction.
    /// This attempts to commit all changes made within the transaction.
    pub async fn commit(&mut self) -> HellResult<()> {
        Ok(self.tx.commit().await.to_hell_err(hell_error::HellErrorKind::DbError)?)
    }
}
