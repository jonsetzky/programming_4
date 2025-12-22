use chrono::{DateTime, Utc};
use dioxus_stores::Store;
use serde_json::Value;
use serde_json::json;
use smol::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use smol::net::TcpStream;
use std::io::Error;
use std::str::FromStr;
use tokio::sync::broadcast;
use tokio::sync::broadcast::Receiver;
use tokio::sync::broadcast::Sender;
use tokio::task::JoinSet;
use uuid::Uuid;

use crate::arc_mutex_signal::AMSignal;
use crate::repository::Message;

enum MessageType {
    // Error = -1,
    // Status = 0,
    Chat = 1,
    // JoinChannel = 2,
    // ChangeTopic = 3,
    // ListChannels = 4,
}

#[derive(Store)]
pub struct TcpChatClient {
    is_probably_connected: AMSignal<bool>,
    should_run: AMSignal<bool>,

    outgoing_tx: Sender<Message>,
    incoming_tx: Sender<Message>,
}

impl TcpChatClient {
    pub fn new() -> TcpChatClient {
        let (otx, _) = broadcast::channel::<Message>(512);
        let (itx, _) = broadcast::channel::<Message>(512);

        TcpChatClient {
            is_probably_connected: AMSignal::new(false),
            should_run: AMSignal::new(true),

            outgoing_tx: otx,
            incoming_tx: itx,
        }
    }

    pub fn set_probably_connected(&self, val: bool) {
        self.is_probably_connected.set(val);
    }

    pub fn is_probably_connected(&self) -> bool {
        self.is_probably_connected.get()
    }

    pub fn get_incoming_rx(&self) -> Receiver<Message> {
        self.incoming_tx.subscribe()
    }

    pub async fn connect(&self) -> Result<(), Error> {
        let addr = "127.0.0.1:10000";
        let stream = match TcpStream::connect(&addr).await {
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
        let tx = self.incoming_tx.clone();
        set.spawn(async move {
            let mut reader = BufReader::new(read);
            while should_run.get() {
                let mut str = String::new();
                match reader.read_line(&mut str).await {
                    Err(_) => {
                        println!("TcpChatClient read: error reading line");
                        break;
                    }
                    Ok(size) => {
                        if size == 0 {
                            break;
                        }

                        let data: Value = match serde_json::from_str(str.as_str()) {
                            Err(_) => {
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

                            tx.send(message)
                                .expect("unable to send message to incoming channel");
                        } else {
                            println!("incoming (unhandled) data: {}", str);
                        }
                    }
                };
            }

            println!("TcpChatClient: stopping read loop");
        });
        let mut write = stream.clone();
        let should_run = self.should_run.clone();
        let mut rx: Receiver<Message> = self.outgoing_tx.subscribe();
        set.spawn(async move {
            while should_run.get()
                && let Ok(msg) = rx.recv().await
            {
                let _ = write
                    .write(
                        format!(
                            "{}\n",
                            json!({
                                "type": 1,
                                "id": msg.id,
                                "message": msg.message,
                                "user": msg.sender,
                                "sent": msg.time.timestamp()
                            })
                            .to_string()
                        )
                        .as_bytes(),
                    )
                    .await;
            }

            println!("TcpChatClient: stopping write loop");
        });

        let _ = set.join_next().await; // wait for one of the streams to finish
        self.is_probably_connected.set(false);
        self.should_run.set(false);
        set.abort_all();
        Ok(())
    }

    pub fn send(&self, message: Message) {
        self.outgoing_tx
            .send(message)
            .expect("failed to send message to mpsc channel");
    }
}
