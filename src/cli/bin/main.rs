use std::net::Ipv4Addr;

use clap::Parser;

#[derive(Parser, Debug)]
#[clap(author = "flexice", version, about, long_about = None)]
struct Args {
    #[clap(long)]
    connect_to_another_server: Option<Ipv4Addr>,

    /// Number of times to greet
    #[clap(short, long, default_value_t = 1)]
    count: u8,
}

fn main() {
    let _args = Args::parse();
}