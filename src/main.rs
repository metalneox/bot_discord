use std::env;

use serenity::async_trait;
use serenity::model::channel::Message;
use serenity::model::gateway::Ready;
use serenity::prelude::*;

use featurebot::core::command;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        let command = msg
            .content
            .split_whitespace()
            .map(|x| x.to_string())
            .collect::<Vec<String>>();

        let mut cmd = String::new();
        let mut value: Option<String> = None;

        if command.len() > 1 {
            cmd = command[0].clone();
            value = Some(command[1].clone());
        } else {
            cmd = command[0].clone();
        }

        let commandi = command::Cmds {
            cmd: cmd,
            value: value,
        };

        let result = commandi.run().await;

        if !msg.author.bot {
            // Rispondi con lo stesso messaggio
            //&msg.content
            if result.is_ok() {
                let msg_data = result.unwrap();
                if let Err(why) = msg.channel_id.say(&ctx.http, msg_data).await {
                    println!("Errore nell'invio del messaggio: {:?}", why);
                }
            }
        }
    }

    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }
}

#[tokio::main]
async fn main() {
    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");

    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT;

    let mut client = Client::builder(&token, intents)
        .event_handler(Handler)
        .await
        .expect("Err creating client");

    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}
