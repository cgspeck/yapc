use humantime::format_duration;
use std::time::Duration;

use crate::{send, AppConfig};

pub async fn shell_done(
    app_config: AppConfig,
    now: u64,
    return_code: i32,
    duration: u64,
    command: String,
) -> Result<(), Box<dyn std::error::Error>> {
    let subject = if return_code == 0 {
        "Execution Succeeded"
    } else {
        "Execution Failed"
    };
    let dur = Duration::new(duration, 0);

    let mut body = String::new();

    body += &format!(
        "Execution of the following command by {} on {} ",
        whoami::username(),
        whoami::hostname()
    );

    if return_code == 0 {
        body += "succeded ";
    } else {
        body += &format!("failed (with code {}) ", return_code);
    };

    body += &format!("in {}", format_duration(dur));

    body += ":\n\n";
    body += command.as_str();

    send::send(
        app_config,
        now,
        body,
        Some(String::from(subject)),
        None,
        None,
        None,
    )
    .await
}
