use clap::Parser;
use std::path::PathBuf;
use strings2xml::strings_to_xml;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    input: PathBuf,
    #[arg(short, long)]
    output: PathBuf,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    let input = &args.input;
    let output = &args.output;
    strings_to_xml(input, output)
}
