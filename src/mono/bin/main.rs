use mono_api::network::Network;
use std::net::{Ipv4Addr, SocketAddrV4};

fn main() {
    let _network: Network = Network::new(SocketAddrV4::new(Ipv4Addr::new(127, 0, 0, 1), 3697));
    _network.listen();
}
