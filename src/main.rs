use std::time::{SystemTime, UNIX_EPOCH};
use config::Config;
use pushover_rs::{send_pushover_request, Message, MessageBuilder, PushoverSound};
use serde::Deserialize;


#[derive(Debug, Default, Deserialize, PartialEq, Eq)]
struct AppConfig {
    app_token: String,
    user_key: String,
}

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    body: String,
    subject: Option<String>
}

// #[derive(Parser)]
// #[command(author, version, about, long_about = None)]
// struct Cli {
//     #[command(subcommand)]
//     command: Option<Commands>,
// }

// #[derive(Subcommand)]
// enum Commands {
//     /// sends a message
//     Send {
//         message: String,
//         body: Option<String>
//     },
// }

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>>  {
    let cli: Cli = Cli::parse();

    let config = Config::builder()
        // TODO: handle case where file doesn't exist
        .add_source(config::File::with_name("/etc/default/rspo.yaml"))
        // TODO: check file in ~/.config/rspo.conf
        .add_source(
            config::Environment::with_prefix("RSPO")
                .try_parsing(true)
                .separator("_")
                .list_separator(" "),
        )
        .build()
        .unwrap();

    let app_config: AppConfig = config.try_deserialize().unwrap();

    let duration_since_epoch = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    let now: u64 = duration_since_epoch.as_secs();
    let message_builder = MessageBuilder::new(app_config.user_key.as_str(),app_config.app_token.as_str(), &cli.body)
        // .set_title("Example push notification sent through Pushover API")
        // .set_url("https://pushover.net/", Some("Pushover"))
        // .set_priority(1)
        .set_sound(PushoverSound::BIKE)
        .set_timestamp(now);

    let message_builder = match cli.subject {
        Some(subject) => message_builder.set_title(&subject),
        None => message_builder,
    };
    
    let message:Message = message_builder.build();
    
    send_pushover_request(message).await.unwrap();
    Ok(())
}
