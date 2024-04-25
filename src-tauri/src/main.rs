// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use anyhow::Result as anyhowResult;
use serde::{Deserialize, Serialize, Serializer};

/*
use rusqlite::{Connection, Result};
// Create the error type that represents all error possible in our program.
#[derive(Debug, thiserror::Error)]
pub enum CommandError {
    #[error(transparent)]
    RusqliteError(#[from] rusqlite::Error),
}

// we must manually implement serde::Serialize
impl Serialize for CommandError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(self.to_string().as_ref())
    }
}

pub type CommandResult<T, E = CommandError> = anyhow::Result<T, E>;


// Create Database using Rusqlite Library
#[tauri::command]
fn create_database() -> CommandResult<()> {
    let conn = Connection::open("test.db")?;
    conn.execute(
        "create table if not exists cat_colors (
            id integer primary key,
            name text not null unique
        )",
        [],
    )?;
    Ok(())
}
*/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_sqlite::init())
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
