use clap::Parser;
use env_logger::{Builder, Target};
use log::{LevelFilter, info};
use std::net::SocketAddrV4;

#[derive(Parser, Debug)]
#[clap(author = "flexice", version, about, long_about = None)]
struct Args {
    #[clap(long, help = "Choose another node to connect")]
    ctan: Option<SocketAddrV4>,

    #[clap(long)]
    verbose: bool,
}

/**

### Derive
[Open](https://github.com/clap-rs/clap/blob/v3.1.15/examples/derive_ref/README.md#arg-attributes)

### Builder
[Open](https://github.com/clap-rs/clap/blob/v3.1.15/examples/tutorial_builder/README.md)

*/
fn main() {
    let args = Args::parse();

    if args.verbose {
        Builder::new()
            .filter_level(LevelFilter::Debug)
            .target(Target::Stdout)
            .init();
    } else {
        Builder::new().target(Target::Stdout).init();
    }

    if args.ctan.is_some() {
        info!("Connecting to {}", args.ctan.unwrap());
    }
}
