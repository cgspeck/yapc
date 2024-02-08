use clap::ValueEnum;
use pushover_rs::PushoverSound;
use serde::Deserialize;

pub mod send;
pub mod shell_done;
pub mod shell_integration;

fn default_priority() -> i8 {
    0
}

fn default_sound() -> YapcPushoverSound {
    YapcPushoverSound::GAMELAN
}

#[derive(Debug, Default, Deserialize, PartialEq, Eq)]
pub struct AppConfig {
    app_token: String,
    user_key: String,
    #[serde(default = "default_priority")]
    priotity: i8,
    #[serde(default = "default_sound")]
    sound: YapcPushoverSound,
}

#[derive(Clone, Debug, PartialEq, Eq, ValueEnum)]
pub enum Shell {
    Bash,
    Zsh,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Eq, ValueEnum)]
pub enum YapcPushoverSound {
    PUSHOVER,
    BIKE,
    BUGLE,
    CASHREGISTER,
    CLASSICAL,
    COSMIC,
    FALLING,
    GAMELAN,
    INCOMING,
    INTERMISSION,
    MAGIC,
    MECHANICAL,
    PIANOBAR,
    SIREN,
    SPACEALARM,
    TUGBOAT,
    ALIEN,
    CLIMB,
    PERSISTENT,
    ECHO,
    UPDOWN,
    VIBRATE,
    NONE,
}

impl Default for YapcPushoverSound {
    fn default() -> Self {
        default_sound()
    }
}

impl Into<PushoverSound> for YapcPushoverSound {
    fn into(self) -> PushoverSound {
        match self {
            YapcPushoverSound::PUSHOVER => PushoverSound::PUSHOVER,
            YapcPushoverSound::BIKE => PushoverSound::BIKE,
            YapcPushoverSound::BUGLE => PushoverSound::BUGLE,
            YapcPushoverSound::CASHREGISTER => PushoverSound::CASHREGISTER,
            YapcPushoverSound::CLASSICAL => PushoverSound::CLASSICAL,
            YapcPushoverSound::COSMIC => PushoverSound::COSMIC,
            YapcPushoverSound::FALLING => PushoverSound::FALLING,
            YapcPushoverSound::GAMELAN => PushoverSound::GAMELAN,
            YapcPushoverSound::INCOMING => PushoverSound::INCOMING,
            YapcPushoverSound::INTERMISSION => PushoverSound::INTERMISSION,
            YapcPushoverSound::MAGIC => PushoverSound::MAGIC,
            YapcPushoverSound::MECHANICAL => PushoverSound::MECHANICAL,
            YapcPushoverSound::PIANOBAR => PushoverSound::PIANOBAR,
            YapcPushoverSound::SIREN => PushoverSound::SIREN,
            YapcPushoverSound::SPACEALARM => PushoverSound::SPACEALARM,
            YapcPushoverSound::TUGBOAT => PushoverSound::TUGBOAT,
            YapcPushoverSound::ALIEN => PushoverSound::ALIEN,
            YapcPushoverSound::CLIMB => PushoverSound::CLIMB,
            YapcPushoverSound::PERSISTENT => PushoverSound::PERSISTENT,
            YapcPushoverSound::ECHO => PushoverSound::ECHO,
            YapcPushoverSound::UPDOWN => PushoverSound::UPDOWN,
            YapcPushoverSound::VIBRATE => PushoverSound::VIBRATE,
            YapcPushoverSound::NONE => PushoverSound::NONE,
        }
    }
}
