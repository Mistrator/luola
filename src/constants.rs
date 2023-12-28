const VERSION: Option<&str> = option_env!("CARGO_PKG_VERSION");

pub fn get_version() -> String {
    String::from(VERSION.unwrap_or("unknown-version"))
}
