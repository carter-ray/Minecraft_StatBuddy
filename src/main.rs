mod commands;

use std::net::TcpStream;
use std::{env, time};
use dotenv::dotenv;

use serenity::async_trait;
use serenity::builder::{CreateInteractionResponse, CreateInteractionResponseMessage};
use serenity::model::application::Interaction;
use serenity::model::gateway::Ready;
use serenity::model::id::GuildId;
use serenity::prelude::*;

use sqlx::Row;

struct Handler;



#[async_trait]
impl EventHandler for Handler {
    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::Command(command) = interaction {
            println!("Received command interaction: {command:#?}");

            let content = match command.data.name.as_str() {
                "get_ip" => commands::getip::run(&command.data.options()).await,
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
                commands::getip::register(),
            ])
            .await;

        println!("I now have the following guild slash commands: {commands:#?}");
        
    }
}

#[tokio::main]
async fn main() {
    dotenv().ok();

    // Configure the client with your Discord bot token in the environment.
    let token: String = env::var("DISCORD_TOKEN")
        .expect("Expected a token in the environment");

    tokio::spawn(async {
        let db_name: String = env::var("DATABASE_URL")
        .expect("Expected a database URL");
    
        println!("DB name: {}", db_name);

        let database: sqlx::SqlitePool = sqlx::sqlite::SqlitePoolOptions::new()
            .max_connections(5)
            .connect_with(
                sqlx::sqlite::SqliteConnectOptions::new()
                    .filename("db/statistics.sqlite")
                    .create_if_missing(true)
            )
            .await
            .expect("Couldn't connect to database");

        println!("Connected to DB");

        // Run migrations, which updates the database's schema to the latest version.
        sqlx::migrate!("db/migrations").run(&database).await.expect("Couldn't run database migrations");
        
        loop {
            println!("Beginning Update Loop");
            update_db(&database).await;
            
            // Sleep for one hour (3600 seconds)
            tokio::time::sleep(time::Duration::from_secs(3600)).await;
        }
    });

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

async fn update_db(database: &sqlx::SqlitePool) {
    let addr_port: String = env::var("LOCAL_ADDR")
        .expect("Expected an rcon address:port in the environment");
    
    let _rcon_pw: String = env::var("RCON_PW")
        .expect("Expected an rcon password in the environment");

    let result = sqlx::query("SELECT username FROM statistics;")
        .fetch_all(database).await
        .unwrap();

        
    let mut usernames: Vec<&str> = Vec::with_capacity(result.len());
    for row in result.iter() {
        let username: &str = row.try_get("username").unwrap();
        usernames.push(username);
    }

    println!("Usernames: {:#?}", usernames);

    let tcp_stream: Result<TcpStream, std::io::Error> = TcpStream::connect(&addr_port);
    match tcp_stream {
        Ok(_stream) => {

            println!("Connected to RCON server")
            

            // Connection::cmd(&mut self, "")
        },
        Err(_) => panic!("Could not connect to RCON server {}", &addr_port)
    }


    // let task_description = task_description.trim();
    // // That's how we are going to use a sqlite command.
    // // We are inserting into the todo table, our task_description in task column and our
    // // user_id in user_Id column.
    // sqlx::query!(
    //     "INSERT INTO todo (task, user_id) VALUES (?, ?)",
    //     task_description,
    //     user_id,
    // )
    // .execute(&self.database) // < Where the command will be executed
    // .await
    // .unwrap();

    // let response = format!("Successfully added `{task_description}` to your todo list");
    // msg.channel_id.say(&ctx, response).await.unwrap();
}

