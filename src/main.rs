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
        if msg.author.bot
            || msg.content.len() < 6
            || !msg.content.is_ascii()
            || &msg.content[..6] != "!novel"
        {
            // a bunch of verifications to get rid of annoying errors in the terminal
            // the 6 is not inclusive in [..6]
            // checking if it's not ascii because there are messages where a non_ascii char lands in byte position 6
            return;
        }

        let novel_name = &msg.content[6..];

        match web_scraper::get_novel_data(novel_name).await {
            Some((title, image_url, synopsis)) => {
                if let Err(error) = msg
                    .channel_id
                    .send_message(&ctx.http, |m| {
                        m.embed(|e| e.title(title).thumbnail(image_url).description(synopsis))
                    })
                    .await
                {
                    error!("Error sending message: {:?}", error);
                }
            }
            _ => {
                if let Err(error) = msg
                    .channel_id
                    .send_message(&ctx.http, |m| {
                        m.embed(|e| e.title("Error").description("The novel title is most likely wrong.\nBut check out a great novel at: https://www.royalroad.com/fiction/44651/breaking-the-chains"))
                    })
                    .await
                {
                    error!("Error sending message: {:?}", error);
                }
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
    let token = if let Some(token) = secret_store.get("DISCORD_TOKEN") {
        token
    } else {
        return Err(anyhow!("'DISCORD_TOKEN' was not found").into());
    };

    let intents = GatewayIntents::GUILD_MESSAGES | GatewayIntents::MESSAGE_CONTENT;

    let client = Client::builder(&token, intents)
        .event_handler(Bot)
        .await
        .expect("Err creating client");

    Ok(client.into())
}
