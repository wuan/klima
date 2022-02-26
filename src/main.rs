mod config;

use clap::Parser;


#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(global = true, long)]
    check: bool,
}

fn main() {
    let args = Args::parse();
    println!("{:?}", args);
}
