mod base64;
mod csv;
mod pwd;

pub use base64::*;
pub use csv::*;
pub use pwd::*;

use clap::{Parser, Subcommand};

#[derive(Debug, Parser)]
#[command(name = "rcli", version, author, about, long_about=None)]
pub struct Opts {
    #[command(subcommand)]
    pub commands: SubCommand,
}

#[derive(Debug, Subcommand)]
pub enum SubCommand {
    #[command(name = "csv", about = "Show csv or convert csv to other formats")]
    Csv(CsvOpts),

    #[command(name = "pwd", about = "Generate a rand password")]
    Pwd(PwdOpts),

    #[command(subcommand, about = "Base64 encode/decode")]
    Base64(Base64SubCommand),
}
