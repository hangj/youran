use anyhow::Result;
use rusqlite::Connection;
use std::{
    collections::HashMap,
    path::{Path, PathBuf},
};

const DEFAULT_DB_FILE: &'static str = "~/.config/youran/db.sqlite";
const ENV_DB_FILE: &'static str = "YOURAN_DB_FILE";
const TABLE_NAME: &'static str = "tb_kv";

#[derive(Debug)]
pub struct Db {
    conn: Connection,
}

impl Db {
    pub fn from<P: AsRef<Path>>(path: Option<P>) -> Result<Db> {
        let dbfile = if let Some(path) = path {
            expand_tilde(path.as_ref()).unwrap()
        } else if let Ok(path) = std::env::var(ENV_DB_FILE) {
            expand_tilde(path).unwrap()
        } else {
            expand_tilde(DEFAULT_DB_FILE).unwrap()
        };

        let dir = dbfile.parent().unwrap();

        std::fs::create_dir_all(dir)?;

        let conn = Connection::open(&dbfile)?;

        let db = Db { conn };
        db.prepare_table()?;

        Ok(db)
    }

    fn prepare_table(&self) -> Result<()> {
        self.conn.execute(
            &format!(
                "CREATE TABLE IF NOT EXISTS {TABLE_NAME}(
                key text primary key,
                value blob
            )"
            ),
            (),
        )?;
        Ok(())
    }

    pub fn get(&self, key: &str) -> Result<Option<Vec<u8>>> {
        let mut stmt = self
            .conn
            .prepare(&format!("SELECT key,value FROM {TABLE_NAME} where key=?"))?;
        let entries = stmt.query_map([key], |row| Ok(row.get::<usize, Vec<u8>>(1)?))?;
        for value in entries {
            return Ok(Some(value?));
        }
        Ok(None)
    }

    pub fn list(&self, limit: usize, offset: usize) -> Result<HashMap<String, Vec<u8>>> {
        let mut stmt = self.conn.prepare(&format!(
            "SELECT key, value FROM {TABLE_NAME} LIMIT {limit} OFFSET {offset}"
        ))?;

        let hm = stmt
            .query_map([], |row| {
                Ok((row.get::<usize, String>(0)?, row.get::<usize, Vec<u8>>(1)?))
            })?
            .into_iter()
            // .inspect(|x| {
            //     println!("x: {:#?}", x);
            // })
            .map(|v| v.unwrap())
            .collect();

        Ok(hm)
    }

    pub fn set(&self, key: &str, value: &[u8]) -> rusqlite::Result<usize> {
        // INSERT INTO table (id, name, age) VALUES(1, "A", 19) ON DUPLICATE KEY UPDATE name="A", age=19
        self.conn.execute(
            &format!("REPLACE INTO {TABLE_NAME}(key, value) VALUES(?1, ?2)"),
            (key, value),
        )
    }

    pub fn delete(&self, key: &str) -> rusqlite::Result<usize> {
        self.conn
            .execute(&format!("DELETE FROM {TABLE_NAME} WHERE key = ?1"), (key,))
    }

    pub fn clear(&self) -> rusqlite::Result<usize> {
        self.conn
            .execute(&format!("TRUNCATE TABLE {TABLE_NAME}"), ())
    }

    #[allow(unused)]
    fn drop(&self) -> rusqlite::Result<usize> {
        self.conn
            .execute(&format!("DROP TABLE IF EXISTS {TABLE_NAME}"), [])
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
