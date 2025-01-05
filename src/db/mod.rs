use crate::config::{DbAuthLevel, DbClientConfig};
use std::ops::Deref;
use std::sync::Arc;
use surrealdb::engine::any::{connect, Any};
use surrealdb::opt::auth;
use surrealdb::opt::auth::Jwt;
use surrealdb::Surreal;

pub type DbError = surrealdb::Error;

#[derive(Debug)]
struct Connection {
    client: Surreal<Any>,
    token: Jwt,
}

#[derive(Debug, Clone)]
pub struct DbConnection {
    inner: Arc<Connection>,
}

impl DbConnection {
    pub fn new(client: Surreal<Any>, token: Jwt) -> Self {
        Self {
            inner: Arc::new(Connection { client, token }),
        }
    }

    pub fn client(&self) -> &Surreal<Any> {
        &self.inner.client
    }

    pub fn token(&self) -> &Jwt {
        &self.inner.token
    }
}

impl From<(Surreal<Any>, Jwt)> for DbConnection {
    fn from((client, token): (Surreal<Any>, Jwt)) -> Self {
        Self::new(client, token)
    }
}

impl Deref for DbConnection {
    type Target = Surreal<Any>;

    fn deref(&self) -> &Self::Target {
        &self.inner.client
    }
}

pub async fn connect_to_database(config: DbClientConfig<'_>) -> Result<DbConnection, DbError> {
    let client = connect(config.address_or_default()).await?;

    let token = match config.auth_level() {
        DbAuthLevel::Root => client.signin(auth::Root {
            username: config.username_or_default(),
            password: config.password_or_default(),
        }),
        DbAuthLevel::Namespace => client.signin(auth::Namespace {
            namespace: config.namespace_or_default(),
            username: config.username_or_default(),
            password: config.password_or_default(),
        }),
        DbAuthLevel::Database => client.signin(auth::Database {
            namespace: config.namespace_or_default(),
            database: config.database_or_default(),
            username: config.username_or_default(),
            password: config.password_or_default(),
        }),
    }
    .await?;

    let _db = client
        .use_ns(config.namespace_or_default())
        .use_db(config.database_or_default());

    Ok(DbConnection::new(client, token))
}
