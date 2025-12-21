use chrono::{DateTime, Utc};
use dioxus::core::spawn;
use dioxus_stores::Store;
use serde_json::Value;
use smol::io::{AsyncBufReadExt, BufReader};
use smol::net::TcpStream;
use std::collections::VecDeque;
use std::io::Error;
use std::rc::Rc;
use std::str::FromStr;
use std::time::Duration;
use tokio::task::{JoinHandle, JoinSet};
use uuid::Uuid;

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
    should_run: AMSignal<bool>,
    incoming: AMSignal<VecDeque<Message>>,
    outgoing: AMSignal<VecDeque<Message>>,
}

impl TcpChatClient {
    pub fn new() -> TcpChatClient {
        TcpChatClient {
            is_probably_connected: AMSignal::new(false),
            should_run: AMSignal::new(true),
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
        self.should_run.set(true);

        let mut set = JoinSet::new();

        let read = stream.clone();
        let should_run = self.should_run.clone();
        set.spawn(async move {
            let mut reader = BufReader::new(read);
            let mut buf = Vec::<u8>::new();
            while should_run.get() {
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
                        if data["type"] == MessageType::Chat as i32 {
                            let message = Message {
                                id: Uuid::from_str(
                                    data["id"]
                                        .as_str()
                                        .expect("unable to read packet's id field as str"),
                                )
                                .expect("unable to parse uuid of message"),
                                message: String::from(
                                    data["message"]
                                        .as_str()
                                        .expect("unable to read packet's message field as str"),
                                ),
                                reply_to: match data["inReplyTo"].as_str() {
                                    Some(str) => Some(
                                        Uuid::from_str(str)
                                            .expect("unable to parse uuid of message"),
                                    ),
                                    None => None,
                                },
                                sender: String::from(
                                    data["user"]
                                        .as_str()
                                        .expect("unable to read packet's user field as str"),
                                ),
                                time: DateTime::<Utc>::from_timestamp(
                                    data["sent"]
                                        .as_i64()
                                        .expect("unable to read packet's sent field as u64"),
                                    0,
                                )
                                .expect("unable to parse sent field as datetime"),
                            };

                            println!("got chat message: {}", message.message);
                        } else {
                            println!("incoming (unhandled) data: {}", str);
                        }
                    }
                };
            }

            println!("TcpChatClient: stopping read loop");
        });
        // let read_ipc = self.is_probably_connected.clone();

        // let write_ipc = self.is_probably_connected.clone();
        let should_run = self.should_run.clone();
        set.spawn(async move {
            println!("TcpChatClient: stopping write loop");
            while should_run.get() {
                tokio::time::sleep(Duration::from_secs(1));
            }
        });

        let _ = set.join_next().await; // wait for one of the streams to finish
        self.is_probably_connected.set(false);
        self.should_run.set(false);

        set.join_all().await;
        Ok(())
    }

    pub async fn close() {}
}
