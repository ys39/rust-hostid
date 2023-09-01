use anyhow::Result;
use clap::Parser;
use libc::gethostid;
use std::io::{self, Write};

#[derive(Parser, Debug)]
#[command(
    author,
    version,
    about="Displays the identification number (in hexadecimal) of the current host.",
    long_about = None
)]
struct Cli {}

fn main() -> Result<()> {
    let _args: Cli = Cli::parse();

    let hostid = unsafe { gethostid() };
    let masked_hostid = hostid & 0xffffffff;
    writeln!(io::stdout(), "{:08x}", masked_hostid)?;

    Ok(())
}
