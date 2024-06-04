use crate::configuration::Settings;
use async_trait::async_trait;
use std::fs;

#[async_trait]
pub trait Migration: Send + Sync {
    fn output_file_name(&self) -> &str;
    async fn run(&self, settings: &Settings);
}

pub struct MigrationConstructor(pub fn() -> Box<dyn Migration>);

inventory::collect!(MigrationConstructor);

pub async fn run(settings: &Settings) {
    let existing_migration_files = get_existing_migration_files();
    println!("{:?}", existing_migration_files);

    let mut migrations: Vec<Box<dyn Migration>> = inventory::iter::<MigrationConstructor>()
        .map(|constr| constr.0())
        .filter(|migration| {
            !existing_migration_files.contains(&migration.output_file_name().to_string())
        })
        .collect();
    migrations.sort_by(|m1, m2| m1.output_file_name().cmp(m2.output_file_name()));

    for migration in migrations {
        migration.run(settings).await;
    }
}

fn get_existing_migration_files() -> Vec<String> {
    fs::read_dir("./migrations/")
        .unwrap()
        .into_iter()
        .filter_map(|result| result.ok())
        .filter(|entry| entry.file_type().map(|ft| ft.is_file()).unwrap_or(false))
        .map(|file| file.file_name().into_string().unwrap())
        .collect()
}
