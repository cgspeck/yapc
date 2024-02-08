use pushover_rs::{send_pushover_request, Message, MessageBuilder, PushoverSound};

use crate::{AppConfig, YapcPushoverSound};

pub async fn send(
    app_config: AppConfig,
    now: u64,
    body: String,
    subject: Option<String>,
    url: Option<String>,
    url_title: Option<String>,
    prioity: Option<i8>,
    sound: Option<YapcPushoverSound>,
) -> Result<(), Box<dyn std::error::Error>> {
    let sound: PushoverSound = (sound.unwrap_or_else(|| app_config.sound)).into();
    let message_builder = MessageBuilder::new(
        app_config.user_key.as_str(),
        app_config.app_token.as_str(),
        &body,
    )
    .set_priority(prioity.unwrap_or_else(|| app_config.priotity))
    .set_sound(sound)
    .set_timestamp(now);

    let message_builder = match subject {
        Some(subject) => message_builder.set_title(&subject),
        None => message_builder,
    };

    let message_builder = match url {
        Some(url) => message_builder.set_url(&url, url_title.as_deref()),
        None => message_builder,
    };

    let message: Message = message_builder.build();

    send_pushover_request(message).await.unwrap();
    Ok(())
}
