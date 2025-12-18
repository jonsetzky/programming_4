use std::{
    io::Error,
    net::SocketAddr,
    result::Result,
    str::{self, FromStr},
};

use smol::net;

// todo use these
#[allow(dead_code)]
enum MessageType {
    Error = -1,
    Status = 0,
    Chat = 1,
    JoinChannel = 2,
    ChangeTopic = 3,
    ListChannels = 4,
}

pub struct O4ChatClient {
    addr: SocketAddr,
    stream: Option<net::TcpStream>,
}

impl O4ChatClient {
    pub fn new(addr: Option<&str>) -> Result<O4ChatClient, std::io::Error> {
        let addr = addr.unwrap_or("127.0.0.1:10000");
        let addr = SocketAddr::from_str(addr).expect("Error parsing socket address");

        Ok(O4ChatClient { addr, stream: None })
    }

    pub async fn connect(&mut self) -> Result<(), Error> {
        assert!(
            self.stream.is_none(),
            "Attempting to connect while client is already connected"
        );
        match net::TcpStream::connect(self.addr).await {
            Err(_err) => Err(_err),
            Ok(_stream) => {
                match _stream.peer_addr() {
                    Ok(addr) => println!("Connected to {}", addr),
                    Err(err) => println!("Error getting peer address {}", err),
                };

                self.stream = Some(_stream);
                Ok(())
            }
        }
    }

    pub fn disconnect(&mut self) -> Result<(), Error> {
        if let Some(stream) = &self.stream {
            println!("Disconnected");

            return match stream.shutdown(std::net::Shutdown::Both) {
                Ok(()) => {
                    println!("Disconnected from {}", self.addr);
                    self.stream = None;
                    Ok(())
                }
                Err(err) => Err(err),
            };
        }
        Ok(())
    }

    pub fn is_connected(&self) -> bool {
        self.stream.is_some()
    }
}
