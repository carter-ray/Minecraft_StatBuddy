mod commands;
mod dbfuncs;
mod rconfuncs;

use std::{env, time};
use dbfuncs::update_db;
use dotenv::dotenv;

use serenity::async_trait;
use serenity::builder::{CreateInteractionResponse, CreateInteractionResponseMessage};
use once_cell::sync::Lazy;
use serenity::model::application::Interaction;
use serenity::model::gateway::Ready;
use serenity::model::id::GuildId;
use serenity::prelude::*;
use sqlx::SqlitePool;
use tokio::sync::OnceCell;

struct Handler;

static DB_POOL: Lazy<OnceCell<SqlitePool>> = Lazy::new(OnceCell::new);


#[async_trait]
impl EventHandler for Handler {
    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::Command(command) = interaction {
            println!("Received command interaction: {command:#?}");

            let content = match command.data.name.as_str() {
                "get_ip" => commands::getip::run(&command.data.options()).await,
                "get_stat" => commands::getstat::run(&command.data.options()).await,
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
            env::var("GUILD_ID")
                .expect("Expected GUILD_ID in environment")
                .parse()
                .expect("GUILD_ID must be an integer"),
        );

        let commands = guild_id
            .set_commands(&ctx.http, vec![
                commands::getip::register().await,
                commands::getstat::register().await,
            ])
            .await;
        match commands {
            Ok(_) => println!("Commands registered"),
            Err(e) => panic!("Commands not registered: {}", e)
        }
        
    }
}

// pub static DATABASE: sqlx::SqlitePool;

async fn init_db() {
    let pool = sqlx::sqlite::SqlitePoolOptions::new()
        .max_connections(5)
        .connect_with(
            sqlx::sqlite::SqliteConnectOptions::new()
                .filename("db/statistics.sqlite")
                .create_if_missing(true)
        ).await;

    match pool {
        Ok(db) => {
            sqlx::migrate!("db/migrations").run(&db)
                .await
                .expect("Couldn't run database migrations");
        
            let _ = DB_POOL.set(db);
        },
        Err(e) => panic!("{e}")
    }
    // Run migrations, which updates the database's schema to the latest version.
}

fn get_db_pool() -> &'static SqlitePool {
    DB_POOL.get().expect("Database pool not initialized")
}

#[tokio::main]
async fn main() {
    dotenv().ok();   

    tokio::spawn(async {
        init_db().await;
        println!("Connected to DB");

        loop {
            println!("Beginning Update Loop");
            update_db(get_db_pool()).await;
            
            // Sleep for one hour (3600 seconds)
            tokio::time::sleep(time::Duration::from_secs(3600)).await;
        }
    });

    // Configure the client with your Discord bot token in the environment.
    let token: String = env::var("DISCORD_TOKEN")
        .expect("Expected a token in the environment");
    
    // Build our client.
    let mut client: Client = Client::builder(token, GatewayIntents::empty())
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


