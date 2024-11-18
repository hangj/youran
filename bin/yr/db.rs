use anyhow::Result;
use rusqlite::Connection;
use std::path::{Path, PathBuf};

const DEFAULT_DB_FILE: &'static str = "~/.config/youran/db.sqlite";
const ENV_DB_FILE: &'static str = "YOURAN_DB_FILE";
const TABLE_NAME: &'static str = "tb_kv";
const INDEX_NAME: &'static str = "idx_timestamp";

#[derive(Debug)]
pub struct Db {
    dbfile: PathBuf,
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

        let db = Db { dbfile, conn };
        db.prepare_table()?;

        Ok(db)
    }

    fn prepare_table(&self) -> Result<()> {
        self.conn.execute(
            // The datatype is optional.
            // see <https://www.sqlite.org/quirks.html#the_datatype_is_optional>
            // and <https://www.sqlite.org/datatype3.html#datatypes_in_sqlite>
            &format!(
                "CREATE TABLE IF NOT EXISTS {TABLE_NAME} (
                key primary key,
                value,
                timestamp DEFAULT CURRENT_TIMESTAMP
            )"
            ),
            (),
        )?;

        self.conn.execute(
            &format!("CREATE INDEX IF NOT EXISTS {INDEX_NAME} ON {TABLE_NAME} (timestamp)"),
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

    pub fn list(&self, limit: usize, offset: usize) -> Result<Vec<(String, Vec<u8>)>> {
        let mut stmt = self.conn.prepare(&format!(
            "SELECT key, value FROM {TABLE_NAME} ORDER BY timestamp desc LIMIT {limit} OFFSET {offset}"
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
        // extern crate chrono;
        // use chrono::prelude::*;
        // let date : DateTime = Utc::now(); // Local::now();
        // date.format("%Y-%m-%d %H:%M:%S").to_string();
        // Utc.datetime_from_str("2014-11-28 12:00:09", "%Y-%m-%d %H:%M:%S").unwrap();

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

    /// truncate table
    pub fn clear(&self) -> rusqlite::Result<usize> {
        // sqlite will do the truncate optimization
        // https://sqlite.org/lang_delete.html
        self.conn.execute(&format!("DELETE FROM {TABLE_NAME}"), ())
    }

    /// drop the table
    #[allow(unused)]
    pub fn drop_table(&self) -> rusqlite::Result<usize> {
        self.conn
            .execute(&format!("DROP TABLE IF EXISTS {TABLE_NAME}"), [])
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
