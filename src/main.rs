use clap::Parser;

struct Config {
    host: String,
    port: u16,
    config: String,
    debug: u8,
}

impl Config {
    fn new() -> Self {
        Self {
            host: "localhost".to_owned(),
            port: 8888,
            config: "".to_string(),
            debug: 0
        }
    }

    fn set_args_val(&mut self, args: &Args) -> &mut Self {
        match &args.host {
            Some(v) => self.host = v.to_string(),
            _ => {}
        }
        match &args.port {
            Some(v) => self.port = *v,
            _ => {}
        }
        match &args.config {
            Some(v) => self.config = v.to_string(),
            _ => {}
        }
        match &args.debug {
            Some(v) => self.debug = *v,
            _ => {}
        }
        self
    }
}

#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(long)]
    host: Option<String>,

    #[arg(long)]
    port: Option<u16>,

    #[arg(short, long, value_name = "CONFIG_PATH")]
    config: Option<String>,

    #[arg(short)]
    debug: Option<u8>
}

fn main() {
    let args = Args::parse();

    let mut config = Config::new();

    config.set_args_val(&args);

    println!("{}:{}", config.host, config.port);
}