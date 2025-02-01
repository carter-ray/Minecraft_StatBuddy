
use serenity::all::{CommandOptionType, CreateCommandOption, ResolvedValue};
use serenity::builder::CreateCommand;
use serenity::model::application::ResolvedOption;
use sqlx::Row;

use crate::get_db_pool;


pub async fn run(options: &[ResolvedOption<'_>]) -> Option<String> {
    let mut player: String = String::new();
    let mut category: String = String::new();

    for arg in options.iter() {
        match arg.value {
            ResolvedValue::String(value) => {
                if value == "player" {
                    player.push_str(value);
                } else if value == "category" {
                    category.push_str(value);
                }
            }
            _ => {}
        }
    }

    Some(format!("{:?} {:?}", player, category))
}

pub async fn register() -> CreateCommand {
    let database: &sqlx::Pool<sqlx::Sqlite> = get_db_pool();

    let results = sqlx::query("PRAGMA table_info(statistics);")
        .fetch_all(database)
        .await
        .unwrap();

    let choices: Vec<String> = results.iter()
        .map(|row| row.try_get("username").unwrap())
        .collect();

    print!("{:?}", &choices);

    let mut cmd_opt: CreateCommandOption = CreateCommandOption::new(
        CommandOptionType::String, 
        "category", 
        "Filter by Category"
    );

    for choice in choices.iter() {
        let name = &choice.replace("_", " ");
        cmd_opt = cmd_opt.add_string_choice(name, choice);
    }

    CreateCommand::new("get_stat")
        .description("Gets statistics from server.")
        .add_option(
            CreateCommandOption::new(CommandOptionType::String, "player", "Filter by Player")
        )
        .add_option(cmd_opt)
}


fn username_cached(username: &str) {
    todo!()
}