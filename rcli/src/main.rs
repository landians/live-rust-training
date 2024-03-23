use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::fs::File;

#[allow(dead_code)]
#[derive(Debug, Serialize, Deserialize)]
pub struct Player {
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

impl Player {
    pub fn to_json(&self) -> Result<String> {
        let value = serde_json::to_string(&self)?;
        Ok(value)
    }

    pub fn from_json(value: String) -> Result<Self> {
        let player: Player = serde_json::from_str(&value)?;
        Ok(player)
    }
}

fn main() -> Result<()> {
    let fd = File::open("assets/juventus.csv")?;
    let mut reader = csv::Reader::from_reader(fd);
    for result in reader.deserialize() {
        let player: Player = result?;
        println!("{:?}", player.to_json())
    }
    Ok(())
}
