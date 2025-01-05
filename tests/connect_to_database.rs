mod fixtures;

use crate::fixtures::db::{
    db_password, db_username, ns_password, ns_username, root_password, root_username,
};
use crate::fixtures::load_environment_variables;
use fixtures::db::initialize_database;
use speculoos::prelude::*;
use surrealdb_migrate::config::{DbAuthLevel, DbClientConfig};
use surrealdb_migrate::db::connect_to_database;

#[tokio::test]
async fn can_connect_to_database_as_root_user() {
    load_environment_variables();
    let config = DbClientConfig::default()
        .with_auth_level(DbAuthLevel::Root)
        .with_username(root_username())
        .with_password(root_password());
    let db = connect_to_database(config).await;

    assert_that!(db).is_ok();
}

#[tokio::test]
async fn can_connect_to_database_as_namespace_user() {
    load_environment_variables();
    initialize_database().await;

    let config = DbClientConfig::default()
        .with_auth_level(DbAuthLevel::Namespace)
        .with_username(ns_username())
        .with_password(ns_password());
    let db = connect_to_database(config).await;

    assert_that!(db).is_ok();
}

#[tokio::test]
async fn can_connect_to_database_as_database_user() {
    load_environment_variables();
    initialize_database().await;

    let config = DbClientConfig::default()
        .with_username(db_username())
        .with_password(db_password());
    let db = connect_to_database(config).await;

    assert_that!(db).is_ok();
}
