use serenity::builder::CreateApplicationCommand;
use serenity::model::prelude::command::CommandOptionType;

pub fn chat_command() -> CreateApplicationCommand {
    let mut command = CreateApplicationCommand::default();
    command.name("chat")
        .description("Chat with the AI")
        .create_option(|option| {
            option
                .name("prompt")
                .description("Your message to the AI")
                .kind(CommandOptionType::String)
                .required(true)
        });
    command
}

pub fn reset_command() -> CreateApplicationCommand {
    let mut command = CreateApplicationCommand::default();
    command.name("reset")
        .description("Reset the conversation history");
    command
}

pub fn set_ephemeral_command() -> CreateApplicationCommand {
    let mut command = CreateApplicationCommand::default();
    command.name("set_ephemeral")
        .description("Set whether bot responses should be ephemeral (only visible to you)")
        .create_option(|option| {
            option
                .name("enabled")
                .description("Enable or disable ephemeral responses")
                .kind(CommandOptionType::Boolean)
                .required(true)
        });
    command
}