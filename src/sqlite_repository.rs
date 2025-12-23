use diesel::prelude::*;
use diesel_migrations::{EmbeddedMigrations, MigrationHarness, embed_migrations};
use dotenvy::dotenv;
use std::{
    env,
    path::{Path, PathBuf},
};

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("./migrations");

pub fn run_migrations(conn: &mut SqliteConnection) {
    match conn.run_pending_migrations(MIGRATIONS) {
        Ok(_) => println!("successfully applied migrations"),
        Err(err) => println!("failed to run migrations: {}", err),
    };
}

pub fn establish_connection() -> SqliteConnection {
    dotenv().ok();

    let mut path = PathBuf::from(neighbor_chat::data_dir());
    path.push(env::var("DATABASE_URL").expect("DATABASE_URL must be set"));

    let path = path.to_str().unwrap();

    SqliteConnection::establish(&path).unwrap_or_else(|_| panic!("Error connecting to {}", path))
}
