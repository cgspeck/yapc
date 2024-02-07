use serde::Deserialize;

pub mod send;
pub mod shell_done;

fn default_priority() -> i8 {
    0
}

#[derive(Debug, Default, Deserialize, PartialEq, Eq)]
pub struct AppConfig {
    app_token: String,
    user_key: String,
    #[serde(default = "default_priority")]
    priotity: i8,
}
