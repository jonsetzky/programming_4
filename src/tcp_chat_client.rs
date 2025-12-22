use chrono::{DateTime, Utc};
use dioxus_stores::Store;
use serde_json::Value;
use serde_json::json;
use smol::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use smol::net::TcpStream;
use std::io::Error;
use std::str::FromStr;
use std::sync::{Arc, Mutex};
use std::time::Duration;
use tokio::sync::broadcast;
use tokio::sync::broadcast::Receiver;
use tokio::sync::broadcast::Sender;
use tokio::task::JoinSet;
use uuid::Uuid;

use crate::arc_mutex_signal::AMSignal;
use crate::repository::Channel;
use crate::repository::Packet;
use crate::repository::PacketBuilder;
use crate::repository::PacketType;
use crate::repository::Repository;

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

    outgoing_tx: Sender<Packet>,
    incoming_tx: Sender<Packet>,

    repo: Arc<Mutex<dyn Repository + Send  + 'static>>,
    packet_builder: PacketBuilder,
}

impl TcpChatClient {
    pub fn new(packet_builder: PacketBuilder, repo: Arc<Mutex<dyn Repository + Send + 'static>>) -> TcpChatClient {
        let (otx, _) = broadcast::channel::<Packet>(512);
        let (itx, _) = broadcast::channel::<Packet>(512);

        TcpChatClient {
            is_probably_connected: AMSignal::new(false),
            should_run: AMSignal::new(true),

            outgoing_tx: otx,
            incoming_tx: itx,

            repo: repo.clone(),
            packet_builder
        }
    }

    pub fn set_probably_connected(&self, val: bool) {
        self.is_probably_connected.set(val);
    }

    pub fn is_probably_connected(&self) -> bool {
        self.is_probably_connected.get()
    }

    pub fn get_incoming_rx(&self) -> Receiver<Packet> {
        self.incoming_tx.subscribe()
    }

    async fn handle_packet(packet_builder: PacketBuilder, repo: Arc<Mutex<impl Repository + Send + 'static + ?Sized>>, incoming_tx: Sender<Packet>, outgoing_tx: Sender<Packet>, packet: Packet) {
        match packet.payload {
            PacketType::RequestChannels { known_channels } => {
                let my_channels: Vec<Channel>;
                {
                    let repo = repo.lock().unwrap();
                    my_channels = repo.get_channels();
                }
                let my_channel_ids = my_channels.iter().map(|c| c.id).collect::<Vec<Uuid>>();

                let new_channel_ids = known_channels.iter().filter(|kc| !my_channel_ids.contains(kc));
                let new_channels = new_channel_ids.map(|id| {
                    my_channels.iter().find(|c| c.id == *id).unwrap().clone()
                }).collect::<Vec<Channel>>();

                let _ = outgoing_tx.send(packet_builder.respond_channels(new_channels));
                return;
            },
            _ => {incoming_tx.send(packet)
            .expect("incoming thread: unable to send message to incoming channel");},
        }

    }

    /// Future finishes when connection ends.
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
        let outgoing_tx = self.outgoing_tx.clone();
        let repo_clone = self.repo.clone();
        let packet_builder = self.packet_builder;
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
                            let payload: PacketType = match serde_tran::from_base64(
                                data["message"].as_str().expect(
                                    "incoming thread: failed to parse message field as str",
                                ),
                            ) {
                                Err(_) => {
                                    println!(
                                        "incoming thread: Received packet with invalid payload data"
                                    );
                                    continue;
                                }
                                Ok(result) => result,
                            };
                            let packet = Packet {
                                id: Uuid::from_str(data["id"].as_str().expect(
                                    "incoming thread: unable to read packet's id field as str",
                                ))
                                .expect("incoming thread: unable to parse uuid of message"),
                                // todo! add parsing other that strings
                                payload,
                                reply_to: match data["inReplyTo"].as_str() {
                                    Some(str) => Some(Uuid::from_str(str).expect(
                                        "incoming thread: unable to parse uuid of message",
                                    )),
                                    None => None,
                                },
                                sender: Uuid::from_str(String::from(
                                    data["user"]
                                        .as_str()
                                        .expect("incoming thread: unable to read packet's user field as str"),
                                ).as_str()).expect("got invalid sender uuid"),
                                time: DateTime::<Utc>::from_timestamp(
                                    data["sent"]
                                        .as_i64()
                                        .expect("incoming thread: unable to read packet's sent field as u64"),
                                    0,
                                )
                                .expect("incoming thread: unable to parse sent field as datetime"),
                            };
                            
                            TcpChatClient::handle_packet(packet_builder, repo_clone.clone(), tx.clone(), outgoing_tx.clone(), packet).await;
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
        let mut rx: Receiver<Packet> = self.outgoing_tx.subscribe();
        set.spawn(async move {
            while should_run.get()
                && let Ok(msg) = rx.recv().await
            {
                let payload = match serde_tran::to_base64(&msg.payload) {
                    Err(err) => {
                        println!("outgoing thread: Failed to convert payload data to base64");
                        continue;
                    }
                    Ok(result) => result,
                };

                let _ = write
                    .write(
                        format!(
                            "{}\n",
                            json!({
                                "type": 1,
                                "id": msg.id,
                                "message": payload,
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

        let keepalive_tx = self.outgoing_tx.clone();
        let packet_builder = self.packet_builder;
        set.spawn(async move {
            loop {
                tokio::time::sleep(Duration::from_secs(30)).await;
                keepalive_tx.send(packet_builder.keepalive());
            }
        });

        let _ = set.join_next().await; // wait for one of the streams to finish
        self.is_probably_connected.set(false);
        self.should_run.set(false);
        set.abort_all(); // kill all threads
        Ok(())
    }

    pub fn send(&self, message: Packet) {
        self.outgoing_tx
            .send(message)
            .expect("failed to send message to mpsc channel");
    }
}
