use std::time::Duration;

use chrono::{DateTime, Local, TimeZone};
use rusqlite::{params, Connection};

use crate::config::Config;

#[derive(Debug)]
pub struct History {
    #[allow(dead_code)]
    pub id: i64,
    pub started_at: DateTime<Local>,
    pub duration: u64,
}

impl History {
    pub fn pretty_print(&self, print_id: bool) {
        let duration = humantime::format_duration(Duration::from_secs(self.duration));
        if print_id {
            println!("{} / {} / {}", self.id, self.started_at, duration);
        } else {
            println!("{} / {}", self.started_at, duration);
        }
    }
}

pub fn insert_history(config: &Config, started_at: i64, duration: u64) -> Result<(), anyhow::Error> {
    let started_at = Local.timestamp_opt(started_at, 0).unwrap();
    let conn = get_db(config)?;
    conn.execute("INSERT INTO history (started_at, duration) VALUES (?1, ?2)", params![started_at, duration])?;
    Ok(())
}

pub fn delete_history(config: &Config, id: u32) -> Result<(), anyhow::Error> {
    let conn = get_db(config)?;
    conn.execute("DELETE FROM history WHERE id = ?1", params![id])?;
    Ok(())
}

pub fn query_history(config: &Config) -> Result<Vec<History>, anyhow::Error> {
    let conn = get_db(config)?;
    let mut stmt = conn.prepare("SELECT id, started_at, duration FROM history ORDER BY started_at DESC limit 100")?;
    let iter =
        stmt.query_map(params![], |row| Ok(History { id: row.get(0)?, started_at: row.get(1)?, duration: row.get(2)? }))?;

    let mut result = Vec::new();
    for data in iter {
        result.push(data?);
    }

    Ok(result)
}

fn get_db(config: &Config) -> Result<Connection, anyhow::Error> {
    let conn = Connection::open(&config.db_path)?;
    conn.execute(
        "CREATE TABLE IF NOT EXISTS history (
            id INTEGER PRIMARY KEY,
            started_at TEXT NOT NULL,
            duration INTEGER NOT NULL
        )",
        [],
    )?;
    Ok(conn)
}
