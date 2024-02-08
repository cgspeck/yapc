use config::Config;
use homedir::get_my_home;
use std::{
    path::Path,
    time::{SystemTime, UNIX_EPOCH},
};
use yapc::{
    send::send, shell_done::shell_done, shell_integration::shell_integration, AppConfig, Shell,
    YapcPushoverSound,
};

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// sends a message
    Send {
        body: String,
        subject: Option<String>,
        #[arg(short, long)]
        url: Option<String>,
        #[arg(short = 't', long)]
        url_title: Option<String>,
        #[arg(short, long)]
        prioity: Option<i8>,
        #[arg(short, long, value_enum)]
        sound: Option<YapcPushoverSound>,
    },
    /// callback for shell integration hook
    ShellDone {
        return_code: i32,
        duration: u64,
        command: String,
    },
    /// emit shell integration code
    ShellIntegration {
        #[arg(value_enum)]
        shell: Shell,
    },
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli: Cli = Cli::parse();

    let builder = Config::builder();

    let cfg_fn = "yapc.yaml";
    let system_cfg = Path::new("/etc/default/").join(cfg_fn);

    let builder = match system_cfg.exists() {
        true => builder.add_source(config::File::from(system_cfg)),
        false => builder,
    };

    let user_cfg = get_my_home().unwrap().unwrap().join(".config").join(cfg_fn);
    let builder = match user_cfg.exists() {
        true => builder.add_source(config::File::from(user_cfg)),
        false => builder,
    };

    let builder = builder.add_source(
        config::Environment::with_prefix("yapc")
            .try_parsing(true)
            .separator("_")
            .list_separator(" "),
    );

    let config = builder.build().unwrap();

    let app_config: AppConfig = config.try_deserialize().unwrap();
    let duration_since_epoch = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    let now: u64 = duration_since_epoch.as_secs();

    match cli.command {
        Command::Send {
            body,
            subject,
            url,
            url_title,
            prioity,
            sound,
        } => {
            send(
                app_config, now, body, subject, url, url_title, prioity, sound,
            )
            .await
        }
        Command::ShellDone {
            return_code,
            duration,
            command,
        } => shell_done(app_config, now, return_code, duration, command).await,
        Command::ShellIntegration { shell } => shell_integration(shell).await,
    }
}
