/**
 * File names of well-known games, for when no game library lists one. Only ever compared with what's
 * already running, and only to put a name on it: nothing here starts, stops or changes a game.
 */
const KNOWN_GAMES: Record<string, string> = {
  "valorant-win64-shipping.exe": "VALORANT",
  "league of legends.exe": "League of Legends",
  "overwatch.exe": "Overwatch",
  "robloxplayerbeta.exe": "Roblox",
  "fortniteclient-win64-shipping.exe": "Fortnite",
  "cs2.exe": "Counter-Strike 2",
  "rocketleague.exe": "Rocket League",
  "r5apex.exe": "Apex Legends",
  "r5apex_dx12.exe": "Apex Legends",
  "gta5.exe": "Grand Theft Auto V",
  "rustclient.exe": "Rust",
  "marvel-win64-shipping.exe": "Marvel Rivals",
  "dota2.exe": "Dota 2",
  "rainbowsix.exe": "Rainbow Six Siege",
  "destiny2.exe": "Destiny 2",
  "warframe.x64.exe": "Warframe",
  "haloinfinite.exe": "Halo Infinite",
  "escapefromtarkov.exe": "Escape from Tarkov",
  "deadbydaylight-win64-shipping.exe": "Dead by Daylight",
  "cyberpunk2077.exe": "Cyberpunk 2077",
  "rdr2.exe": "Red Dead Redemption 2",
  "eldenring.exe": "Elden Ring",
  "discovery.exe": "The Finals",
  "starfield.exe": "Starfield",
  "bg3.exe": "Baldur's Gate 3",
  "bg3_dx11.exe": "Baldur's Gate 3",
  "palworld-win64-shipping.exe": "Palworld",
  "helldivers2.exe": "Helldivers 2",
  "tslgame.exe": "PUBG: Battlegrounds",
  "minecraft.windows.exe": "Minecraft",
  "hd-player.exe": "BlueStacks",
  "dnplayer.exe": "LDPlayer",
  "nox.exe": "NoxPlayer",
};

export const knownGame = (fileName: string): string | undefined =>
  KNOWN_GAMES[fileName.toLowerCase()];
