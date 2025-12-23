use std::path::Path;
use std::sync::LazyLock;

use directories::ProjectDirs;

static PROJECT_DIRS: LazyLock<ProjectDirs> =
    LazyLock::new(|| ProjectDirs::from("", "jonsetzky", "Neighbor Chat").unwrap());

pub fn data_dir() -> &'static Path {
    PROJECT_DIRS.data_dir()
}
