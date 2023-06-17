use dashmap::DashMap;
use deadpool_postgres::Runtime;
use deadpool_postgres::*;
use eyre::*;
use postgres_from_row::FromRow;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use std::collections::hash_map::DefaultHasher;
use std::hash::Hash;
use std::hash::Hasher;
use std::path::Path;
use std::process::Command;
use std::sync::Arc;
pub use tokio_postgres::types::ToSql;
use tokio_postgres::Statement;
pub use tokio_postgres::{NoTls, Row, ToStatement};
use tracing::*;

pub type DatabaseConfig = deadpool_postgres::Config;
pub trait DatabaseRequest {
    type ResponseRow: Send + Sync + Clone + Serialize + DeserializeOwned + FromRow;
    fn statement(&self) -> &str;
    fn params(&self) -> Vec<&(dyn ToSql + Sync)>;
}
#[derive(Clone)]
pub struct DbClient {
    pool: Pool,
    prepared_stmts: Arc<DashMap<String, Statement>>,
    conn_hash: u64,
}
impl DbClient {
    pub async fn query<T>(
        &self,
        statement: &T,
        params: &[&(dyn ToSql + Sync)],
    ) -> Result<Vec<Row>, Error>
    where
        T: ?Sized + Sync + Send + ToStatement,
    {
        Ok(self
            .pool
            .get()
            .await
            .context("Failed to connect to database")?
            .query(statement, params)
            .await?)
    }
    pub async fn prepare(&self, statement: &str) -> Result<Statement, Error> {
        Ok(self
            .pool
            .get()
            .await
            .context("Failed to connect to database")?
            .prepare(statement)
            .await?)
    }
    pub async fn execute<T: DatabaseRequest>(&self, req: T) -> Result<DataTable<T::ResponseRow>> {
        let mut error = None;
        for _ in 0..2 {
            let statement = if let Some(stmt) = self.prepared_stmts.get(req.statement()) {
                stmt.clone()
            } else {
                let stmt = self.prepare(req.statement()).await?;
                self.prepared_stmts
                    .insert(req.statement().to_string(), stmt.clone());
                stmt
            };

            let rows = match self.query(&statement, &req.params()).await {
                Ok(rows) => rows,
                Err(err) => {
                    let reason = err.to_string();
                    if reason.contains("cache lookup failed for type")
                        || reason.contains("cached plan must not change result type")
                    {
                        warn!("Database has been updated. Cleaning cache and retrying query");
                        self.prepared_stmts.clear();
                        error = Some(err);
                        continue;
                    }
                    return Err(err);
                }
            };
            let mut response = DataTable::with_capacity(rows.len());
            for row in rows {
                response.push(T::ResponseRow::try_from_row(&row)?);
            }
            return Ok(response);
        }
        Err(error.unwrap())
    }
    pub fn conn_hash(&self) -> u64 {
        self.conn_hash
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DataTable<T> {
    rows: Vec<T>,
}
impl<T> DataTable<T> {
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            rows: Vec::with_capacity(capacity),
        }
    }
    pub fn first<R>(&self, f: impl Fn(&T) -> R) -> Option<R> {
        self.rows.first().map(|x| f(x))
    }
    pub fn rows(&self) -> &Vec<T> {
        &self.rows
    }
    pub fn into_rows(self) -> Vec<T> {
        self.rows
    }
    pub fn into_iter(self) -> impl Iterator<Item = T> {
        self.rows.into_iter()
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
    pub fn map<R>(self, f: impl Fn(T) -> R) -> Vec<R> {
        self.rows.into_iter().map(f).collect()
    }
}
pub async fn connect_to_database(config: DatabaseConfig) -> Result<DbClient> {
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
    Ok(DbClient {
        pool,
        prepared_stmts: Arc::new(Default::default()),
        conn_hash,
    })
}
pub fn database_test_config() -> DatabaseConfig {
    DatabaseConfig {
        user: Some("postgres".to_string()),
        password: Some("123456".to_string()),
        dbname: Some("mc2fi".to_string()),
        host: Some("localhost".to_string()),
        ..Default::default()
    }
}

pub fn drop_and_recreate_database() -> Result<()> {
    let script = Path::new("scripts/drop_and_recreate_database.sh");
    Command::new("bash")
        .arg(script)
        .arg("etc/config.json")
        .status()?;
    Ok(())
}
