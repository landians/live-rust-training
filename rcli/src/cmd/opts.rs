use clap::{Parser, Subcommand, ValueEnum};
use std::path::Path;

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
}

#[derive(Debug, Parser)]
pub struct CsvOpts {
    #[arg(short, long, value_parser = validate_input_file)]
    pub input: String,

    #[arg(short, long)]
    pub output: Option<String>,

    #[arg(short, long, value_enum, default_value_t = OutputFormat::Json)]
    pub format: OutputFormat,

    #[arg(short, long, default_value_t = ',')]
    pub delimiter: char,

    #[arg(long, default_value_t = true)]
    pub header: bool,
}

#[derive(Debug, Clone, Copy, ValueEnum)]
pub enum OutputFormat {
    Json,
    Yaml,
}

fn validate_input_file(filename: &str) -> Result<String, &'static str> {
    if Path::new(filename).exists() {
        Ok(filename.into())
    } else {
        Err("File is not exists")
    }
}

#[derive(Debug, Parser)]
pub struct PwdOpts {
    #[arg(long, default_value_t = true)]
    pub upper: bool,

    #[arg(long, default_value_t = true)]
    pub lower: bool,

    #[arg(long, default_value_t = true)]
    pub number: bool,

    #[arg(long, default_value_t = true)]
    pub symbol: bool,

    #[arg(short, long, default_value_t = 6)]
    pub length: u8,
}
