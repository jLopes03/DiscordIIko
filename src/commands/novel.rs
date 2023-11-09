use crate::extras::web_scraper;
use serenity::builder::{CreateApplicationCommand, CreateEmbed};
use serenity::model::prelude::command::CommandOptionType;
use serenity::model::prelude::interaction::application_command::{
    CommandDataOption, CommandDataOptionValue,
};

pub async fn run(options: &[CommandDataOption]) -> CreateEmbed {
    let option = options
        .get(0)
        .expect("Expected an option")
        .resolved
        .as_ref()
        .expect("Expected a valid novel name");

    let mut embed = CreateEmbed::default();

    match option {
        CommandDataOptionValue::String(novel_name) => {
            match web_scraper::get_novel_data(novel_name).await {
                Some((name, image_url, synopsis)) => {
                    embed.title(name).thumbnail(image_url).description(synopsis);
                    embed
                }

                None => {
                    embed.title("Error").description("The novel title is most likely wrong.\nOr it could be that the Wayback Machine hasn't done a backup of that website.\nBut check out a great novel at: https://www.royalroad.com/fiction/44651/breaking-the-chains");
                    embed
                }
            }
        }

        &_ => {
            embed
                .title("Error")
                .description("Probably an invalid option");
            embed
        }
    }
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command
        .name("novel")
        .description("Info about a novel on Novel Updates")
        .create_option(|option| {
            option
                .name("name")
                .description("The name of a novel on Novel Updates as it is on the url")
                .kind(CommandOptionType::String)
                .required(true)
        })
}
