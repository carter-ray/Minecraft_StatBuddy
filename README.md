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

`/ip` - Returns the public IP address of your minecraft server

`/stat <category> <statistic_name> [player]` - Returns statistics from the minecraft server

# .env keys
```env
DISCORD_BOT_TOKEN=<Really.Long.Discord-Token>   		      // secret token for your Discord bot
GUILD_ID=999999999999999999							            // discord server/guild id where the bot will respond
GLOBAL_SERVER_ADDRESS=<public_domain:port>|<:port>          // assumes localhost:25565 if missing
RCON_ADDR_PORT=<static_ip:port>|<hostname:port>|<:port>     // assumes localhost:25575 if missing
RCON_PW=<secure-password>
```
