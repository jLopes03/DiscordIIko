use anyhow::anyhow;
use serenity::async_trait;
use serenity::builder::CreateEmbed;
use serenity::model::gateway::Ready;
use serenity::model::prelude::command::Command;
use serenity::model::prelude::{Interaction, InteractionResponseType};
use serenity::prelude::*;
use shuttle_secrets::SecretStore;
use tracing::info;

mod commands;
mod extras;
struct Bot;

#[async_trait]
impl EventHandler for Bot {
    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::ApplicationCommand(command) = interaction {
            println!("Received: {:?}", command);

            // this placeholder allows the code more than 3 seconds to run and display a response on discord
            if let Err(why) = command
                .create_interaction_response(&ctx, |response| {
                    response
                        .kind(InteractionResponseType::ChannelMessageWithSource)
                        .interaction_response_data(|message| {
                            message
                                .add_embed(CreateEmbed::default().title("Processing...").to_owned())
                        })
                })
                .await
            {
                println!("Error sending placeholder: {}", why)
            }

            let content = match command.data.name.as_str() {
                "novel" => commands::novel::run(&command.data.options).await,
                _ => CreateEmbed::default().title("not implemented").to_owned(),
            };

            if let Err(why) = command
                .edit_original_interaction_response(&ctx, |response| response.set_embed(content))
                .await
            {
                println!("Cannot edit placeholder: {}", why);
            }
        }
    }

    async fn ready(&self, ctx: Context, ready: Ready) {
        info!("{} is connected!", ready.user.name);

        let registered_commands =
            Command::set_global_application_commands(&ctx.http, |slash_commands| {
                slash_commands
                    .create_application_command(|command| commands::novel::register(command))
            })
            .await;

        println!(
            "Registered the following commands: {:?}",
            registered_commands
        );
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
