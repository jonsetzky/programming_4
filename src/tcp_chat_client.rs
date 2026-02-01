use chrono::{DateTime, Utc};
use dioxus_desktop::wry::cookie::time::UtcDateTime;
use dioxus_stores::Store;
use serde_json::{Value, json};
use serde_tran::ErrorKind;
use smol::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use smol::net::TcpStream;
use std::collections::HashMap;
use std::io::{self, Error};
use std::str::FromStr;
use std::sync::{Arc, Mutex};
use std::time::Duration;
use std::time::SystemTime;
use tokio::sync::broadcast;
use tokio::sync::broadcast::Receiver;
use tokio::sync::broadcast::Sender;
use tokio::task::JoinSet;
use uuid::Uuid;

use crate::arc_mutex_signal::AMSignal;
use crate::packet::Packet;

pub struct TcpChatClient {
    stream: TcpStream,
    reader: BufReader<TcpStream>,
}

impl TcpChatClient {
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
    use regex::Regex;
    use std::time::SystemTime;

    use crate::packet_builder::PacketBuilder;

    use super::*;

    #[tokio::test]
    async fn it_connects() {
        match TcpChatClient::connect(Some("127.0.0.1:10000")).await {
            Err(err) => panic!("{err}"),
            Ok(client) => {
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
// #[derive(Store)]
// pub struct TcpChatClient {
//     is_probably_connected: AMSignal<bool>,
//     should_run: AMSignal<bool>,

//     outgoing_tx: Sender<Packet>,
//     incoming_tx: Sender<Packet>,

//     repo: Arc<Mutex<dyn Repository + Send  + 'static>>,
//     packet_builder: PacketBuilder,

//     other_clients: Arc<Mutex<HashMap<Uuid, SystemTime>>>,
// }

// impl TcpChatClient {
//     pub fn new(packet_builder: PacketBuilder, repo: Arc<Mutex<dyn Repository + Send + 'static>>) -> TcpChatClient {
//         let (otx, _) = broadcast::channel::<Packet>(512);
//         let (itx, _) = broadcast::channel::<Packet>(512);

//         TcpChatClient {
//             is_probably_connected: AMSignal::new(false),
//             should_run: AMSignal::new(true),

//             outgoing_tx: otx,
//             incoming_tx: itx,

//             repo: repo.clone(),
//             packet_builder,

//             other_clients: Arc::new(Mutex::new(HashMap::<Uuid, SystemTime>::new())),
//         }
//     }

//     pub fn set_probably_connected(&self, val: bool) {
//         self.is_probably_connected.set(val);
//     }

//     pub fn is_probably_connected(&self) -> bool {
//         self.is_probably_connected.get()
//     }

//     pub fn get_incoming_rx(&self) -> Receiver<Packet> {
//         self.incoming_tx.subscribe()
//     }

//     async fn handle_packet(
//         packet_builder: PacketBuilder,
//         repo: Arc<Mutex<impl Repository + Send + 'static + ?Sized>>,
//         incoming_tx: Sender<Packet>,
//         outgoing_tx: Sender<Packet>,
//         other_clients: Arc<Mutex<HashMap<Uuid, SystemTime>>>,
//         packet: Packet) {
//         match packet.payload {
//             PacketType::KeepAlive => {
//                 let mut ocs = other_clients.lock().unwrap();
//                 ocs.insert(packet.sender, SystemTime::now());
//             },
//             PacketType::Hello {channels_checksum, nickname } => {
//                 print!("{} said hello! their channels are {}. ", nickname, channels_checksum);

//                 {
//                     let mut ocs = other_clients.lock().unwrap();
//                     ocs.insert(packet.sender, SystemTime::now());
//                 }

//                 if packet.recipient.is_none() {
//                     println!("saying hello back...");
//                     let resp = packet_builder.hello(repo.lock().unwrap().get_channels_checksum(), Some(packet.sender));
//                     let _ = outgoing_tx.send(resp);
//                 } else if channels_checksum != repo.lock().unwrap().get_channels_checksum() {
//                     // all connected users are expected to have synced their channels so ask the first one for a channel update
//                     let known_channels: Vec<Uuid>;
//                     {
//                         known_channels = repo.lock().unwrap().get_channels_uuids();
//                     }

//                     let _ = outgoing_tx.send(packet_builder.request_channels(known_channels, true, packet.sender));
//                     println!("requesting their channels...");
//                 } else {
//                     println!();
//                 }

//             },
//             PacketType::RequestChannels { known_channels, request_back } => {
//                 let my_channels: Vec<Channel>;
//                 {
//                     let repo = repo.lock().unwrap();
//                     my_channels = repo.get_channels();
//                 }
//                 let my_channel_ids = my_channels.iter().map(|c| c.id).collect::<Vec<Uuid>>();

//                 let new_channel_ids = my_channel_ids.iter().filter(|kc| !known_channels.contains(kc));
//                 let new_channels = new_channel_ids.map(|id| {
//                     my_channels.iter().find(|c| c.id == *id).unwrap().clone()
//                 }).collect::<Vec<Channel>>();

//                 println!("sent {} channels to public", new_channels.len());
//                 let _ = outgoing_tx.send(packet_builder.respond_channels(new_channels));

//                 if request_back {
//                     let known_channels: Vec<Uuid>;
//                     {
//                         known_channels = repo.lock().unwrap().get_channels_uuids();
//                     }
//                     let _ = outgoing_tx.send(packet_builder.request_channels(known_channels, false, packet.sender));
//                 }

//                 return;
//             },
//             PacketType::RespondChannels { new_channels } => {
//                 let my_channels: Vec<Channel>;
//                 {
//                     let repo = repo.lock().unwrap();
//                     my_channels = repo.get_channels();
//                 }
//                 let my_channel_ids = my_channels.iter().map(|c| c.id).collect::<Vec<Uuid>>();

//                 let mut channels_to_add = Vec::<Channel>::new();
//                 for channel in new_channels {
//                     // todo add handling for name collision
//                     //    it would require updating local as well as remote databases.

//                     // skip known channels
//                     if my_channel_ids.contains(&channel.id) {
//                         continue;
//                     }

//                     channels_to_add.push(channel.clone());
//                 }

//                 println!("received {} channels from public", channels_to_add.len());
//                 {
//                     let repo = repo.lock().unwrap();
//                     repo.add_channels(channels_to_add);
//                 }
//                 return;
//             }
//             _ => {
//                 incoming_tx.send(packet)
//                 .expect("incoming thread: unable to send message to incoming channel");
//             },
//         }

//     }

//     /// Future finishes when connection ends.
//     pub async fn connect(&self) -> Result<(), Error> {
//         let addr = "127.0.0.1:10000";
//         let stream = match TcpStream::connect(&addr).await {
//             Err(err) => {
//                 println!("Failed to connect to {}", addr);
//                 self.set_probably_connected(false);
//                 return Err(err);
//             }
//             Ok(stream) => stream,
//         };
//         println!("Connected to {}", addr);
//         self.set_probably_connected(true);
//         self.should_run.set(true);

//         let mut set = JoinSet::new();

//         let read = stream.clone();
//         let should_run = self.should_run.clone();
//         let tx = self.incoming_tx.clone();
//         let outgoing_tx = self.outgoing_tx.clone();
//         let repo_clone = self.repo.clone();
//         let packet_builder = self.packet_builder.clone();
//         let other_clients = self.other_clients.clone();
//         set.spawn(async move {
//             let mut reader = BufReader::new(read);
//             while should_run.get() {
//                 let mut str = String::new();
//                 match reader.read_line(&mut str).await {
//                     Err(_) => {
//                         println!("TcpChatClient read: error reading line");
//                         break;
//                     }
//                     Ok(size) => {
//                         if size == 0 {
//                             break;
//                         }

//                             let data: Value = match serde_json::from_str(str.as_str()) {
//                             Err(_) => {
//                                 println!("Unable to parse incoming data as json");
//                                 continue;
//                             }
//                             Ok(data) => data,
//                         };
//                         if data["type"] == MessageType::Chat as i32 {
//                             let payload: PacketType = match serde_tran::from_base64(
//                                 data["message"].as_str().expect(
//                                     "incoming thread: failed to parse message field as str",
//                                 ),
//                             ) {
//                                 Err(_) => {
//                                     println!(
//                                         "incoming thread: Received packet with invalid payload data"
//                                     );
//                                     continue;
//                                 }
//                                 Ok(result) => result,
//                             };
//                             let mut packet = Packet {
//                                 id: Uuid::from_str(data["id"].as_str().expect(
//                                     "incoming thread: unable to read packet's id field as str",
//                                 ))
//                                 .expect("incoming thread: unable to parse uuid of message"),
//                                 payload,
//                                 reply_to: match data["inReplyTo"].as_str() {
//                                     Some(str) => Some(Uuid::from_str(str).expect(
//                                         "incoming thread: unable to parse uuid of message",
//                                     )),
//                                     None => None,
//                                 },
//                                 sender: Uuid::from_str(String::from(
//                                     data["user"]
//                                         .as_str()
//                                         .expect("incoming thread: unable to read packet's user field as str"),
//                                 ).as_str()).expect("got invalid sender uuid"),
//                                 time: DateTime::<Utc>::from_timestamp(
//                                     data["sent"]
//                                         .as_i64()
//                                         .expect("incoming thread: unable to read packet's sent field as u64"),
//                                     0,
//                                 )
//                                 .expect("incoming thread: unable to parse sent field as datetime"),
//                                 recipient: None
//                             };

//                             if let Some(recipient) = data["directMessageTo"].as_str() {
//                                 packet.recipient = Some(Uuid::from_str(recipient).expect("incoming thread: unable to parse uuid of message"));
//                             }

//                             TcpChatClient::handle_packet(packet_builder.clone(), repo_clone.clone(), tx.clone(), outgoing_tx.clone(), other_clients.clone(), packet).await;
//                         } else {
//                             println!("incoming (unhandled) data: {}", str);
//                         }
//                     }
//                 };
//             }

//             println!("TcpChatClient: stopping read loop");
//         });
//         let mut write = stream.clone();
//         let should_run = self.should_run.clone();
//         let mut rx: Receiver<Packet> = self.outgoing_tx.subscribe();
//         set.spawn(async move {
//             while should_run.get()
//                 && let Ok(msg) = rx.recv().await
//             {
//                 let Ok(data) = msg.into_json() else {
//                     continue;
//                 };

//                 let _ = write
//                     .write(format!( "{data}\n").as_bytes(),
//                     )
//                     .await;
//             }

//             println!("TcpChatClient: stopping write loop");
//         });

//         let outgoing_tx = self.outgoing_tx.clone();
//         let packet_builder = self.packet_builder.clone();
//         let repo_clone = self.repo.clone();
//         let other_clients = self.other_clients.clone();
//         set.spawn(async move {
//             {
//                 let _ = outgoing_tx.send(packet_builder.hello(repo_clone.lock().unwrap().get_channels_checksum(), None));
//             }

//             loop {
//                 tokio::time::sleep(Duration::from_secs(30)).await;
//                 let _ = outgoing_tx.send(packet_builder.keepalive());

//                 // remove stale clients
//                 let now = SystemTime::now();
//                 let mut ocs = other_clients.lock().unwrap();
//                 ocs
//                     .iter()
//                     .filter_map(|(uuid, last_keepalive)| {
//                         if *last_keepalive + Duration::from_mins(1) <= now {
//                             return Some(uuid.clone())
//                         }
//                         return None
//                     }).collect::<Vec<Uuid>>()
//                     .iter()
//                     .for_each(|uuid| {
//                         ocs.remove(&uuid);
//                     });
//             }
//         });

//         let _ = set.join_next().await; // wait for one of the streams to finish
//         self.is_probably_connected.set(false);
//         self.should_run.set(false);
//         set.abort_all(); // kill all threads
//         Ok(())
//     }

//     pub fn send(&self, message: Packet) {
//         self.outgoing_tx
//             .send(message)
//             .expect("failed to send message to mpsc channel");
//     }
// }
