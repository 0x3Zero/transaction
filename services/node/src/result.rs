use eyre::Result;
use marine_rs_sdk::marine;

use crate::{
    cron::{Cron},
    cron_tx::CronTx,
    error::ServiceError,
    meta_contract::MetaContract,
    metadatas::Metadata,
    transaction::{Transaction, TransactionReceipt}, registry::Registry,
};

#[marine]
#[derive(Debug)]
pub struct FdbResult {
    pub transaction_hash: String,
}

#[marine]
#[derive(Debug)]
pub struct FdbClock {
    pub timestamp: i64,
}

#[marine]
#[derive(Debug)]
pub struct FdbTransactionResult {
    pub success: bool,
    pub err_msg: String,
    pub transaction: Transaction,
}

impl From<Result<Transaction, ServiceError>> for FdbTransactionResult {
    fn from(result: Result<Transaction, ServiceError>) -> Self {
        match result {
            Ok(transaction) => Self {
                success: true,
                err_msg: "".to_string(),
                transaction,
            },
            Err(err) => Self {
                success: false,
                err_msg: err.to_string(),
                transaction: Transaction::default(),
            },
        }
    }
}

#[marine]
#[derive(Debug)]
pub struct FdbTransactionsResult {
    pub success: bool,
    pub err_msg: String,
    pub transactions: Vec<Transaction>,
}

impl From<Result<Vec<Transaction>, ServiceError>> for FdbTransactionsResult {
    fn from(result: Result<Vec<Transaction>, ServiceError>) -> Self {
        match result {
            Ok(transactions) => Self {
                success: true,
                err_msg: "".to_string(),
                transactions,
            },
            Err(err) => Self {
                success: false,
                err_msg: err.to_string(),
                transactions: Vec::new(),
            },
        }
    }
}

#[marine]
#[derive(Debug)]
pub struct FdbTransactionReceiptResult {
    pub success: bool,
    pub err_msg: String,
    pub receipt: TransactionReceipt,
}

impl From<Result<TransactionReceipt, ServiceError>> for FdbTransactionReceiptResult {
    fn from(result: Result<TransactionReceipt, ServiceError>) -> Self {
        match result {
            Ok(receipt) => Self {
                success: true,
                err_msg: "".to_string(),
                receipt,
            },
            Err(err) => Self {
                success: false,
                err_msg: err.to_string(),
                receipt: TransactionReceipt::default(),
            },
        }
    }
}

#[marine]
#[derive(Debug)]
pub struct FdbTransactionReceiptsResult {
    pub success: bool,
    pub err_msg: String,
    pub receipts: Vec<TransactionReceipt>,
}

impl From<Result<Vec<TransactionReceipt>, ServiceError>> for FdbTransactionReceiptsResult {
    fn from(result: Result<Vec<TransactionReceipt>, ServiceError>) -> Self {
        match result {
            Ok(receipts) => Self {
                success: true,
                err_msg: "".to_string(),
                receipts,
            },
            Err(err) => Self {
                success: false,
                err_msg: err.to_string(),
                receipts: Vec::new(),
            },
        }
    }
}

#[marine]
#[derive(Debug)]
pub struct FdbMetadataResult {
    pub success: bool,
    pub err_msg: String,
    pub metadata: Metadata,
}

impl From<Result<Metadata, ServiceError>> for FdbMetadataResult {
    fn from(result: Result<Metadata, ServiceError>) -> Self {
        match result {
            Ok(metadata) => Self {
                success: true,
                err_msg: "".to_string(),
                metadata,
            },
            Err(err) => Self {
                success: false,
                err_msg: err.to_string(),
                metadata: Metadata::default(),
            },
        }
    }
}

#[marine]
#[derive(Debug)]
pub struct FdbMetadatasResult {
    pub success: bool,
    pub err_msg: String,
    pub metadatas: Vec<Metadata>,
}

impl From<Result<Vec<Metadata>, ServiceError>> for FdbMetadatasResult {
    fn from(result: Result<Vec<Metadata>, ServiceError>) -> Self {
        match result {
            Ok(metadatas) => Self {
                success: true,
                err_msg: "".to_string(),
                metadatas,
            },
            Err(err) => Self {
                success: false,
                err_msg: err.to_string(),
                metadatas: Vec::new(),
            },
        }
    }
}

#[marine]
#[derive(Debug)]
pub struct FdbMetadataHistoryResult {
    pub success: bool,
    pub err_msg: String,
    pub metadata: String,
    pub history: Vec<String>,
}

impl From<Result<Vec<String>, ServiceError>> for FdbMetadataHistoryResult {
    fn from(result: Result<Vec<String>, ServiceError>) -> Self {
        match result {
            Ok(metadatas) => Self {
                success: true,
                err_msg: "".to_string(),
                metadata: metadatas[0].clone(),
                history: metadatas[1..].to_vec(),
            },
            Err(err) => Self {
                success: false,
                err_msg: err.to_string(),
                metadata: "{}".to_string(),
                history: Vec::new(),
            },
        }
    }
}

#[marine]
#[derive(Debug)]
pub struct FdbMetaContractResult {
    pub success: bool,
    pub err_msg: String,
    pub meta: MetaContract,
}

impl From<Result<MetaContract, ServiceError>> for FdbMetaContractResult {
    fn from(result: Result<MetaContract, ServiceError>) -> Self {
        match result {
            Ok(meta) => Self {
                success: true,
                err_msg: "".to_string(),
                meta,
            },
            Err(err) => Self {
                success: false,
                err_msg: err.to_string(),
                meta: MetaContract::default(),
            },
        }
    }
}

#[marine]
#[derive(Debug)]
pub struct FdbRegistryResult {
    pub success: bool,
    pub err_msg: String,
    pub registry: Registry,
}

impl From<Result<Registry, ServiceError>> for FdbRegistryResult {
    fn from(result: Result<Registry, ServiceError>) -> Self {
        match result {
            Ok(registry) => Self {
                success: true,
                err_msg: "".to_string(),
                registry,
            },
            Err(err) => Self {
                success: false,
                err_msg: err.to_string(),
                registry: Registry::default(),
            },
        }
    }
}

#[marine]
#[derive(Debug)]
pub struct FdbCronsResult {
    pub success: bool,
    pub err_msg: String,
    pub crons: Vec<Cron>,
}

impl From<Result<Vec<Cron>, ServiceError>> for FdbCronsResult {
    fn from(result: Result<Vec<Cron>, ServiceError>) -> Self {
        match result {
            Ok(crons) => Self {
                success: true,
                err_msg: "".to_string(),
                crons,
            },
            Err(err) => Self {
                success: false,
                err_msg: err.to_string(),
                crons: Vec::new(),
            },
        }
    }
}

#[marine]
#[derive(Debug)]
pub struct FdbCronTxsResult {
    pub success: bool,
    pub err_msg: String,
    pub cron_txs: Vec<CronTx>,
}

impl From<Result<Vec<CronTx>, ServiceError>> for FdbCronTxsResult {
    fn from(result: Result<Vec<CronTx>, ServiceError>) -> Self {
        match result {
            Ok(cron_txs) => Self {
                success: true,
                err_msg: "".to_string(),
                cron_txs,
            },
            Err(err) => Self {
                success: false,
                err_msg: err.to_string(),
                cron_txs: Vec::new(),
            },
        }
    }
}

#[marine]
#[derive(Debug)]
pub struct FdbCronTxResult {
    pub success: bool,
    pub err_msg: String,
    pub cron_tx: CronTx,
}

impl From<Result<CronTx, ServiceError>> for FdbCronTxResult {
    fn from(result: Result<CronTx, ServiceError>) -> Self {
        match result {
            Ok(cron_tx) => Self {
                success: true,
                err_msg: "".to_string(),
                cron_tx,
            },
            Err(err) => Self {
                success: false,
                err_msg: err.to_string(),
                cron_tx: CronTx::default(),
            },
        }
    }
}
