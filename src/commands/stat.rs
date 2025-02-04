use serenity::all::{CommandOptionType, CreateCommandOption};
use serenity::builder::CreateCommand;
use serenity::model::application::{ResolvedOption, ResolvedValue};

use sqlx::Row;

use crate::constants::{get_stat_vec, StatCategory};
use crate::dbfuncs::query_db;

pub async fn run(options: &[ResolvedOption<'_>]) -> Option<String> {
    let mut player: String = String::new();
    let mut stat: String = String::new();
    let opt = options[0].name;
    match &options[0].value {
        ResolvedValue::SubCommand(subcmds) => {
            for subcmd in subcmds {
                match subcmd.name {
                    "mob" => {
                        match subcmd.value {
                            ResolvedValue::String(val) => {
                                if opt == "kill" {
                                    stat.push_str("kill_");
                                } else if opt == "killed_by" {
                                    stat.push_str("killed_by_");
                                }
                                stat.push_str(&val.replace(" ", "_"));
                            },
                            _ => {},
                        }
                    },
                    "statistic" => {
                        match subcmd.value {
                            ResolvedValue::String(val) => {
                                stat.push_str(val);
                            },
                            _ => {},
                        }
                    },
                    "player" => {
                        match subcmd.value {
                            ResolvedValue::String(val) => player.push_str(val),
                            _ => {},
                        }
                    }
                    _ => {}
                }
            }
        }
        _ => {}
    }

    let mut query: String = format!("SELECT username, \"{}\" FROM statistics", stat);
    if player != "" {
        query += format!(" WHERE username = \"{}\"", &player).as_str();
    } else {
        query += format!(" ORDER BY \"{}\" DESC", stat).as_str();
    }
    query += ";";

    let result: Vec<sqlx::sqlite::SqliteRow> = query_db(query.as_str()).await;

    let mut response: String = String::new();
    response.push_str(&format!("**{}:**\n", &stat));
    for (i, row) in result.iter().enumerate() {
        if stat == "" {
            continue;
        }
        response.push_str(&row.try_get::<String, _>("username").unwrap().to_string());
        response.push_str(": ");
        response.push_str(&row.try_get::<u64, _>(&*stat).unwrap().to_string());
        if i != result.len() - 1 {
            response.push_str("\n");
        }
    }
    if response == "" {
        response.push_str("Not a valid user");
    }
    Some(response)
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
        StatCategory::Use
    );

    // BREAK_STATS
    let break_stat_opts: CreateCommandOption = init_choices(
        CommandOptionType::String,
        "statistic",
        "Choose Crafting Statistic",
        StatCategory::Break
    );

    // TRAVEL_STATS
    let travel_stat_opts: CreateCommandOption = init_choices(
        CommandOptionType::String,
        "statistic",
        "Choose Crafting Statistic",
        StatCategory::Travel
    );

    // MISC_STATS
    let misc_stat_opts: CreateCommandOption = init_choices(
        CommandOptionType::String,
        "statistic",
        "Choose Crafting Statistic",
        StatCategory::Misc
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
            .add_sub_option(player_filter.clone())
        )
        .add_option(
            CreateCommandOption::new(
                CommandOptionType::SubCommand,
                "misc",
                "Statistics about Misc. things"
            )
            .add_sub_option(misc_stat_opts)
            .add_sub_option(player_filter)
        )
        
        
}


fn init_choices(kind: CommandOptionType, name: &str, description: &str, choice_type: StatCategory) -> CreateCommandOption {
    let mut cmd_option: CreateCommandOption = CreateCommandOption::new(
        kind, 
        name,
        description,
    );
    cmd_option = cmd_option.required(true);
    for choice in get_stat_vec(choice_type) {
        cmd_option = cmd_option.add_string_choice(choice.to_string(), choice.to_string());
    }
    cmd_option
}