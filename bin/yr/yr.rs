use anyhow::Result;
use std::{io::Write, os::unix::prelude::OsStrExt};

mod config;
mod db;

use config::*;

fn main() -> Result<()> {
    let args = config::parse_args()?;

    let db = db::Db::from(args.db)?;

    match args.action {
        Action::Get(Get { ref key }) | Action::Qr(Get { ref key }) => {
            let value = db.get(key)?;
            if args.verbose {
                println!("> value: {:#?}", value);
            }

            if let Some(value) = value {
                if let Action::Get(ref _g) = args.action {
                    std::io::stdout().write_all(&value)?;
                    std::io::stdout().flush()?;

                    print!("{}", if args.newline { "\n" } else { "" });
                } else {
                    qr2term::print_qr(&value)?;
                }
            }
        }
        Action::Set(Set { key, value }) => {
            if !value.is_empty() {
                db.set(&key, value.as_bytes())?;
            } else {
                db.delete(&key)?;
            }
        }
        Action::Ls(ls) => {
            let entries = db.list(ls.limit, ls.offset)?;

            for (key, value) in entries.iter() {
                if args.verbose {
                    println!("> {key}: {:?}", value);
                }

                print!("{key}: ");
                std::io::stdout().write_all(&value)?;
                std::io::stdout().flush()?;
                println!();
            }
        }
        Action::Clear => {
            db.clear()?;
        }
    }

    Ok(())
}



#[test]
fn test() -> Result<()> {
    let dbfile = "./test.db";

    let db = db::Db::from(Some(dbfile))?;
    db.clear()?;

    assert_eq!(db.get("hello")?, None);
    assert_eq!(db.set("hello", "world 😊".as_bytes())?, 1);
    assert_eq!(db.get("hello")?.unwrap(), "world 😊".as_bytes());
    assert_eq!(db.delete("hello")?, 1);

    db.drop_database()?;

    Ok(())
}
