pub mod file;

use file::create::*;
use file::read::*;
use file::recovery::*;

use std::io::Result;

use clap::Parser;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    file: String,
}

fn main() -> Result<()> {
    let args = Args::parse();

    match args.file.as_str() {
        "read" => read(),
        "create" => create(),
        "recovery" => recovery(),
        _ => Ok(()),
    }
}
