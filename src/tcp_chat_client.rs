use smol::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use smol::net::TcpStream;
use std::io;

use crate::packet::Packet;

pub struct TcpChatClient {
    stream: TcpStream,
    reader: BufReader<TcpStream>,
}

impl TcpChatClient {
    pub fn clone(&self) -> TcpChatClient {
        TcpChatClient {
            stream: self.stream.clone(),
            reader: BufReader::new(self.stream.clone()),
        }
    }

    pub async fn connect(addr: Option<&str>) -> io::Result<TcpChatClient> {
        let addr = addr.unwrap_or("127.0.0.1:10000");

        match TcpStream::connect(&addr).await {
            Err(err) => {
                println!("Failed to connect to {}", addr);
                return Err(err);
            }
            Ok(stream) => Ok(TcpChatClient {
                reader: BufReader::new(stream.clone()),
                stream,
            }),
        }
    }

    pub async fn send(&self, packet: Packet) -> io::Result<usize> {
        let mut data = packet.into_bytes();
        data.push('\n' as u8);

        // println!("send(): {}", String::from_utf8(data.clone()).unwrap());

        self.stream.clone().write(data.as_slice()).await
    }
    pub async fn recv(&mut self) -> io::Result<Packet> {
        let mut str = String::new();
        match self.reader.read_line(&mut str).await {
            Err(err) => {
                println!("TcpChatClient read: error reading line");
                Err(err)
            }
            Ok(size) => {
                if size == 0 {
                    return Err(std::io::ErrorKind::ConnectionAborted.into());
                }
                // println!("recv(): {}", str.as_str());
                Ok(Packet::from_bytes(str.as_bytes()))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::packet_builder::PacketBuilder;

    use super::*;

    #[tokio::test]
    async fn it_connects() {
        match TcpChatClient::connect(Some("127.0.0.1:10000")).await {
            Err(err) => panic!("{err}"),
            Ok(_client) => {
                assert!(true);
            }
        }
    }
    #[tokio::test]
    async fn it_can_list_channels() {
        let packet_builder = PacketBuilder::new("test user".into());
        match TcpChatClient::connect(Some("127.0.0.1:10000")).await {
            Err(err) => panic!("{err}"),
            Ok(mut client) => {
                let _status_msg = client.recv().await.expect("failed with recv()");
                let _topic_msg = client.recv().await.expect("failed with recv()");

                let bytes_written = client
                    .send(packet_builder.list_channels())
                    .await
                    .expect("failed to send");
                assert!(bytes_written > 0);

                let channels_msg = client.recv().await.expect("failed with recv()");
                match channels_msg {
                    Packet::ListChannels { channels } => {
                        assert!(channels.unwrap().len() == 2);
                    }
                    _ => panic!("didn't get channels message as expected"),
                }
            }
        }
    }
}
