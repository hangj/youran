use anyhow::{Result};
// use rusqlite::Connection;
use sqlx::{sqlite::SqlitePoolOptions, Pool, Sqlite, Row};
use std::path::{Path, PathBuf};

const DEFAULT_DB_FILE: &'static str = "~/.config/youran/db.sqlite";
const ENV_DB_FILE: &'static str = "YOURAN_DB_FILE";
const TABLE_NAME: &'static str = "tb_kv";

#[derive(Debug)]
pub struct Db {
    dbfile: PathBuf,
    conn: Pool<Sqlite>,
}

impl Db {
    pub async fn from<P: AsRef<Path>>(path: Option<P>) -> Result<Db> {
        let dbfile = if let Some(path) = path {
            expand_tilde(path.as_ref()).unwrap()
        } else if let Ok(path) = std::env::var(ENV_DB_FILE) {
            expand_tilde(path).unwrap()
        } else {
            expand_tilde(DEFAULT_DB_FILE).unwrap()
        };

        let dir = dbfile.parent().unwrap();

        std::fs::create_dir_all(dir)?;

        let conn = SqlitePoolOptions::new()
            .max_connections(1)
            .connect(format!("sqlite://{}", dbfile.display()).as_str())
            .await?;

        sqlx::migrate!().run(&conn).await?;

        let db = Db { dbfile, conn };

        Ok(db)
    }

    pub async fn get(&self, key: &str) -> Result<Option<Vec<u8>>> {
        let rows = sqlx::query(format!("SELECT value FROM {TABLE_NAME} where key=?").as_str())
            .bind(key)
            .fetch_all(&self.conn)
            .await?;

        if rows.is_empty() {
            return Ok(None);
        }

        Ok(Some(rows[0].get("value")))
    }

    pub async fn list(&self, limit: usize, offset: usize) -> Result<Vec<(String, Vec<u8>)>> {
        let rows = sqlx::query_as(
            format!(
                "SELECT key, value FROM {TABLE_NAME} ORDER BY timestamp desc LIMIT ? OFFSET ?"
                ).as_str()
            )
            .bind(limit as u32)
            .bind(offset as u32)
            .fetch_all(&self.conn)
            .await?;

        Ok(rows)
    }

    pub async fn set(&self, key: &str, value: &[u8]) -> Result<u64> {
        // extern crate chrono;
        // use chrono::prelude::*;
        // let date : DateTime = Utc::now(); // Local::now();
        // date.format("%Y-%m-%d %H:%M:%S").to_string();
        // Utc.datetime_from_str("2014-11-28 12:00:09", "%Y-%m-%d %H:%M:%S").unwrap();

        // INSERT INTO table (id, name, age) VALUES(1, "A", 19) ON DUPLICATE KEY UPDATE name="A", age=19

        Ok(sqlx::query(format!("REPLACE INTO {TABLE_NAME}(key, value) VALUES(?, ?)").as_str())
            .bind(key)
            .bind(value)
            .execute(&self.conn)
            .await?
            .rows_affected())
    }

    pub async fn delete(&self, key: &str) -> Result<u64> {
        Ok(sqlx::query(format!("DELETE FROM {TABLE_NAME} WHERE key = ?").as_str())
            .bind(key)
            .execute(&self.conn)
            .await?
            .rows_affected())
    }

    /// truncate table
    pub async fn clear(&self) -> Result<()> {
        // sqlite will do the truncate optimization
        // https://sqlite.org/lang_delete.html
        sqlx::query(format!("DELETE FROM {TABLE_NAME}").as_str())
            .execute(&self.conn)
            .await?;
        Ok(())
    }

    /// drop the table
    #[allow(unused)]
    pub async fn drop_table(&self) -> Result<()> {
        sqlx::query(format!("DROP TABLE IF EXISTS {TABLE_NAME}").as_str())
            .execute(&self.conn)
            .await?;
        Ok(())
    }

    /// remove the dbfile
    #[allow(unused)]
    pub fn drop_database(&self) -> std::io::Result<()> {
        std::fs::remove_file(self.dbfile.as_path())
    }
}

/// expand tilde in rust
/// https://stackoverflow.com/questions/54267608/expand-tilde-in-rust-path-idiomatically
fn expand_tilde<P: AsRef<Path>>(path_user_input: P) -> Option<PathBuf> {
    let p = path_user_input.as_ref();
    if !p.starts_with("~") {
        return Some(p.to_path_buf());
    }
    if p == Path::new("~") {
        return dirs::home_dir();
    }
    dirs::home_dir().map(|mut h| {
        if h == Path::new("/") {
            // Corner case: `h` root directory;
            // don't prepend extra `/`, just drop the tilde.
            p.strip_prefix("~").unwrap().to_path_buf()
        } else {
            h.push(p.strip_prefix("~/").unwrap());
            h
        }
    })
}
