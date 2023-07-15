mod args;
mod config;
mod cron;
mod logger;
mod rcon;
mod rcon_pure;
mod rcon_web;

use args::Args;
use clap::Parser;
use config::Config;
use cron::create_scheduler;
use logger::log;
use rcon::Rcon;
use rcon_web::WebRcon;
use std::sync::Arc;
use tokio::sync::Mutex;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    let config = Config::from_file(args.config);

    let web_rcon = Arc::new(Mutex::new(
        WebRcon::connect(&config.ip, &config.port, &config.password).await,
    ));

    log("INFO", "bright_black", "Connected.");

    let receiver = { web_rcon.lock().await.receiver.clone() };

    create_scheduler(config.clone(), web_rcon.clone()).await;

    tokio::spawn(async move {
        loop {
            let mut input = String::default();
            std::io::stdin().read_line(&mut input).unwrap();
            web_rcon.lock().await.execute(&input).await;
        }
    });

    while let Ok(data) = receiver.recv().await {
        if config.server_log {
            match data {
                rcon::RconMessage::Chat {
                    Channel,
                    Message,
                    UserId,
                    Username,
                    ..
                } => {
                    log(
                        "CHAT",
                        "green",
                        &format!("<{}[{}]>({}) {}", Username, UserId, Channel, Message),
                    );
                }
                rcon::RconMessage::Generic { message } => {
                    log("INFO", "bright_black", &message);
                }
                rcon::RconMessage::Warning { message } => {
                    log("WARN", "red", &message);
                }
                rcon::RconMessage::Error { message } => {
                    log("ERROR", "red", &message);
                }
                rcon::RconMessage::Disconnected => {
                    std::process::exit(0);
                }
            }
        }
    }

    Ok(())
}
