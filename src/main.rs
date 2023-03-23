use std::env;

use anyhow::anyhow;
use serenity::async_trait;
use serenity::model::channel::Message;
use serenity::model::gateway::Ready;
use serenity::prelude::*;
use shuttle_secrets::SecretStore;
use tracing::{error, info};

struct Bot;

mod web_scraper;

#[async_trait]
impl EventHandler for Bot {
    async fn message(&self, ctx: Context, msg: Message) {
        let mut iter = msg.content.split_ascii_whitespace();
        if iter.next() == Some("!novel") {
            let novel_name = iter.next().unwrap();
            let message = web_scraper::get_novel_data(novel_name);

            if let Err(e) = msg.channel_id.say(&ctx.http, message).await {
                error!("Error sending message: {:?}", e);
            }
        }
    }

    async fn ready(&self, _: Context, ready: Ready) {
        info!("{} is connected!", ready.user.name);
    }
}

#[shuttle_runtime::main]
async fn serenity(
    #[shuttle_secrets::Secrets] secret_store: SecretStore,
) -> shuttle_serenity::ShuttleSerenity {
    
    // let token = if let Some(token) = secret_store.get("DISCORD_TOKEN") {
    //     token
    // } else {
    //     return Err(anyhow!("'DISCORD_TOKEN' was not found").into());
    // };

        let token = env::var("DISCORD_TOKEN").expect("expected token");

    let intents = GatewayIntents::GUILD_MESSAGES | GatewayIntents::MESSAGE_CONTENT;

    let client = Client::builder(&token, intents)
        .event_handler(Bot)
        .await
        .expect("Err creating client");

    Ok(client.into())
}
