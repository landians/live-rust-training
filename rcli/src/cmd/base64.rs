use base64::prelude::*;
use clap::{Parser, Subcommand};

#[derive(Debug, Subcommand)]
pub enum Base64SubCommand {
    #[command(name = "encode", about = "base64 encode")]
    Encode(Base64EncodeOpts),

    #[command(name = "decode", about = "base64 decode")]
    Decode(Base64DecodeOpts),
}

#[derive(Debug, Parser)]
pub struct Base64EncodeOpts {
    #[arg(short, long)]
    pub input: String,
}

#[derive(Debug, Parser)]
pub struct Base64DecodeOpts {
    #[arg(short, long)]
    pub input: String,
}

pub fn process_base64_encode(opts: &Base64EncodeOpts) -> anyhow::Result<()> {
    let encode_value = BASE64_STANDARD.encode(&opts.input);
    println!("{}", encode_value);
    Ok(())
}

pub fn process_base64_decode(opts: &Base64DecodeOpts) -> anyhow::Result<()> {
    let decode_value = BASE64_STANDARD.decode(&opts.input)?;
    println!("{:?}", String::from_utf8(decode_value));
    Ok(())
}
