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

# config.json structure
```json
{
	"discord_token": "<Really.Long.Discord-Token>",    // secret token for your Discord bot
	"guild_id": 000000000000000000,                    // discord server/guild id where the bot will respond, as an integer
	"global_server_addr": "<public_domain>:<port> |"   // assumes you have a website/domain routing to your server
                        ":<port>",                    // assumes localhost
	"rcon_addr_port": "<local_hostname>:<port> |"      // assumes your router implements local DNS
                     "<local_ip_address>:<port> |"    // assumes the computer running the Minecraft Server has a static IP on the local network
                     ":<port>",                       // assumes localhost
	"rcon_pw": "<secure-password>"                     // RCON password
}
```
