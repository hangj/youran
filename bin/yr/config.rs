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
    Qr(Get),
    /// set the value of the given key
    Set(Set),
    /// list all the keys
    Ls(Ls),
    /// clear all the keys
    Clear,
}

#[derive(Debug, Parser)]
pub struct Get {
    pub key: String,
}

#[derive(Debug, Parser)]
pub struct Set {
    pub key: String,
    #[clap(value_parser, default_value = "", action(clap::ArgAction::Set))] // allow invalid utf8
    pub value: OsString,
}

#[derive(Debug, Parser)]
pub struct Ls {
    /// maxium items returned
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
