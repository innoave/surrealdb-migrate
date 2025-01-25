use crate::fixtures::db::{
    client_config_for_testcontainer, connect_to_test_database_as_database_user, get_db_tables_info,
    start_surrealdb_testcontainer,
};
use assertor::*;
use std::collections::HashMap;
use std::path::Path;
use surrealdb_migrate::config::RunnerConfig;
use surrealdb_migrate::runner::MigrationRunner;

mod fixtures;

#[tokio::test]
async fn run_migrations_on_empty_db() {
    let db_server = start_surrealdb_testcontainer().await;
    let db_config = client_config_for_testcontainer(&db_server).await;
    let db = connect_to_test_database_as_database_user(db_config).await;

    let config =
        RunnerConfig::default().with_migrations_folder(Path::new("../fixtures/basic/migrations"));
    let runner = MigrationRunner::new(config);

    runner.migrate(&db).await.expect("failed to run migrations");

    let tables_info = get_db_tables_info(&db).await;

    assert_that!(tables_info.keys())
        .contains_exactly(["migrations".to_string(), "quote".to_string()].iter());

    let quotes: Vec<HashMap<String, String>> = db
        .query("SELECT text FROM quote ORDER BY text")
        .await
        .expect("failed to query quotes")
        .take(0)
        .expect("did not get expected query result");

    assert_that!(quotes.iter().map(|row| &row["text"])).contains_exactly_in_order(
        [
            "Behind every great man is a woman rolling her eyes. - Jim Carrey".to_string(),
            "If you want a guarantee, buy a toaster. - Clint Eastwood".to_string(),
            "It takes considerable knowledge just to realize the extent of your own ignorance. - Thomas Sowell".to_string(),
            "don't seek happiness - create it".to_string(),
        ]
        .iter(),
    );
}
