use clap::Parser;
use rcli::{process_csv, process_pwd, Opts, SubCommand};

fn main() -> anyhow::Result<()> {
    let opts = Opts::parse();
    match opts.commands {
        SubCommand::Csv(opts) => process_csv(&opts)?,
        SubCommand::Pwd(opts) => process_pwd(&opts)?,
    }
    Ok(())
}
