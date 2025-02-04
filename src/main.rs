mod commands;
mod dbfuncs;
mod rconfuncs;
pub mod constants;

use dbfuncs::{init_db, update_db};
use serde::Deserialize;

use std::io::BufReader;
use std::time;
use std::fs::File;
use once_cell::sync::Lazy;
use serenity::async_trait;
use serenity::builder::{CreateInteractionResponse, CreateInteractionResponseMessage};
use serenity::model::application::Interaction;
use serenity::model::gateway::Ready;
use serenity::model::id::GuildId;
use serenity::prelude::*;


struct Handler;

#[derive(Debug, Deserialize)]
pub struct Config{
    discord_token: String,
    guild_id: u64,
    global_server_addr: String,
    rcon_addr_port: String,
    rcon_pw: String
}

pub static CONFIG: Lazy<Config> = Lazy::new(|| {
    let file = File::open("config.json").expect("Failed to open config.json");
    let reader = BufReader::new(file);

    serde_json::from_reader(reader).expect("Failed to parse config.json")
});



#[async_trait]
impl EventHandler for Handler {
    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::Command(command) = interaction {
            let content = match command.data.name.as_str() {
                "ip" => commands::ip::run(&command.data.options()).await,
                "stat" => commands::stat::run(&command.data.options()).await,
                _ => Some("not implemented :(".to_string()),
            };

            if let Some(content) = content {
                let data = CreateInteractionResponseMessage::new().content(content);
                let builder = CreateInteractionResponse::Message(data);
                if let Err(why) = command.create_response(&ctx.http, builder).await {
                    println!("Cannot respond to slash command: {why}");
                }
            }
        }
    }

    async fn ready(&self, ctx: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);

        let guild_id = GuildId::new(
            CONFIG.guild_id
        );

        let commands = guild_id
            .set_commands(&ctx.http, vec![
                commands::ip::register().await,
                commands::stat::register().await,
            ])
            .await;
        match commands {
            Ok(_) => println!("Commands registered"),
            Err(e) => panic!("Commands not registered: {}", e)
        }
        
    }
}

#[tokio::main]
async fn main() {
    
    tokio::spawn(async {
        init_db().await;
        println!("Connected to DB");
        
        loop {
            println!("Beginning Update Loop");
            update_db().await;
            
            // Sleep for one hour (3600 seconds)
            tokio::time::sleep(time::Duration::from_secs(3600)).await;
        }
    });
    
    // Build our client.
    let mut client: Client = Client::builder(&CONFIG.discord_token, GatewayIntents::empty())
        .event_handler(Handler)
        .await
        .expect("Error creating client");

    // Finally, start a single shard, and start listening to events.
    //
    // Shards will automatically attempt to reconnect, and will perform exponential backoff until
    // it reconnects.
    if let Err(why) = client.start().await {
        println!("Client error: {why:?}");
    }
}


