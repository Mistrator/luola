const VERSION: Option<&str> = option_env!("CARGO_PKG_VERSION");

pub fn get_version() -> String {
    String::from(VERSION.unwrap_or("unknown-version"))
}

pub const WORLD_HEIGHT: i32 = 50;
pub const WORLD_WIDTH: i32 = 50;
pub const WORLD_LAYERS: i32 = 10;
