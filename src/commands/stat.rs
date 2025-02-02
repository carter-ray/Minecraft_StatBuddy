

use serenity::all::{CommandOptionType, CreateCommandOption};
use serenity::builder::CreateCommand;
use serenity::model::application::ResolvedOption;

use crate::constants::{get_stat_vec, StatCategory};



pub async fn run(options: &[ResolvedOption<'_>]) -> Option<String> {
    println!("{:?}", options);
    Some("TODO".to_string())
}

pub async fn register() -> CreateCommand {

    // GENERAL STATS
    let general_stat_opts: CreateCommandOption = init_choices(
        CommandOptionType::String, 
        "statistic", 
        "Choose General Statistic",
        StatCategory::General
    );

    // CRAFT STATS
    let craft_stat_opts: CreateCommandOption = init_choices(
        CommandOptionType::String,
        "statistic",
        "Choose Crafting Statistic",
        StatCategory::Craft
    );

    // MINE_STATS
    let mine_stat_opts: CreateCommandOption = init_choices(
        CommandOptionType::String,
        "statistic",
        "Choose Crafting Statistic",
        StatCategory::Mine
    );

    // USE_STATS
    let use_stat_opts: CreateCommandOption = init_choices(
        CommandOptionType::String,
        "statistic",
        "Choose Crafting Statistic",
        StatCategory::Mine
    );

    // BREAK_STATS
    let break_stat_opts: CreateCommandOption = init_choices(
        CommandOptionType::String,
        "statistic",
        "Choose Crafting Statistic",
        StatCategory::Mine
    );

    // TRAVEL_STATS
    let travel_stat_opts: CreateCommandOption = init_choices(
        CommandOptionType::String,
        "statistic",
        "Choose Crafting Statistic",
        StatCategory::Mine
    );

    // MISC_STATS
    let misc_stat_opts: CreateCommandOption = init_choices(
        CommandOptionType::String,
        "statistic",
        "Choose Crafting Statistic",
        StatCategory::Mine
    );

    // KILL / KILLED BY (max 25 options, can't use options)
    let mob_opts: CreateCommandOption = CreateCommandOption::new(
        CommandOptionType::String, 
        "mob",
        "Mob involved"
    );

    // PLAYER FILTER
    let player_filter: CreateCommandOption = CreateCommandOption::new(
        CommandOptionType::String, 
        "player", 
        "filter by player"
    );

    CreateCommand::new("stat")
        .description("Gets statistics from server.")
        .add_option(
            CreateCommandOption::new(
                CommandOptionType::SubCommand, 
                "general", "General Statistics about the Player"
            )
            .add_sub_option(general_stat_opts)
            .add_sub_option(player_filter.clone())
        )
        .add_option(
            CreateCommandOption::new(
                CommandOptionType::SubCommand, 
                "crafting",
                "Statistics about Crafting"
            )
            .add_sub_option(craft_stat_opts)
            .add_sub_option(player_filter.clone())
        )
        .add_option(
            CreateCommandOption::new(
                CommandOptionType::SubCommand,
                "mining",
                "Statistics about Mining"
            )
            .add_sub_option(mine_stat_opts)
            .add_sub_option(player_filter.clone())
        
        )
        .add_option(
            CreateCommandOption::new(
                CommandOptionType::SubCommand,
                "use",
                "Statistics about Item Use"
            )
            .add_sub_option(use_stat_opts)
            .add_sub_option(player_filter.clone())
        )
        .add_option(
            CreateCommandOption::new(
                CommandOptionType::SubCommand,
                "breaking",
                "Statistics about Tool Breaking"
            )
            .add_sub_option(break_stat_opts)
            .add_sub_option(player_filter.clone())
        )
        .add_option(
            CreateCommandOption::new(
                CommandOptionType::SubCommand,
                "travel",
                "Statistics about Travel"
            )
            .add_sub_option(travel_stat_opts)
            .add_sub_option(player_filter.clone())
        )
        .add_option(
            CreateCommandOption::new(
                CommandOptionType::SubCommand,
                "misc",
                "Statistics about Misc. things"
            )
            .add_sub_option(misc_stat_opts)
            .add_sub_option(player_filter.clone())
        )
        .add_option(
            CreateCommandOption::new(
                CommandOptionType::SubCommand, 
                "kill",
                "Number of mobs killed"
            )
            .add_sub_option(mob_opts.clone())
            .add_sub_option(player_filter.clone())
        )
        .add_option(
            CreateCommandOption::new(
                CommandOptionType::SubCommand, 
                "killed_by",
                "Number of deaths to a mob"
            )
            .add_sub_option(mob_opts)
            .add_sub_option(player_filter)
        )
        
        
}


fn init_choices(kind: CommandOptionType, name: &str, description: &str, choice_type: StatCategory) -> CreateCommandOption {
    let mut cmd_option: CreateCommandOption = CreateCommandOption::new(
        kind, 
        name,
        description,
    );
    
    for choice in get_stat_vec(choice_type) {
        cmd_option = cmd_option.add_string_choice(choice.to_string(), choice.to_string());
    }
    cmd_option
}