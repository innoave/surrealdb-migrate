use cli_table::format::Justify;
use cli_table::{Cell, CellStruct, Table, TableDisplay};
use color_eyre::Report;
use color_eyre::eyre::WrapErr;
use surrealdb_migrate::config::MIGRATION_KEY_FORMAT_STR;
use surrealdb_migrate::migration::{Execution, Migration};

fn migrations_table_header() -> Vec<CellStruct> {
    vec![
        "Key".cell(),
        "Title".cell(),
        "Kind".cell(),
        "Script".cell(),
        "Rank".cell(),
        "Applied at".cell(),
        "Applied by".cell(),
        "Checksum".cell(),
    ]
}

pub fn format_migration_table(
    migrations: Vec<(Migration, Option<Execution>)>,
) -> Result<TableDisplay, Report> {
    migrations
        .into_iter()
        .map(|(mig, exe)| {
            if let Some(exe) = exe {
                vec![
                    mig.key.format(MIGRATION_KEY_FORMAT_STR).to_string().cell(),
                    mig.title.cell(),
                    mig.kind.to_string().cell(),
                    mig.script_path
                        .file_name()
                        .map_or_else(String::new, |filename| {
                            filename.to_string_lossy().to_string()
                        })
                        .cell(),
                    exe.applied_rank.to_string().cell().justify(Justify::Right),
                    exe.applied_at
                        .naive_local()
                        .format("%Y-%m-%d %H:%M:%S")
                        .to_string()
                        .cell(),
                    exe.applied_by.cell(),
                    exe.checksum.to_string().cell(),
                ]
            } else {
                vec![
                    mig.key.format(MIGRATION_KEY_FORMAT_STR).to_string().cell(),
                    mig.title.cell(),
                    mig.kind.to_string().cell(),
                    mig.script_path
                        .file_name()
                        .map_or_else(String::new, |filename| {
                            filename.to_string_lossy().to_string()
                        })
                        .cell(),
                    "".cell().justify(Justify::Right),
                    "".cell(),
                    "".cell(),
                    "".cell(),
                ]
            }
        })
        .table()
        .title(migrations_table_header())
        .display()
        .wrap_err("can not format migrations as table")
}
