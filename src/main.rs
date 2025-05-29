pub mod commands;
pub mod dbfuncs;
pub mod rconfuncs;
pub mod constants;

use dbfuncs::{init_db, update_db};
use serde::Deserialize;

use std::io::{self};
use std::{process, time};
use once_cell::sync::Lazy;
use serenity::async_trait;
use serenity::builder::{CreateInteractionResponse, CreateInteractionResponseMessage};
use serenity::model::application::Interaction;
use serenity::model::gateway::Ready;
use serenity::model::id::GuildId;
use serenity::prelude::*;
use dotenv::dotenv;

struct Handler;

#[derive(Debug, Deserialize)]
pub struct Config{
    discord_token: String,
    guild_id: u64,
    global_server_addr: String,
    rcon_addr_port: String,
    rcon_pw: String
}

fn pause_before_exit() {
    println!("Press Enter to exit...");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    process::exit(1);
}

fn fix_addr(addr_port: String) -> String {
    let mut addr_port_vec: Vec<&str> = addr_port.split(":").collect();
    if addr_port_vec[0].len() == 0 {
        addr_port_vec[0] = "localhost";
    }
    else if  addr_port_vec.len() == 1{
        addr_port_vec.insert(0, "localhost");
    }
    addr_port_vec.join(":")
}

pub static CONFIG: Lazy<Config> = Lazy::new(|| {
    let dbt = std::env::var("DISCORD_BOT_TOKEN").expect("DISCORD_BOT_TOKEN not found");
    let gid = std::env::var("GUILD_ID").expect("GUILD_ID not found");
    let gsa = fix_addr(std::env::var("GLOBAL_SERVER_ADDRESS").unwrap_or(":25565".to_string()));
    let rap: String = fix_addr(std::env::var("RCON_ADDR_PORT").unwrap_or(":25575".to_string()));
    let rcpw = std::env::var("RCON_PW").expect("RCON_PW not found");
    Config { 
        discord_token: dbt,
        guild_id: gid.parse::<u64>().expect("GUILD_ID could not be parsed as a number."),
        global_server_addr: gsa,
        rcon_addr_port: rap, 
        rcon_pw: rcpw
    }
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
    dotenv().ok();
    let _ = CONFIG;
    if CONFIG.discord_token.is_empty() || CONFIG.rcon_pw.is_empty() {
        eprintln!("INvalid .env file.");
        let mut missings = String::from("");
        if CONFIG.discord_token.is_empty() {
            missings.push_str("\n\tDISCORD_BOT_TOKEN");
        }
        if CONFIG.rcon_pw.is_empty() {
            missings.push_str("\n\tRCON_PW");
        }
        eprintln!("Missing:{}", missings);
        pause_before_exit();
    }

    tokio::spawn(async {
        init_db().await;
        println!("initialized DB");
        
        println!("Beginning DB Update Loop");
        loop {
            update_db().await;
            // sleep for 1 hour
            tokio::time::sleep(time::Duration::from_secs(3600)).await;
        }
    });
    
    let mut client: Client = Client::builder(&CONFIG.discord_token, GatewayIntents::empty())
        .event_handler(Handler)
        .await
        .expect("Error creating client");

    if let Err(why) = client.start().await {
        eprintln!("Client error: {why:?}");
        pause_before_exit();
    }
}


