use crate::datatable::RDataTable;
use dashmap::DashMap;
use deadpool_postgres::Runtime;
use deadpool_postgres::*;
use eyre::*;
use postgres_from_row::FromRow;
use secrecy::{ExposeSecret, SecretString};
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use std::collections::hash_map::DefaultHasher;
use std::hash::Hash;
use std::hash::Hasher;
use std::path::Path;
use std::process::Command;
use std::sync::Arc;
use std::time::Duration;
pub use tokio_postgres::types::ToSql;
use tokio_postgres::Statement;
pub use tokio_postgres::{NoTls, Row, ToStatement};
use tracing::*;

#[derive(Clone, Debug, Default, Deserialize)]
pub struct DatabaseConfig {
    /// See [`tokio_postgres::Config::user`].
    pub user: Option<String>,
    /// See [`tokio_postgres::Config::password`].
    pub password: Option<SecretString>,
    /// See [`tokio_postgres::Config::dbname`].
    pub dbname: Option<String>,
    /// See [`tokio_postgres::Config::options`].
    pub options: Option<String>,
    /// See [`tokio_postgres::Config::application_name`].
    pub application_name: Option<String>,
    /// See [`tokio_postgres::Config::ssl_mode`].
    pub ssl_mode: Option<SslMode>,
    /// This is similar to [`Config::hosts`] but only allows one host to be
    /// specified.
    ///
    /// Unlike [`tokio_postgres::Config`] this structure differentiates between
    /// one host and more than one host. This makes it possible to store this
    /// configuration in an environment variable.
    ///
    /// See [`tokio_postgres::Config::host`].
    pub host: Option<String>,
    /// See [`tokio_postgres::Config::host`].
    pub hosts: Option<Vec<String>>,
    /// This is similar to [`Config::ports`] but only allows one port to be
    /// specified.
    ///
    /// Unlike [`tokio_postgres::Config`] this structure differentiates between
    /// one port and more than one port. This makes it possible to store this
    /// configuration in an environment variable.
    ///
    /// See [`tokio_postgres::Config::port`].
    pub port: Option<u16>,
    /// See [`tokio_postgres::Config::port`].
    pub ports: Option<Vec<u16>>,
    /// See [`tokio_postgres::Config::connect_timeout`].
    pub connect_timeout: Option<Duration>,
    /// See [`tokio_postgres::Config::keepalives`].
    pub keepalives: Option<bool>,
    /// See [`tokio_postgres::Config::keepalives_idle`].
    pub keepalives_idle: Option<Duration>,
    /// See [`tokio_postgres::Config::target_session_attrs`].
    pub target_session_attrs: Option<TargetSessionAttrs>,
    /// See [`tokio_postgres::Config::channel_binding`].
    pub channel_binding: Option<ChannelBinding>,

    /// [`Manager`] configuration.
    ///
    /// [`Manager`]: super::Manager
    pub manager: Option<ManagerConfig>,

    /// [`Pool`] configuration.
    pub pool: Option<PoolConfig>,
}

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
    #[deprecated]
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

    pub async fn execute<T: DatabaseRequest>(&self, req: T) -> Result<RDataTable<T::ResponseRow>> {
        let mut error = None;
        for _ in 0..2 {
            let client = self
                .pool
                .get()
                .await
                .context("Failed to connect to database")?;
            let statement = client.prepare(req.statement()).await?;
            // TODO: cache statement along with the client into prepared_stmts. other wise there will be runtime error
            let rows = match client.query(&statement, &req.params()).await {
                Ok(rows) => rows,
                Err(err) => {
                    let reason = err.to_string();
                    if reason.contains("cache lookup failed for type")
                        || reason.contains("cached plan must not change result type")
                        || reason.contains("prepared statement")
                    {
                        warn!("Database has been updated. Cleaning cache and retrying query");
                        self.prepared_stmts.clear();
                        error = Some(err);
                        continue;
                    }
                    return Err(err.into());
                }
            };
            let mut response = RDataTable::with_capacity(rows.len());
            for row in rows {
                response.push(T::ResponseRow::try_from_row(&row)?);
            }
            return Ok(response);
        }
        Err(error.unwrap().into())
    }
    pub fn conn_hash(&self) -> u64 {
        self.conn_hash
    }
}

pub async fn connect_to_database(config: DatabaseConfig) -> Result<DbClient> {
    let config = Config {
        user: config.user,
        password: config.password.map(|s| s.expose_secret().clone()),
        dbname: config.dbname,
        options: config.options,
        application_name: config.application_name,
        ssl_mode: config.ssl_mode,
        host: config.host,
        hosts: config.hosts,
        port: config.port,
        ports: config.ports,
        connect_timeout: config.connect_timeout,
        keepalives: config.keepalives,
        keepalives_idle: config.keepalives_idle,
        target_session_attrs: config.target_session_attrs,
        channel_binding: config.channel_binding,
        manager: config.manager,
        pool: config.pool,
    };
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
        password: Some("123456".to_string().into()),
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
