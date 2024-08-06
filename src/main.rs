use dotenv::dotenv;
use serenity::prelude::*;
use std::env;

mod bot;
mod ai;
mod utils;

use bot::handler::Handler;

#[tokio::main]
async fn main() {
    dotenv().ok();
    utils::logger::setup_logger();

    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");
    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT;

    let mut client = Client::builder(&token, intents)
        .event_handler(Handler::new())
        .await
        .expect("Err creating client");

    if let Err(why) = client.start().await {
        log::error!("Client error: {:?}", why);
    }
}