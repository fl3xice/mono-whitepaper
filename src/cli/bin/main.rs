use clap::Parser;
use env_logger::{Builder, Target};
use log::{info, LevelFilter};
use std::{
    io::{Read, Write},
    net::{Ipv4Addr, SocketAddrV4, TcpStream},
};

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

    // So far the first default host on localhost:3697
    let mut node = SocketAddrV4::new(Ipv4Addr::new(127, 0, 0, 1), 3697);

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
        node = args.ctan.unwrap();
    }

    let mut stream = TcpStream::connect(node).unwrap();

    // buffer for write
    let mut buffer_w = String::new();

    stream.write(buffer_w.as_bytes()).unwrap();

    // buffer for read
    let mut buffer_r = [0; 4096];

    stream.read(&mut buffer_r).unwrap();

    // Print received data
    println!("{}", String::from_utf8_lossy(&buffer_r[..]));
}
