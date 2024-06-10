use std::env;
use std::time::Duration;
use webhooks::{send_error_webhook, send_webhook};

mod webhooks;

#[async_std::main]
async fn main() {
    dotenv::dotenv().ok();

    let token = env::var("WEBHOOK_TOKEN").expect("WEBHOOK_TOKEN not found");
    let host = env::var("HOST").expect("Server IP not found");

    let request = mcping::get_status(host.as_str(), Duration::from_secs(5));
    match request {
        Ok((latency, response)) => {
            send_webhook(&token, response, latency).await;
        }
        Err(e) => {
            println!("Error: {}", e);
            send_error_webhook(&token, e).await;
        }
    }
}
