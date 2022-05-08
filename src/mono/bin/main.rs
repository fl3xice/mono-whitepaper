use mono_api::network::Network;
use std::{env, thread};
use std::net::{Ipv4Addr, SocketAddrV4};

fn main() {
    let args = env::args().collect::<Vec<_>>();
    let node: SocketAddrV4;

    // So far the first default host on localhost:3697
    if args.len() > 1 {
        node = args[1].to_string().parse().expect("Invalid node address");
    } else {
        node = SocketAddrV4::new(Ipv4Addr::new(127, 0, 0, 1), 3697);
    }

    let network: Network = Network::new(node);
    thread::spawn(move || { network.listen() });
    println!("Listening on {:?}", node);
    thread::park();
}
