use std::path::PathBuf;
use clap::Parser;

#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    name: String,

    #[arg(short, long, value_name = "CONFIG_PATH")]
    config: Option<PathBuf>,

    #[arg(short)]
    debug: Option<u8>
}

fn main() {
    let args = Args::parse();

    if let _is_name_exist = args.name.len() > 0 {
        println!("Hello, {}!", args.name);
    }
}