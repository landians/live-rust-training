use clap::{Parser, ValueEnum};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

const DEFAULT_CAPACITY: usize = 128;

#[derive(Debug, Serialize, Deserialize)]
struct Player {
    #[serde(rename = "Name")]
    name: String,
    #[serde(rename = "Position")]
    position: String,
    #[serde(rename = "DOB")]
    dob: String,
    #[serde(rename = "Nationality")]
    nationality: String,
    #[serde(rename = "Kit Number")]
    kit_number: u8,
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

pub fn process_csv(opts: &CsvOpts) -> anyhow::Result<()> {
    let mut reader = csv::Reader::from_path(&opts.input)?;
    let mut players = Vec::with_capacity(DEFAULT_CAPACITY);

    for result in reader.deserialize() {
        let player: Player = result?;
        players.push(player);
    }

    let output = match &opts.output {
        Some(o) => o.clone(),
        None => format!("output.{:?}", &opts.format),
    };

    let contents = match opts.format {
        OutputFormat::Json => serde_json::to_string_pretty(&players)?,
        OutputFormat::Yaml => serde_yaml::to_string(&players)?,
    };

    fs::write(output, contents)?;

    Ok(())
}
