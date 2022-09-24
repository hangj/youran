use anyhow::Result;
use clap::Parser;
// use qrcode_generator::QrCodeEcc;
use rusqlite::Connection;
use std::{
    ffi::OsString,
    io::Write,
    os::unix::prelude::OsStrExt,
    path::{Path, PathBuf},
};

const DEFAULT_DB_FILE: &'static str = "~/.config/youran/db.sqlite";
const ENV_DB_FILE: &'static str = "YOURAN_DB_FILE";
const TABLE_NAME: &'static str = "tb_kv";


#[derive(Debug, Parser)]
#[clap(name="yr", author, version, about, long_about = None)]
struct Args {
    /// give more verbose output
    #[clap(short, long, value_parser, default_value_t = false)]
    verbose: bool,
    /// specify database file, use ~/.config/youran/db.sqlite by default if not set environment variable YOURAN_DB_FILE
    #[clap(long, value_parser)]
    db: Option<PathBuf>,
    /// Do not print the trailing newline character
    #[clap(
        short,
        long,
        value_parser,
        default_value_t = true,
        action(clap::ArgAction::SetFalse)
    )]
    newline: bool,
    #[clap(subcommand)]
    action: Action,
}

#[derive(clap::Subcommand, Debug)]
enum Action {
    /// get the value of the given key
    Get(Get),
    /// show the QRCode for the given key
    Qr(Get),
    /// set the value of the given key
    Set(Set),
    /// list all the keys
    Ls,
    /// clear all the keys
    Clear,
}

#[derive(Debug, Parser)]
struct Get {
    key: String,
}

#[derive(Debug, Parser)]
struct Set {
    key: String,
    #[clap(action(clap::ArgAction::Set))] // allow invalid utf8
    value: Option<OsString>,
}

fn main() -> Result<()> {
    let args = Args::try_parse();
    if args.is_err() {
        let args: Vec<_> = std::env::args_os().collect();
        println!("> args_os: {:#?}", args);
    }
    let args = args?;

    if args.verbose {
        println!("> args: {:#?}", args);
    }

    let dbfile = if let Some(path) = args.db {
        expand_tilde(path).unwrap()
    } else if let Ok(path) = std::env::var(ENV_DB_FILE) {
        expand_tilde(path).unwrap()
    } else {
        expand_tilde(DEFAULT_DB_FILE).unwrap()
    };

    let dir = dbfile.parent().unwrap();

    std::fs::create_dir_all(dir)?;

    // let conn = Connection::open_in_memory()?;
    let conn = Connection::open(&dbfile)?;

    conn.execute(
        &format!("CREATE TABLE IF NOT EXISTS {TABLE_NAME}(
            key text primary key,
            value blob
        )"),
        (),
    )?;

    match args.action {
        Action::Get(Get { ref key }) | Action::Qr(Get { ref key }) => {
            let mut stmt = conn.prepare(&format!("SELECT key,value FROM {TABLE_NAME} where key=?"))?;
            let entries = stmt.query_map([key], |row| Ok(row.get::<usize, Vec<u8>>(1)?))?;
            for value in entries {
                let value = value.unwrap();

                if args.verbose {
                    println!("> value: {:?}", value);
                }

                if let Action::Get(ref _g) = args.action {
                    std::io::stdout().write_all(&value).unwrap();
                    std::io::stdout().flush().unwrap();

                    print!("{}", if args.newline { "\n" } else { "" });
                } else {
                    qr2term::print_qr(&value)?;
                }
            }
        }
        Action::Set(Set { key, value }) => {
            let value = value.filter(|v| !v.is_empty());

            if let Some(value) = value {
                // INSERT INTO table (id, name, age) VALUES(1, "A", 19) ON DUPLICATE KEY UPDATE name="A", age=19
                conn.execute(
                    &format!("REPLACE INTO {TABLE_NAME}(key, value) VALUES(?1, ?2)"),
                    (&key, value.as_bytes()),
                )?;
            } else {
                conn.execute(&format!("DELETE FROM {TABLE_NAME} WHERE key = ?1"), (&key,))?;
            }
        }
        Action::Ls => {
            let mut stmt = conn.prepare(&format!("SELECT key,value FROM {TABLE_NAME}"))?;
            let entries = stmt.query_map([], |row| {
                Ok((row.get::<usize, String>(0)?, row.get::<usize, Vec<u8>>(1)?))
            })?;
            for value in entries {
                let (key, value) = value.unwrap();

                if args.verbose {
                    println!("> {key}: {:?}", value);
                }

                print!("{key}: ");
                std::io::stdout().write_all(&value).unwrap();
                std::io::stdout().flush().unwrap();

                println!();
            }
        }
        Action::Clear => {
            conn.execute(&format!("DROP TABLE IF EXISTS {TABLE_NAME}"), [])?;
        },
        
    }

    Ok(())
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

#[test]
fn test() {
    println!("current filename: {}", file!());

    let s = Some(String::from(""));
    let s = s.filter(|s| !s.is_empty());
    println!("s: {:#?}", s);
}
