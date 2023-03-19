use crate::defaults::{METHOD_CREATE, METHOD_UPDATE, TRANSACTIONS_TABLE_NAME};
use crate::error::ServiceError;
use crate::error::ServiceError::{InternalError, InvalidMethod};
use crate::storage_impl::Storage;
use crate::transaction::Transaction;
use marine_sqlite_connector::{State, Statement, Value};

impl Storage {
    pub fn create_transactions_tables(&self) {
        let table_schema = format!(
            "
            CREATE TABLE IF NOT EXISTS {} (
                hash TEXT PRIMARY KEY,
                from_peer_id TEXT NOT NULL,
                host_id TEXT NOT NULL,
                status INTEGER NOT NULL,
                data_key TEXT NOT NULL,
                metadata TEXT NOT NULL,
                public_key TEXT NOT NULL,
                alias TEXT,
                timestamp INTEGER NOT NULL,
                error_text TEXT NULL,
                metadata_cid TEXT NOT NULL,
                encryption_type TEXT,
                method TEXT NOT NULL
            );",
            TRANSACTIONS_TABLE_NAME
        );

        let current_table_schema = self
            .get_table_schema(TRANSACTIONS_TABLE_NAME.to_string())
            .expect(f!("failed to get {TRANSACTIONS_TABLE_NAME} table schema").as_str());
        if !current_table_schema.is_empty() && table_schema != current_table_schema {
            self.delete_table(TRANSACTIONS_TABLE_NAME.to_string())
                .expect(f!("failed to delete {TRANSACTIONS_TABLE_NAME} table").as_str())
        }

        let result = self.connection.execute(table_schema);

        if let Err(error) = result {
            println!("create_transactions_table error: {}", error);
        }
    }

    pub fn write_transaction(&self, transaction: Transaction) -> Result<String, ServiceError> {
        if transaction.method != METHOD_CREATE && transaction.method != METHOD_UPDATE {
            return Err(InvalidMethod(f!("invalid methodh: {transaction.method}")));
        }

        let s = format!(
            "insert into {} (hash, from_peer_id, host_id, status, data_key, metadata, public_key, alias, timestamp, metadata_cid, encryption_type, method) values ('{}', '{}', '{}', '{}', '{}', '{}', '{}', '{}', '{}', '{}', '{}', '{}');",
            TRANSACTIONS_TABLE_NAME,
            transaction.hash,
            transaction.from_peer_id,
            transaction.host_id,
            transaction.status,
            transaction.data_key,
            transaction.metadata,
            transaction.public_key,
            transaction.alias,
            transaction.timestamp,
            transaction.metadata_cid,
            transaction.encryption_type,
            transaction.method
        );

        log::info!("{}", s);

        self.connection.execute(s)?;

        Ok(transaction.hash)
    }

    pub fn update_transaction_status(
        &self,
        hash: String,
        status: i32,
        error_text: String,
    ) -> Result<(), ServiceError> {
        self.connection.execute(format!(
            "
          update {}
          set status = '{}', error_text = '{}'
          where hash = '{}';
          ",
            TRANSACTIONS_TABLE_NAME, status, error_text, hash
        ))?;

        Ok(())
    }

    pub fn get_transaction(&self, hash: String) -> Result<Transaction, ServiceError> {
        let mut statement = self
            .connection
            .prepare(f!("SELECT * FROM {TRANSACTIONS_TABLE_NAME} WHERE hash = ?"))?;

        statement.bind(1, &Value::String(hash.clone()))?;

        if let State::Row = statement.next()? {
            read(&statement)
        } else {
            Err(InternalError(f!(
                "not found non-host records for given key_hash: {hash}"
            )))
        }
    }
}

pub fn read(statement: &Statement) -> Result<Transaction, ServiceError> {
    Ok(Transaction {
        hash: statement.read::<String>(0)?,
        from_peer_id: statement.read::<String>(1)?,
        host_id: statement.read::<String>(2)?,
        status: statement.read::<i64>(3)? as i64,
        data_key: statement.read::<String>(4)?,
        metadata: statement.read::<String>(5)?,
        public_key: statement.read::<String>(6)?,
        alias: statement.read::<String>(7)?,
        timestamp: statement.read::<i64>(8)? as u64,
        error_text: statement.read::<String>(9)?,
        metadata_cid: statement.read::<String>(10)?,
        encryption_type: statement.read::<String>(11)?,
        method: statement.read::<String>(12)?,
    })
}
