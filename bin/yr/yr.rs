
use std::{
    cmp::max,
    io::{Read, Write},
    os::unix::prelude::OsStrExt,
};

mod config;
mod db;
use youran::auto_pad_str::*;


use config::*;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = config::parse_args()?;

    let db = db::Db::from(args.db).await?;

    match args.action {
        Action::Get(Get { ref key }) | Action::Qr(Get { ref key }) => {
            let value = db.get(key).await?;
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
            let mut buf = Vec::new();
            let value = if let Some(ref value) = value {
                value.as_bytes()
            } else {
                // when the value is not provided, we read from stdin
                std::io::stdin().read_to_end(&mut buf)?;
                &buf
            };

            if value.is_empty() {
                db.delete(&key).await?;
            } else {
                db.set(&key, value).await?;
            }
        }
        Action::Ls(ls) => {
            let entries = db.list(ls.limit, ls.offset).await?;

            let alignment = entries.iter().fold(0, |acc, (key, _)| {
                max(key.as_auto_pad_str().rendered_len(), acc)
            }) + 4;

            for (key, value) in entries.iter() {
                if args.verbose {
                    println!("> {key}: {:?}", value);
                }

                // https://doc.rust-lang.org/std/fmt/index.html
                print!("{:<alignment$}", format!("{key}:").as_auto_pad_str());
                // print!("{:<alignment$}:  ", key.as_auto_pad_str());
                std::io::stdout().write_all(&value)?;
                std::io::stdout().flush()?;
                println!();
            }
        }
        Action::Clear => {
            db.clear().await?;
        }
    }

    Ok(())
}

#[tokio::test]
async fn test() -> anyhow::Result<()> {
    let dbfile = "./test.db";

    let db = db::Db::from(Some(dbfile)).await?;
    db.clear().await?;

    assert_eq!(db.get("hello").await?, None);
    assert_eq!(db.set("hello", "world 😊".as_bytes()).await?, 1);
    assert_eq!(db.get("hello").await?.unwrap(), "world 😊".as_bytes());
    db.list(10, 0).await?;
    assert_eq!(db.delete("hello").await?, 1);

    db.drop_database()?;

    Ok(())
}


#[test]
fn string_length_in_terminal() {
    let s1 = "hello 😊😊";
    let s2 = "hello word";

    println!("{:<1$}|", s1, s1.len());
    println!("{:<1$}|", s2, s1.len());

    println!("{:<1$}|", s1.as_auto_pad_str(), s1.as_auto_pad_str().rendered_len());
    println!("{:<1$}|", s2.as_auto_pad_str(), s2.as_auto_pad_str().rendered_len());
}

