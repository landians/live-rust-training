use clap::Parser;
use rcli::{
    process_base64_decode, process_base64_encode, process_csv, process_pwd, Base64SubCommand, Opts,
    SubCommand,
};

// cargo run --bin rcli base64 encode -i 4932749237
fn main() -> anyhow::Result<()> {
    let opts = Opts::parse();
    match opts.commands {
        SubCommand::Csv(opts) => process_csv(&opts)?,
        SubCommand::Pwd(opts) => process_pwd(&opts)?,
        SubCommand::Base64(cmds) => match cmds {
            Base64SubCommand::Encode(opts) => process_base64_encode(&opts)?,
            Base64SubCommand::Decode(opts) => process_base64_decode(&opts)?,
        },
    }
    Ok(())
}
