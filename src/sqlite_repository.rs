use ::num::clamp;
use diesel::prelude::*;
use diesel_migrations::{EmbeddedMigrations, MigrationHarness, embed_migrations};
use dotenvy::dotenv;
use std::{cell::RefCell, env, path::PathBuf};
use uuid::Uuid;

use crate::{
    models::{ChannelModel, MessageModel, UuidWrapper},
    repository::{Channel, Message, Repository},
};

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("./migrations");

pub fn run_migrations(conn: &mut SqliteConnection) {
    match conn.run_pending_migrations(MIGRATIONS) {
        Ok(_) => println!("successfully applied migrations"),
        Err(err) => println!("failed to run migrations: {}", err),
    };
}

pub fn establish_connection() -> SqliteConnection {
    dotenv().ok();

    let mut path = PathBuf::from(neighbor_chat::data_dir());
    path.push(env::var("DATABASE_URL").expect("DATABASE_URL must be set"));

    let path = path.to_str().unwrap();

    SqliteConnection::establish(&path).unwrap_or_else(|_| panic!("Error connecting to {}", path))
}

pub struct SqliteRepository {
    conn: RefCell<SqliteConnection>,
}

impl SqliteRepository {
    pub fn new() -> SqliteRepository {
        SqliteRepository {
            conn: RefCell::new(establish_connection()),
        }
    }
}

use crate::schema::channels::dsl::*;
use crate::schema::messages::dsl::*;
impl Repository for SqliteRepository {
    fn add_channels(&self, chls: Vec<Channel>) {
        let mut conn = self.conn.borrow_mut();
        let chls: Vec<ChannelModel> = chls.into_iter().map(ChannelModel::from).collect();

        let _ = diesel::insert_into(channels)
            .values(&chls)
            .execute(&mut conn as &mut SqliteConnection);
    }
    fn get_channels(&self) -> Vec<Channel> {
        let mut conn = self.conn.borrow_mut();
        let results = channels
            .select(ChannelModel::as_select())
            .load(&mut conn as &mut SqliteConnection)
            .expect("failed to get channels from db");

        results.into_iter().map(Channel::from).collect()
    }
    fn get_channels_checksum(&self) -> u32 {
        let chls = self.get_channels();
        let mut hasher = crc32fast::Hasher::new();
        for chl in chls {
            hasher.update(chl.id.as_bytes());
        }
        hasher.finalize()
    }
    fn get_channels_uuids(&self) -> Vec<Uuid> {
        self.get_channels().into_iter().map(|c| c.id).collect()
    }
    fn add_message(&self, msg: Message) {
        let mut conn = self.conn.borrow_mut();

        match diesel::insert_into(messages)
            .values(MessageModel::from(msg))
            .execute(&mut conn as &mut SqliteConnection)
        {
            Ok(_) => println!("successfully added message"),
            Err(err) => println!("failed to add message"),
        };
    }
    #[allow(unused_variables)]
    fn get_message_range(
        &self,
        channel_id: Uuid,
        to: chrono::DateTime<chrono::Utc>,
        since: chrono::Duration,
    ) -> Vec<Message> {
        todo!();
    }
    /// The count is limited to \[0, 50\] results.
    fn get_n_messages_before(
        &self,
        channel_id: Uuid,
        from: chrono::DateTime<chrono::Utc>,
        count: usize,
    ) -> Vec<Message> {
        let mut conn = self.conn.borrow_mut();
        messages
            .filter(time.le(from))
            // .filter(channel.eq(UuidWrapper(channel_id)))
            .limit(clamp(count as i64, 0, 50))
            .select(MessageModel::as_select())
            .load(&mut conn as &mut SqliteConnection)
            .expect("failed to load n messages before from db")
            .into_iter()
            .map(Message::from)
            .collect()
    }
    #[allow(unused_variables)]
    fn get_unread_message_count(&self, channel_id: Uuid) -> usize {
        todo!();
    }
}
