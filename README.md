# Minecraft Stat Buddy
Stat Buddy is a [Discord](https://discord.com/) bot that interfaces with a home minecraft server through RCON to obtain statistics tracked by [Vanilla Tweaks](https://vanillatweaks.net/picker/datapacks/)

## How to use
 - Suggested: Host this yourself on the same network as your Minecraft server to avoid port forwarding your RCON port.
 - Suggested: Do not use the same port for RCON as you would to connect to the server if you are port forwarding.
 - Enable RCON,
    - Suggested: Set a nice, long, complex password
 - Download the "Track Statistics" and "Track Raw Statistics" datapacks from [Vanilla Tweaks](https://vanillatweaks.net/picker/datapacks/)
 - Add these packs to your worlds datapack folder.
 - Restart the Minecraft server.

 
## Requirements
- Minecraft Server
- Discord
- RCON Enabled (strong password recommended)
- VanillaTweaks's "Track Statistics" datapack
- VanillaTweaks's "Track Raw Statistics" datapack


# Commands

`/server <domain|ip>` - Set the domain or IP of your minecraft server
`/getip` - Returns the public IP address of your minecraft server

All commands can optionally be provided a player to get stats for a single player.

`/playTime [player]` - Gets the playtime
`/elyraKm [player]` - Gets the Km flown by elytra
`/swimKm [player]` - Gets the Km swam by players
`/MineCoal [player]` - Gets the number of Coal mined by
`/MineCopper [player]` - Gets the number of Copper mined by players
`/MineDiamond [player]` - Gets the number of Diamonds mined by players
`/MineIron [player]` - Gets the number of Iron mined by players
`/MineGold [player]` - Gets the number of Gold mined by players
`/MineRedstone [player]` - Gets the number of Redstone mined by players
`/MineLapis [player]` - Gets the number of Lapis mined by players
`/MineEmerald [player]` - Gets the number of Emerald mined by players
`/MineNetherite [player]` - Gets the number of Netherite mined by players
`/MineQuartz [player]` - Gets the number of Quarts mined by players
`/MineOre [player]` - Gets the total count of Ores mined by players
`/Deaths [player]` - Gets the number of Deaths players have
`/Crafts [player]` - Gets the number of crafts players have
`/FurnaceUsed [player]` - Gets the number of times players have used a furnace.
`/VillagerTrade [player]` - Gets the number of times players have traded with a villager
`/Jump [player]` - Gets the number of times players have jumped
`/AnimalsBred [player]` - Gets the number of times players have bred animals
`/DamageDealt [player]` - Gets the amount of damage players have dealt
`/DamageTaken [player]` - Gets the amount of damage players have taken
`/Brewing [player]` - Gets the number of times players have used a Brewing stand
`/ChestOpened [player]` - Gets the number of Chests players have opened
`/ShulkerBox [player]` - Gets the number of Shulker Boxes players have opened
`/EnderChest [player]` - Gets the number of Ender Chests players have opened
`/UseTorch [player]` - Gets the number of Torches players have placed
`/UseTotem [player]` - Gets the number of Totems of Undying players have used
`/FishCaught [player]` - Gets the number of Fish players have caught
`/EnchantItem [player]` - Gets the number of items enchaned by players
`/TotalKills [player]` - Gets the total kills by players
`/RecordsPlayed [player]` - Gets the number of Records Played by players
`/FlowerPotted [player]` - Gets the number of Flowers Potted by players
`/BreakAxe [player]` - Gets the number of Axes broken
`/BreakBow [player]` - Gets the number of Bows broken
`/BreakHoe [player]` - Gets the number of Hoes broken
`/BreakPick [player]` - Gets the number of Picaxes broken
`/BreakShears [player]` - Gets the number of Shears broken
`/BreakShovel [player]` - Gets the number of Shovels broken
`/BreakSword [player]` - Gets the number of Swords broken
`/CraftShulkBox [player]` - Gets the number of Shulker Boxes crafted
`/CraftEndCryst [player]` - Gets the number of End Crystals crafted
`/CraftConduit [player]` - Gets the number of Conduits crafted
`/CraftBeacon [player]` - Gets the number of Beacons crafted
`/Dth [player] <mob> ` - Gets the number of player Deaths caused by a given `Mob`
`/Kill <mob> [player]` - Gets the number of kills for the `Mob`
`/UseEnderPearl [player]` - Gets the number of ender pearls used
`/TuneNoteblock [player]` - Gets the number of Noteblocks tuned
`/UseBonemeal [player]` - Gets the number of Bonemeal used
`/UseBottleEnch [player]` - Gets the number of Bottles of Enchanting used
`/UseBucket [player]` - Gets the number of Buckets used
`/UseFishingRod [player]` - Gets the number of Fishing Rods used
`/UseGoldApple [player]` - Gets the number of Golden Apples used
`/UsePotion [player]` - Gets the number of Potions used
`/UseTrident [player]` - Gets the number of Tridents used