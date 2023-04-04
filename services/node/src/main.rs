#![allow(improper_ctypes)]

mod defaults;
mod error;
mod meta_contract;
mod meta_contract_impl;
mod metadatas;
mod metadatas_impl;
mod result;
mod storage_impl;
mod transaction;
pub mod transactions_impl;

use defaults::{ENCRYPTION_TYPE_ED25519, ENCRYPTION_TYPE_SECP256K1, STATUS_SUCCESS};
use defaults::{METHOD_CONTRACT, METHOD_METADATA, STATUS_FAILED};
use marine_rs_sdk::marine;
use marine_rs_sdk::module_manifest;
use marine_rs_sdk::WasmLoggerBuilder;

use error::ServiceError::{
    self, InvalidMethod, InvalidOwner, InvalidSignature, NoEncryptionType,
    NotSupportedEncryptionType,
};
use meta_contract::MetaContract;
use metadatas::{FinalMetadata, Metadata};
use result::{
    FdbMetaContractResult, FdbMetadataHistoryResult, FdbMetadatasResult, FdbTransactionResult,
    FdbTransactionsResult,
};
use result::{FdbMetadataResult, FdbResult};
use serde_json::Value;
use std::time::{SystemTime, UNIX_EPOCH};
use storage_impl::get_storage;
use transaction::{Transaction, TransactionSubset};
use types::{IpfsDagGetResult, IpfsDagPutResult};

#[macro_use]
extern crate fstrings;

module_manifest!();

pub fn wrapped_try<F, T>(func: F) -> T
where
    F: FnOnce() -> T,
{
    func()
}

pub fn main() {
    WasmLoggerBuilder::new()
        .with_log_level(log::LevelFilter::Info)
        .build()
        .unwrap();

    let storage = get_storage().unwrap();
    storage.create_meta_contract_tables();
    storage.create_transactions_tables();
    storage.create_metadatas_tables();
}

#[marine]
pub fn send_transaction(
    data_key: String,
    token_key: String,
    token_id: String,
    alias: String,
    public_key: String,
    signature: String,
    data: String,
    method: String,
    nonce: i64,
) -> FdbResult {
    let mut service_id = "".to_string();
    let mut error: Option<ServiceError> = None;
    let storage = get_storage().expect("Database non existance");

    if error.is_none() {
        if method != METHOD_CONTRACT && method != METHOD_METADATA {
            error = Some(InvalidMethod(f!("invalid method: {method}")));
        }
    }

    let enc_verify = get_public_key_type(public_key.clone().as_str());
    if enc_verify.len() <= 0 {
        error = Some(ServiceError::InvalidEncryption(public_key.clone()));
    }

    if error.is_none() {
        if method == METHOD_METADATA {
            let result = storage.get_owner_metadata_by_datakey_and_alias(
                data_key.clone(),
                public_key.clone(),
                alias.clone(),
            );

            log::info!("{:?}", result);

            match result {
                Ok(metadata) => {
                    if metadata.public_key != public_key {
                        error = Some(InvalidOwner(f!("not owner of data_key: {public_key}")));
                    }
                }
                Err(ServiceError::RecordNotFound(_)) => {}
                Err(e) => error = Some(e),
            }
        } else if method == METHOD_CONTRACT {
            service_id = data.clone();
        }
    }

    if error.is_none() {
        if enc_verify.clone().is_empty() {
            error = Some(NoEncryptionType())
        } else {
            if enc_verify.clone().ne(ENCRYPTION_TYPE_SECP256K1)
                && enc_verify.clone().ne(ENCRYPTION_TYPE_ED25519)
            {
                error = Some(NotSupportedEncryptionType(enc_verify.clone()));
            }
        }
    }

    if error.is_none() {
        let v = verify(
            public_key.clone(),
            signature.clone(),
            data.clone(),
            enc_verify.clone(),
        );

        if !v {
            error = Some(InvalidSignature(f!("not owner of data_key: {public_key}")));
        }
    }

    let cp = marine_rs_sdk::get_call_parameters();

    let now = SystemTime::now();
    let timestamp = now.duration_since(UNIX_EPOCH).expect("Time went backwards");

    let mut transaction = Transaction::new(
        token_key,
        cp.init_peer_id,
        cp.host_id,
        data_key,
        nonce,
        data,
        public_key,
        alias,
        timestamp.as_millis() as u64,
        service_id,
        method,
        token_id,
    );

    if !error.is_none() {
        transaction.error_text = error.unwrap().to_string();
        transaction.status = STATUS_FAILED;
    }

    log::info!("{:?}", transaction);
    let _ = storage.write_transaction(transaction.clone());

    FdbResult {
        transaction_hash: transaction.hash,
    }
}

#[marine]
pub fn get_transaction(hash: String) -> FdbTransactionResult {
    wrapped_try(|| get_storage()?.get_transaction(hash)).into()
}

#[marine]
pub fn get_metadata(data_key: String, public_key: String, alias: String) -> FdbMetadataResult {
    wrapped_try(|| {
        get_storage()?.get_owner_metadata_by_datakey_and_alias(data_key, public_key, alias)
    })
    .into()
}

#[marine]
pub fn get_metadatas(data_key: String) -> FdbMetadatasResult {
    wrapped_try(|| get_storage()?.get_metadata_by_datakey(data_key)).into()
}

#[marine]
pub fn get_meta_contract(token_key: String) -> FdbMetaContractResult {
    wrapped_try(|| get_storage()?.get_meta_contract(token_key)).into()
}

#[marine]
pub fn get_pending_transactions() -> FdbTransactionsResult {
    wrapped_try(|| get_storage()?.get_pending_transactions()).into()
}

#[marine]
pub fn get_metadata_with_history(
    data_key: String,
    public_key: String,
    alias: String,
) -> FdbMetadataHistoryResult {
    wrapped_try(|| {
        let storage = get_storage().expect("Internal error to database connector");

        let result = storage.get_owner_metadata_by_datakey_and_alias(data_key, public_key, alias);

        let metadata;
        let mut metadatas: Vec<String> = Vec::new();

        match result {
            Ok(m) => {
                metadata = m;
            }
            Err(e) => return Err(e),
        };

        let mut read_metadata_cid: String = metadata.cid.clone();

        while read_metadata_cid.len() > 0 {
            let result = get(read_metadata_cid.clone(), "".to_string(), 0);
            let val: Value = serde_json::from_str(&result.block).unwrap();

            let input = format!(r#"{}"#, val);
            metadatas.push(input);

            let previous_cid = val
                .get("previous")
                .and_then(|v| v.get("/"))
                .and_then(|v| v.as_str());

            if previous_cid.is_none() {
                break;
            } else {
                read_metadata_cid = previous_cid.unwrap().to_string();
            }
        }

        Ok(metadatas)
    })
    .into()
}

// *********** SMART CONTRACT *****************
#[marine]
pub fn bind_meta_contract(transaction_hash: String) {
    let mut current_meta_contract;
    let mut is_update = false;
    let mut error: Option<ServiceError> = None;

    let storage = get_storage().expect("Internal error to database connector");

    let mut transaction = storage.get_transaction(transaction_hash).unwrap().clone();

    let sm_result = storage.get_meta_contract(transaction.token_key.clone());

    match sm_result {
        Ok(contract) => {
            if transaction.public_key != contract.public_key {
                error = Some(InvalidOwner(f!("{transaction.public_key}")))
            } else {
                current_meta_contract = contract;
                current_meta_contract.meta_contract_id = transaction.data.clone();
            }
            is_update = true;
        }
        Err(ServiceError::RecordNotFound(_)) => {}
        Err(e) => error = Some(e),
    }

    if error.is_none() {
        let meta_result;

        if !is_update {
            current_meta_contract = MetaContract {
                token_key: transaction.token_key.clone(),
                meta_contract_id: transaction.meta_contract_id.clone(),
                public_key: transaction.public_key.clone(),
            };

            meta_result = storage.write_meta_contract(current_meta_contract);
        } else {
            meta_result = storage
                .rebind_meta_contract(transaction.token_key.clone(), transaction.data.clone());
        }

        match meta_result {
            Ok(()) => {}
            Err(e) => error = Some(e),
        }
    }

    if !error.is_none() {
        transaction.error_text = error.unwrap().to_string();
        transaction.status = STATUS_FAILED;
    } else {
        transaction.status = STATUS_SUCCESS;
        transaction.error_text = "".to_string();
    }

    let _ = storage.update_transaction_status(
        transaction.hash.clone(),
        transaction.status.clone(),
        transaction.error_text.clone(),
    );
}

// *********** VALIDATOR *****************
#[marine]
pub fn set_metadata(
    transaction_hash: String,
    meta_contract_id: String,
    on_metacontract_result: bool,
    metadatas: Vec<FinalMetadata>,
    final_error_msg: String,
) {
    let storage = get_storage().expect("Internal error to database connector");
    let mut transaction = storage.get_transaction(transaction_hash).unwrap().clone();

    if !on_metacontract_result {
        transaction.status = STATUS_FAILED;
        if final_error_msg.is_empty() {
            transaction.error_text = "Metadata not updateable".to_string();
        } else {
            transaction.error_text = final_error_msg;
        }
    } else {
        for data in metadatas {
            let result = storage.get_owner_metadata_by_datakey_and_alias(
                transaction.data_key.clone(),
                data.public_key.clone(),
                data.alias.clone(),
            );

            log::info!("{:?}", result);

            match result {
                Ok(metadata) => {
                    transaction.status = STATUS_SUCCESS;

                    let tx = TransactionSubset {
                        hash: transaction.hash.clone(),
                        timestamp: transaction.timestamp.clone(),
                        meta_contract_id: meta_contract_id.clone(),
                    };

                    let tx_serde = serde_json::to_string(&tx).unwrap();

                    let result_ipfs_dag_put =
                        put_block(data.content, metadata.cid, tx_serde, "".to_string(), 0);
                    let content_cid = result_ipfs_dag_put.cid;

                    let _ = storage.update_cid(metadata.data_key, metadata.public_key, content_cid);
                }
                Err(ServiceError::RecordNotFound(_)) => {
                    transaction.status = STATUS_SUCCESS;

                    let tx = TransactionSubset {
                        hash: transaction.hash.clone(),
                        timestamp: transaction.timestamp.clone(),
                        meta_contract_id: meta_contract_id.clone(),
                    };

                    let tx_serde = serde_json::to_string(&tx).unwrap();

                    let result_ipfs_dag_put =
                        put_block(data.content, "".to_string(), tx_serde, "".to_string(), 0);
                    let content_cid = result_ipfs_dag_put.cid;

                    let metadata = Metadata::new(
                        transaction.data_key.clone(),
                        data.alias.clone(),
                        content_cid,
                        data.public_key.clone(),
                    );

                    let _ = storage.write_metadata(metadata);
                }
                Err(e) => {
                    transaction.error_text = e.to_string();
                    transaction.status = STATUS_FAILED;
                }
            };
        }
    }

    let _ = storage.update_transaction_status(
        transaction.hash.clone(),
        transaction.status.clone(),
        transaction.error_text.clone(),
    );
}

/************************ *********************/
#[marine]
#[link(wasm_import_module = "ipfsdag")]
extern "C" {
    #[link_name = "put_block"]
    pub fn put_block(
        content: String,
        previous_cid: String,
        transaction: String,
        api_multiaddr: String,
        timeout_sec: u64,
    ) -> IpfsDagPutResult;

    #[link_name = "get"]
    pub fn get(hash: String, api_multiaddr: String, timeout_sec: u64) -> IpfsDagGetResult;
}

#[marine]
#[link(wasm_import_module = "crypto")]
extern "C" {
    #[link_name = "verify"]
    pub fn verify(public_key: String, signature: String, message: String, enc: String) -> bool;

    #[link_name = "get_public_key_type"]
    pub fn get_public_key_type(public_key: &str) -> String;
}
