mod commands;

use std::{
    env::var,
    sync::{Arc, Mutex}
};

use poise::serenity_prelude as serenity;

type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;


#[tokio::main]
async fn main() {   
    let token = std::env::var("DISCORD_TOKEN")
        .expect("missing DISCORD_TOKEN");
    let guild_id: u32 = std::env::var("GUILD_ID")
        .parse::<u32>()
        .expect("Missing GUILD_ID or could not parse as int").

    let intents = serenity::GatewayIntents::non_privileged();

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![
                commands::get_ip(),
            ],

        })
        .setup(move |ctx, _ready, framework| {
            Box::pin(async move {
                poise::builtins::register_in_guild(ctx, &framework.options().commands, guild_id).await?;
                Ok(Data {

                })
            })
        })
        .build();

    let client = serenity::ClientBuilder::new(token, intents)
        .framework(framework)
        .await;
    client.unwrap().start().await.unwrap();
}
