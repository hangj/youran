use std::{
    cmp::max,
    io::{Read, Write},
    os::unix::prelude::OsStrExt,
};

mod config;
mod db;
use youran::auto_pad_str::*;

use config::*;

fn main() -> anyhow::Result<()> {
    let args = config::parse_args()?;

    let db = db::Db::from(args.db)?;

    match args.action {
        Action::Get(Get { key }) => {
            let value = db.get(&key)?;
            if args.verbose {
                println!("> value: {:#?}", value);
            }

            if let Some(value) = value {
                std::io::stdout().write_all(&value)?;
                std::io::stdout().flush()?;

                print!("{}", if args.newline { "\n" } else { "" });
            }
        }
        Action::Qr(Qr { key, output }) => {
            let value = db.get(&key)?;
            if args.verbose {
                println!("> value: {:#?}", value);
            }
            if value.is_none() {
                return Ok(());
            }
            let value = value.unwrap();

            if let Some(output) = output {
                use image::Luma;
                use qrcode::render::svg;
                use qrcode::QrCode;

                // Encode some data into bits.
                let code = QrCode::new(&value)?;

                if let Some(true) = output.extension().map(|o| o == "svg") {
                    let image = code
                        .render()
                        .min_dimensions(200, 200)
                        .dark_color(svg::Color("#000000"))
                        .light_color(svg::Color("#ffffff"))
                        .build();
                    let mut f = std::fs::File::create_new(output)?;
                    f.write_all(image.as_bytes())?;
                } else {
                    // Render the bits into an image.
                    let image = code.render::<Luma<u8>>().build();
                    image.save(output)?;
                }
            } else {
                qr2term::print_qr(&value)?;
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
                db.delete(&key)?;
            } else {
                db.set(&key, value)?;
            }
        }
        Action::Ls(ls) => {
            let entries = db.list(ls.limit, ls.offset)?;

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
            db.clear()?;
        }
    }

    Ok(())
}

#[test]
fn test() -> anyhow::Result<()> {
    let dbfile = "./test.db";

    let db = db::Db::from(Some(dbfile))?;
    db.clear()?;

    assert_eq!(db.get("hello")?, None);
    assert_eq!(db.set("hello", "world 😊".as_bytes())?, 1);
    assert_eq!(db.get("hello")?.unwrap(), "world 😊".as_bytes());
    db.list(10, 0)?;
    assert_eq!(db.delete("hello")?, 1);

    db.drop_database()?;

    Ok(())
}

#[test]
fn string_length_in_terminal() {
    let s1 = "hello 😊😊";
    let s2 = "hello word";

    println!("{:<1$}|", s1, s1.len());
    println!("{:<1$}|", s2, s1.len());

    println!(
        "{:<1$}|",
        s1.as_auto_pad_str(),
        s1.as_auto_pad_str().rendered_len()
    );
    println!(
        "{:<1$}|",
        s2.as_auto_pad_str(),
        s2.as_auto_pad_str().rendered_len()
    );
}
