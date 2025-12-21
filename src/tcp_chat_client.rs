use dioxus::core::spawn;
use dioxus_stores::Store;
use serde_json::Value;
use smol::io::{AsyncBufReadExt, BufReader};
use smol::net::TcpStream;
use std::collections::VecDeque;
use std::io::Error;
use std::sync::{Arc, Mutex};

use crate::arc_mutex_signal::AMSignal;
use crate::message_repository::Message;

enum MessageType {
    Error = -1,
    Status = 0,
    Chat = 1,
    JoinChannel = 2,
    ChangeTopic = 3,
    ListChannels = 4,
}

#[derive(Store)]
pub struct TcpChatClient {
    is_probably_connected: AMSignal<bool>,
    incoming: AMSignal<VecDeque<Message>>,
    outgoing: AMSignal<VecDeque<Message>>,
}

impl TcpChatClient {
    pub fn new() -> TcpChatClient {
        TcpChatClient {
            is_probably_connected: AMSignal::new(false),
            incoming: AMSignal::new(VecDeque::new()),
            outgoing: AMSignal::new(VecDeque::new()),
        }
    }

    pub fn set_probably_connected(&self, val: bool) {
        self.is_probably_connected.set(val);
    }

    pub fn is_probably_connected(&self) -> bool {
        self.is_probably_connected.get()
    }

    pub async fn connect(&self) -> Result<(), Error> {
        let addr = "127.0.0.1:10000";
        let mut stream = match TcpStream::connect(&addr).await {
            Err(err) => {
                println!("Failed to connect to {}", addr);
                self.set_probably_connected(false);
                return Err(err);
            }
            Ok(stream) => stream,
        };
        println!("Connected to {}", addr);
        self.set_probably_connected(true);

        let read = stream.clone();
        let read_handle = tokio::spawn(async move {
            let mut reader = BufReader::new(read);
            let mut buf = Vec::<u8>::new();
            loop {
                let mut str = String::new();
                match reader.read_line(&mut str).await {
                    Err(err) => {
                        println!("TcpChatClient read: error reading line");
                        break;
                    }
                    Ok(size) => {
                        if size == 0 {
                            break;
                        }

                        let data: Value = match serde_json::from_str(str.as_str()) {
                            Err(err) => {
                                println!("Unable to parse incoming data as json");
                                continue;
                            }
                            Ok(data) => data,
                        };
                        if data["id"] == MessageType::Chat as i32 {
                            println!("got chat message: {}", data["message"]);
                        } else {
                            println!("incoming (unhandled) data: {}", str);
                        }
                    }
                };
            }

            println!("TcpChatClient: stopping read loop");
        });
        let read_ipc = self.is_probably_connected.clone();
        spawn(async move {
            read_handle.await;
            read_ipc.set(false);
        });

        let write_ipc = self.is_probably_connected.clone();
        // tokio::spawn(async move {
        //     println!("TcpChatClient: stopping write loop");
        // });
        Ok(())
    }

    pub async fn close() {}
}
