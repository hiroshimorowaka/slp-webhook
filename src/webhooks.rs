use std::env;

use mcping::Response;
use webhook::client::WebhookClient;

const BOT_USERNAME: &str = "Server";

pub async fn send_error_webhook(token: &str, err: mcping::Error) {
    let url: &str = token;
    let client: WebhookClient = WebhookClient::new(url);
    let message = client
        .send(|message| {
            message.username(BOT_USERNAME).embed(|embed| {
                embed
                    .color("10038562")
                    .title("Server Status")
                    .field("Status", "Probably Offline", false)
                    .field("Informations", err.to_string().as_str(), false)
            })
        })
        .await;
    match message {
        Ok(_) => {
            println!("Message sent!");
        }
        Err(e) => {
            println!("Error: {}", e);
        }
    }
}

pub async fn send_webhook(token: &str, response: Response, latency: u64) {
    let url: &str = token;
    let client: WebhookClient = WebhookClient::new(url);
    let players = match response.players.sample {
        Some(players) => players
            .iter()
            .map(|player| {
                let name = player.name.clone();
                name
            })
            .collect::<Vec<String>>()
            .join("\n"),
        None => String::from("No players online"),
    };
    let message = client
        .send(|message| {
            message.username(BOT_USERNAME).embed(|embed| {
                embed
                    .title("Server Status")
                    .color("5763719")
                    .field("Status", "Online", true)
                    .field("Ping", &latency.to_string(), true)
                    .field(
                        "Player Count",
                        format!("{}/{}", response.players.online, response.players.max).as_str(),
                        true,
                    )
                    .field("Players", players.as_str(), false)
                    .field(
                        "Server IP",
                        env::var("HOST").unwrap_or(String::from("...")).as_str(),
                        true,
                    )
                    .field("Version", response.version.name.as_str(), true)
            })
        })
        .await;
    match message {
        Ok(_) => {
            println!("Message sent!");
        }
        Err(e) => {
            println!("Error: {}", e);
        }
    }
}
