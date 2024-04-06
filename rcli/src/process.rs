use serde::{Deserialize, Serialize};
use std::fs;

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

pub fn process_csv(input: &str, output: &str) -> anyhow::Result<()> {
    let mut reader = csv::Reader::from_path(input)?;
    let mut players = Vec::with_capacity(128);
    for result in reader.deserialize() {
        let player: Player = result?;
        players.push(player)
    }

    let player_contents = serde_json::to_string_pretty(&players)?;
    fs::write(output, player_contents)?;

    Ok(())
}
