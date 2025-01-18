mod fixtures;

use crate::fixtures::db::{
    client_config_for_testcontainer, db_password, db_username, ns_password, ns_username,
    start_surrealdb_testcontainer,
};
use crate::fixtures::load_environment_variables;
use assertor::*;
use database_migration::config::DbAuthLevel;
use fixtures::db::initialize_database;
use surrealdb_migrate_db_client::connect_to_database;

#[tokio::test]
async fn test_surrealdb_version() {
    let db_server = start_surrealdb_testcontainer().await;
    let config = client_config_for_testcontainer(&db_server).await;
    let db = connect_to_database(&config)
        .await
        .expect("failed to connect to SurrealDb testcontainer");

    let db_version = db.version().await.expect("failed to get SurrealDB version");

    assert_that!(db_version.major).is_equal_to(2);
    assert_that!(db_version.minor).is_equal_to(1);
}

#[tokio::test]
async fn can_connect_to_database_as_root_user() {
    load_environment_variables();
    let db_server = start_surrealdb_testcontainer().await;

    let config = client_config_for_testcontainer(&db_server)
        .await
        .with_auth_level(DbAuthLevel::Root);
    let db = connect_to_database(&config).await;

    assert_that!(db).is_ok();
}

#[tokio::test]
async fn can_connect_to_database_as_namespace_user() {
    load_environment_variables();
    let db_server = start_surrealdb_testcontainer().await;
    let config = client_config_for_testcontainer(&db_server).await;
    initialize_database(&config).await;

    let config = config
        .with_namespace("playground")
        .with_auth_level(DbAuthLevel::Namespace)
        .with_username(ns_username())
        .with_password(ns_password());
    let db = connect_to_database(&config).await;

    assert_that!(db).is_ok();
}

#[tokio::test]
async fn can_connect_to_database_as_database_user() {
    load_environment_variables();
    let db_server = start_surrealdb_testcontainer().await;
    let config = client_config_for_testcontainer(&db_server).await;
    initialize_database(&config).await;

    let config = config
        .with_namespace("playground")
        .with_database("test")
        .with_auth_level(DbAuthLevel::Database)
        .with_username(db_username())
        .with_password(db_password());
    let db = connect_to_database(&config).await;

    assert_that!(db).is_ok();
}
