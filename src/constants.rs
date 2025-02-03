pub enum StatCategory {
    General,
    Craft,
    Mine,
    Use,
    Break,
    Travel,
    Misc,
    Kill,
    KilledBy,
    All
}

pub fn get_stat_vec(stat_type: StatCategory) -> Vec<&'static str> {
    match stat_type {
        StatCategory::General   => GENERAL_STATS.to_vec(),
        StatCategory::Craft     => CRAFT_STATS.to_vec(),
        StatCategory::Mine      => MINE_STATS.to_vec(),
        StatCategory::Break     => BREAK_STATS.to_vec(),
        StatCategory::Use       => USE_STATS.to_vec(),
        StatCategory::Travel    => TRAVEL_STATS.to_vec(),
        StatCategory::Misc      => MISC_STATS.to_vec(),
        StatCategory::Kill      => KILL_STATS.to_vec(),
        StatCategory::KilledBy  => KILLED_BY_STATS.to_vec(),
        StatCategory::All       => {
            let mut all: Vec<&str> = Vec::new();
            all.extend(GENERAL_STATS.iter());
            all.extend(CRAFT_STATS.iter());
            all.extend(BREAK_STATS.iter());
            all.extend(TRAVEL_STATS.iter());
            all.extend(MINE_STATS.iter());
            all.extend(USE_STATS.iter());
            all.extend(KILL_STATS.iter());
            all.extend(KILLED_BY_STATS.iter());
            all.extend(MISC_STATS.iter());
            all
        }
    }
}

pub static GENERAL_STATS: [&str; 10] = [
    "health",
    "level",
    "deaths",
    "total_kills",
    "mob_kills",
    "player_kills",
    "damage_dealt",
    "damage_taken",
    "play_minutes",
    "time_since_last_death"
];

pub static CRAFT_STATS: [&str; 5] = [
    "crafts",
    "craft_beacon",
    "craft_conduit",
    "craft_end_crystal",
    "craft_shulker_box"
];

pub static BREAK_STATS: [&str; 12] = [
    "break_bow",
    "break_diamond_axe",
    "break_diamond_hoe",
    "break_diamond_pickaxe",
    "break_diamond_shovel",
    "break_diamond_sword",
    "break_netherite_axe",
    "break_netherite_hoe",
    "break_netherite_pickaxe",
    "break_netherite_shovel",
    "break_netherite_sword",
    "break_shears"
];

pub static TRAVEL_STATS: [&str; 12] = [
    "walk",
    "sprint",
    "crouch",
    "sneak",
    "jumps",
    "aviate",
    "fall",
    "minecart_ride",
    "horse_rode",
    "swim_km",
    "elytra_km",
    "pig_ride"
];

pub static MINE_STATS: [&str; 9] = [
    "mine_all_coal",
    "mine_all_copper",
    "mine_all_diamond",
    "mine_all_emerald",
    "mine_all_gold",
    "mine_all_iron",
    "mine_all_lapis",
    "mine_all_redstone",
    "mine_ancient_debris"
];

pub static USE_STATS: [&str; 11] = [
    "spy_glass_used",
    "use_bonemeal",
    "use_golden_apple",
    "use_potion",
    "use_torch",
    "use_totem",
    "thrown_bottle_o_enchanting",
    "thrown_ender_pearl",
    "thrown_eye_of_ender",
    "thrown_snowball",
    "thrown_trident"
];

pub static MISC_STATS: [&str; 22] = [
    "animals_bred",
    "bells_rung",
    "brewing",
    "buckets_filled",
    "cake_eaten",
    "cast_fishing_rod",
    "chest_opened",
    "drink_honey_bottle",
    "ender_chest",
    "fish_caught",
    "flower_potted",
    "furnace_used",
    "items_enchanted",
    "lava_buckets_emptied",
    "play_noteblock",
    "records_played",
    "shulker_box_used",
    "trap_chest_open",
    "tune_noteblock",
    "villager_trade",
    "water_buckets_emptied",
    "wheat_planted"
];

pub static KILL_STATS: [&str; 77] = [
    "kill_armadillo",
    "kill_axolotl",
    "kill_bat",
    "kill_bee",
    "kill_blaze",
    "kill_bogged",
    "kill_breeze",
    "kill_camel",
    "kill_cave_spider",
    "kill_chicken",
    "kill_cod",
    "kill_count",
    "kill_cow",
    "kill_creeper",
    "kill_dolphin",
    "kill_donkey",
    "kill_drowned",
    "kill_elder_guardian",
    "kill_ender_dragon",
    "kill_enderman",
    "kill_endermite",
    "kill_evoker",
    "kill_fox",
    "kill_ghast",
    "kill_glow_squid",
    "kill_goat",
    "kill_guardian",
    "kill_hoglin",
    "kill_horse",
    "kill_husk",
    "kill_illusioner",
    "kill_iron_golem",
    "kill_llama",
    "kill_magma_cube",
    "kill_mooshroom",
    "kill_mule",
    "kill_ocelot",
    "kill_panda",
    "kill_parrot",
    "kill_phantom",
    "kill_pig",
    "kill_piglin",
    "kill_pillager",
    "kill_polar_bear",
    "kill_pufferfish",
    "kill_rabbit",
    "kill_ravager",
    "kill_salmon",
    "kill_sheep",
    "kill_shulker",
    "kill_silverfish",
    "kill_skeleton",
    "kill_skeleton_horse",
    "kill_slime",
    "kill_sniffer",
    "kill_snow_golem",
    "kill_spider",
    "kill_squid",
    "kill_stray",
    "kill_strider",
    "kill_trader_llama",
    "kill_tropical_fish",
    "kill_turtle",
    "kill_vex",
    "kill_villager",
    "kill_vindicator",
    "kill_wandering_trader",
    "kill_warden",
    "kill_witch",
    "kill_wither",
    "kill_wither_skeleton",
    "kill_wolf",
    "kill_zoglin",
    "kill_zombie",
    "kill_zombie_horse",
    "kill_zombie_villager",
    "kill_zombified_piglin"
];

pub static KILLED_BY_STATS: [&str; 55] = [
    "killed_by_bee",
    "killed_by_blaze",
    "killed_by_bogged",
    "killed_by_breeze",
    "killed_by_cave_spider",
    "killed_by_creeper",
    "killed_by_dolphin",
    "killed_by_drowned",
    "killed_by_elder_guardian",
    "killed_by_ender_dragon",
    "killed_by_enderman",
    "killed_by_endermite",
    "killed_by_evoker",
    "killed_by_fox",
    "killed_by_ghast",
    "killed_by_guardian",
    "killed_by_hoglin",
    "killed_by_husk",
    "killed_by_illusioner",
    "killed_by_iron_golem",
    "killed_by_llama",
    "killed_by_magma_cube",
    "killed_by_panda",
    "killed_by_phantom",
    "killed_by_piglin",
    "killed_by_pillager",
    "killed_by_polar_bear",
    "killed_by_pufferfish",
    "killed_by_rabbit",
    "killed_by_ravager",
    "killed_by_shulker",
    "killed_by_silverfish",
    "killed_by_skeleton",
    "killed_by_skeleton_horse",
    "killed_by_slime",
    "killed_by_snow_golem",
    "killed_by_spider",
    "killed_by_stray",
    "killed_by_strider",
    "killed_by_trader_llama",
    "killed_by_turtle",
    "killed_by_vex",
    "killed_by_villager",
    "killed_by_vindicator",
    "killed_by_wandering_trader",
    "killed_by_warden",
    "killed_by_witch",
    "killed_by_wither",
    "killed_by_wither_skeleton",
    "killed_by_wolf",
    "killed_by_zoglin",
    "killed_by_zombie",
    "killed_by_zombie_horse",
    "killed_by_zombie_villager",
    "killed_by_zombified_piglin"
];