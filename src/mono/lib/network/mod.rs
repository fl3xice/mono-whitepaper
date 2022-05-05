use std::io::Read;
use std::net::SocketAddrV4;
use std::net::TcpListener;
use std::thread;

pub const BUFFER_SIZE: usize = 4096;
pub const END_CHAR: u8 = 0xE2; // ≃

pub struct Network {
    address: SocketAddrV4,
}

impl Network {
    pub fn new(address: SocketAddrV4) -> Self {
        Self { address }
    }

    pub fn listen(&self) {
        let listener = TcpListener::bind(self.address).unwrap_or_else(|err| panic!("{}", err));
        listener.set_ttl(64).expect("Could not set TTL");

        for stream in listener.incoming() {
            thread::spawn(move || {
                let mut stream = stream.unwrap();
                let mut buffer = [0; BUFFER_SIZE]; 

                // Read input data from stream
                stream.read(&mut buffer).unwrap();
            });
        }
    }

    pub fn address(&self) -> &SocketAddrV4 {
        &self.address
    }
}
