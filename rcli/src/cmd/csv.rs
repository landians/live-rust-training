use crate::cmd::opts::{CsvOpts, OutputFormat};
use serde::{Deserialize, Serialize};
use std::fs;

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
