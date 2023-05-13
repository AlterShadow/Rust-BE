use deadpool_postgres::Runtime;
use deadpool_postgres::*;
use eyre::*;
use serde::{Deserialize, Serialize};
use std::collections::hash_map::DefaultHasher;
use std::hash::Hash;
use std::hash::Hasher;
use tokio_postgres::types::ToSql;
use tokio_postgres::{NoTls, Row, ToStatement};
use tracing::*;
pub type DatabaseConfig = deadpool_postgres::Config;
#[derive(Clone)]
pub struct SimpleDbClient {
    pool: Pool,
    conn_hash: u64,
}
impl SimpleDbClient {
    pub async fn query<T>(
        &self,
        statement: &T,
        params: &[&(dyn ToSql + Sync)],
    ) -> Result<Vec<Row>, Error>
    where
        T: ?Sized + Sync + Send + ToStatement,
    {
        Ok(self.pool.get().await?.query(statement, params).await?)
    }
    pub fn conn_hash(&self) -> u64 {
        self.conn_hash
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DbResponse<T> {
    rows: Vec<T>,
}
impl<T> DbResponse<T> {
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            rows: Vec::with_capacity(capacity),
        }
    }
    pub fn rows(&self) -> &Vec<T> {
        &self.rows
    }
    pub fn into_rows(self) -> Vec<T> {
        self.rows
    }
    pub fn len(&self) -> usize {
        self.rows.len()
    }
    pub fn is_empty(&self) -> bool {
        self.rows.is_empty()
    }
    pub fn into_result(self) -> Option<T> {
        self.rows.into_iter().next()
    }
    pub fn push(&mut self, row: T) {
        self.rows.push(row);
    }
}
pub async fn connect_to_database(config: DatabaseConfig) -> Result<SimpleDbClient> {
    info!(
        "Connecting to database {}:{} {}",
        config.host.as_deref().unwrap_or(""),
        config.port.unwrap_or(0),
        config.dbname.as_deref().unwrap_or("")
    );
    let mut hasher = DefaultHasher::new();
    config.host.hash(&mut hasher);
    config.port.hash(&mut hasher);
    config.dbname.hash(&mut hasher);
    let conn_hash = hasher.finish();
    let pool = config.create_pool(Some(Runtime::Tokio1), NoTls)?;
    Ok(SimpleDbClient { pool, conn_hash })
}
