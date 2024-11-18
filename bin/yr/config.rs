use std::{ffi::OsString, path::PathBuf};

use clap::{Error, Parser};

#[derive(Debug, Parser)]
#[clap(name="yr", author, version, about, long_about = None)]
pub struct Args {
    /// give more verbose output
    #[clap(short, long, value_parser, default_value_t = false)]
    pub verbose: bool,
    /// specify database file, use ~/.config/youran/db.sqlite by default if not set environment variable YOURAN_DB_FILE
    #[clap(long, value_parser)]
    pub db: Option<PathBuf>,
    /// Do not print the trailing newline character
    #[clap(
        short,
        long,
        value_parser,
        default_value_t = true,
        action(clap::ArgAction::SetFalse)
    )]
    pub newline: bool,
    #[clap(subcommand)]
    pub action: Action,
}

#[derive(clap::Subcommand, Debug)]
pub enum Action {
    /// get the value of the given key
    Get(Get),
    /// show the QRCode for the given key
    Qr(Qr),
    /// set the value of the given key
    Set(Set),
    /// list the latest updated key-values
    Ls(Ls),
    /// clear all the keys, empty the table
    Clear,
}

#[derive(Debug, Parser)]
pub struct Get {
    pub key: String,
}

#[derive(Debug, Parser)]
pub struct Qr {
    pub key: String,
    /// print qrcode to the terminal if not set, Or save it to output file, the extension of output
    /// should be an image type(.png) or .svg
    #[clap(short, long, value_parser, action(clap::ArgAction::Set))]
    pub output: Option<std::path::PathBuf>,
}

#[derive(Debug, Parser)]
pub struct Set {
    pub key: String,
    /// read from stdin if not provided.
    /// the key will be deleted if value is empty.
    #[clap(value_parser, action(clap::ArgAction::Set))] // allow invalid utf8
    pub value: Option<OsString>,
}

#[derive(Debug, Parser)]
pub struct Ls {
    /// maxium records returned
    #[clap(long, value_parser, default_value_t=10, action = clap::ArgAction::Set)]
    pub limit: usize,
    /// offset
    #[clap(long, value_parser, default_value_t=0, action = clap::ArgAction::Set)]
    pub offset: usize,
}

pub fn parse_args() -> Result<Args, Error> {
    // let args_os: Vec<_> = std::env::args_os().collect();
    // println!("> args_os: {:#?}", args);

    let args = Args::parse();

    if args.verbose {
        println!("> args: {:#?}", args);
    }
    Ok(args)
}
