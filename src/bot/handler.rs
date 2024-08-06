use serenity::async_trait;
use serenity::model::application::interaction::{Interaction, InteractionResponseType};
use serenity::model::gateway::Ready;
use serenity::model::prelude::UserId;
use serenity::prelude::*;
use std::sync::Arc;
use std::collections::HashMap;

use crate::bot::commands::{chat_command, reset_command, set_ephemeral_command};
use crate::ai::command_r_plus::CommandRPlus;

pub struct Handler {
    ai: Arc<Mutex<CommandRPlus>>,
    ephemeral_settings: Arc<Mutex<HashMap<UserId, bool>>>,
}

impl Handler {
    pub fn new() -> Self {
        Handler {
            ai: Arc::new(Mutex::new(CommandRPlus::new())),
            ephemeral_settings: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    async fn is_ephemeral(&self, user_id: UserId) -> bool {
        let settings = self.ephemeral_settings.lock().await;
        *settings.get(&user_id).unwrap_or(&true)
    }

    async fn set_ephemeral(&self, user_id: UserId, enabled: bool) {
        let mut settings = self.ephemeral_settings.lock().await;
        settings.insert(user_id, enabled);
    }
}

#[async_trait]
impl EventHandler for Handler {
    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::ApplicationCommand(command) = interaction {
            log::info!("Received command: {}", command.data.name);

            let is_ephemeral = self.is_ephemeral(command.user.id).await;

            if let Err(why) = command
                .create_interaction_response(&ctx.http, |response| {
                    response
                        .kind(InteractionResponseType::DeferredChannelMessageWithSource)
                        .interaction_response_data(|message| message.ephemeral(is_ephemeral))
                })
                .await
            {
                log::error!("Cannot send thinking response: {}", why);
                return;
            }

            let content = match command.data.name.as_str() {
                "chat" => {
                    let prompt = command.data.options.get(0)
                        .and_then(|opt| opt.value.as_ref())
                        .and_then(|value| value.as_str())
                        .unwrap_or("No prompt provided");
                    
                    let mut ai = self.ai.lock().await;
                    match ai.chat(prompt).await {
                        Ok(response) => response,
                        Err(e) => {
                            log::error!("Error processing chat command: {}", e);
                            format!("Error: {}", e)
                        }
                    }
                },
                "reset" => {
                    let mut ai = self.ai.lock().await;
                    ai.reset_history();
                    "Conversation history has been reset.".to_string()
                },
                "set_ephemeral" => {
                    let enabled = command.data.options.get(0)
                        .and_then(|opt| opt.value.as_ref())
                        .and_then(|value| value.as_bool())
                        .unwrap_or(true);
                    self.set_ephemeral(command.user.id, enabled).await;
                    format!("Ephemeral mode has been set to: {}", enabled)
                },
                _ => "Not implemented :(".to_string(),
            };

            if let Err(why) = command
                .edit_original_interaction_response(&ctx.http, |response| {
                    response.content(content)
                })
                .await
            {
                log::error!("Cannot send command response: {}", why);
            }
        }
    }

    async fn ready(&self, ctx: Context, ready: Ready) {
        log::info!("{} is connected!", ready.user.name);

        let guild_id = ready.guilds[0].id;

        let commands = guild_id
            .set_application_commands(&ctx.http, |commands| {
                commands
                    .create_application_command(|command| {
                        *command = chat_command();
                        command
                    })
                    .create_application_command(|command| {
                        *command = reset_command();
                        command
                    })
                    .create_application_command(|command| {
                        *command = set_ephemeral_command();
                        command
                    })
            })
            .await;

        match commands {
            Ok(_) => log::info!("Slash commands registered successfully"),
            Err(why) => log::error!("Failed to register slash commands: {:?}", why),
        }
    }
}