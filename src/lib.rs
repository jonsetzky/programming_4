#![feature(lock_value_accessors)]
use std::path::Path;
use std::sync::LazyLock;

use directories::ProjectDirs;

pub mod arc_mutex_signal;

pub mod packet;
pub mod packet_builder;
pub mod tcp_chat_client;

static PROJECT_DIRS: LazyLock<ProjectDirs> =
    LazyLock::new(|| ProjectDirs::from("", "jonsetzky", "Neighbor Chat").unwrap());

pub fn data_dir() -> &'static Path {
    PROJECT_DIRS.data_dir()
}
