pub static DB_PATH: &str = "/tmp/node.db";
pub static TRANSACTIONS_TABLE_NAME: &str = "transactions";
pub static METADATAS_TABLE_NAME: &str = "metadatas";
pub static META_CONTRACT_TABLE_NAME: &str = "metacontracts";
pub static STATUS_PENDING: i64 = 0;
pub static STATUS_SUCCESS: i64 = 1;
pub static STATUS_FAILED: i64 = 2;
// METHODS
pub static METHOD_CONTRACT: &str = "contract";
pub static METHOD_METADATA: &str = "metadata";
pub static METHOD_CLONE: &str = "clone";
pub static ENCRYPTION_TYPE_SECP256K1: &str = "secp256k1";
pub static ENCRYPTION_TYPE_ED25519: &str = "ed25519";
