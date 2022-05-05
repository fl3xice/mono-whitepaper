use std::io::Read;
use std::net::SocketAddrV4;
use std::net::TcpListener;
use std::thread;

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
                let mut buffer = [0; 4096]; 

                // Read input data from stream
                stream.read(&mut buffer).unwrap();
                
                println!("{}", String::from_utf8_lossy(&buffer[..]));
            });
        }
    }

    pub fn address(&self) -> &SocketAddrV4 {
        &self.address
    }
}
