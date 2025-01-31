
use serenity::all::{CommandOptionType, CreateCommandOption};
use serenity::builder::CreateCommand;
use serenity::model::application::ResolvedOption;

pub async fn run(_options: &[ResolvedOption<'_>]) -> Option<String> {
    Some("TODO".to_string())
}

pub fn register() -> CreateCommand {
    CreateCommand::new("get_stat")
        .description("Gets statistic.")
        .add_option(
            CreateCommandOption::new(CommandOptionType::String, "player", "Filter by Player")
        )
        .add_option(
            CreateCommandOption::new(CommandOptionType::String, "category", "Filter by Category")
        )
}