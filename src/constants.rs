const VERSION: Option<&str> = option_env!("CARGO_PKG_VERSION");

pub fn get_version() -> String {
    String::from(VERSION.unwrap_or("unknown-version"))
}

pub const WORLD_HEIGHT: i32 = 50;
pub const WORLD_WIDTH: i32 = 50;
pub const WORLD_LAYERS: i32 = 10;

// Smallest possible level. Player characters start at level 1, so this makes it
// possible to have creatures that are weaker than new player characters.
pub const MIN_LEVEL: i32 = -1;
