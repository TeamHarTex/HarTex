HarTex-rust-discord-bot
=======================

[![HarTex Community](https://img.shields.io/discord/886101109331075103?color=%237289DA&label=HarTex%20Community&logo=discord&style=for-the-badge)](https://discord.gg/Xu8453VBAv)

[![GitHub Badge](https://img.shields.io/badge/github-HarTex-6f42c1.svg?style=for-the-badge&logo=github)](https://github.com/HT-Studios/HarTex-rust-discord-bot)
[![License](https://img.shields.io/github/license/HarTexTeam/HarTex-rust-discord-bot?style=for-the-badge&logo=pastebin)](https://www.apache.org/licenses/LICENSE-2.0.txt)
![Minimum Supported Rust Version](https://img.shields.io/badge/rust-1.59-93450a.svg?style=for-the-badge&logo=rust)

![Alt](https://repobeats.axiom.co/api/embed/19c38ac467e75c4e7bb533031896ac6e299321c6.svg "Repobeats analytics image")

HarTex is a Discord Bot primarily for moderation and administration. It is built to be stable, flexible and customizable.

Honourable Mention
------------------

<img src="https://resources.jetbrains.com/storage/products/company/brand/logos/jb_beam.png" alt="JetBrains Logo (Main) logo." width="100">

This project is built with the amazing JetBrains products. [Check them out!](https://www.jetbrains.com/)

Should I run HarTex locally?
----------------------------

Probably not. HarTex has so many moving pieces that running a local instance of it is very complicated. The sole purpose of having
the source code released and updated frequently, is to allow others to understand the inner workings of the bot and audit its functionality.
You *may* run a local instance of HarTex for your guild.

HarTex Development
------------------

### Project Structure

| Crate             | Description                                                                                      |
|-------------------|--------------------------------------------------------------------------------------------------|
| `hartex`          | The binary of the bot.                                                                           |
| `hartex_cmdsys`   | The command system of the bot.                                                                   |
| `hartex_conftoml` | The TOML configuration API, deserialization and serialization of TOML configuration of the bot.  |
| `hartex_core`     | The core library of the bot, designed to be as minimal as possible but provides essential types. |
| `hartex_dbmani`   | The database manipulation component of the bot.                                                  |
| `hartex_driver`   | The "main" function of the bot. Glues everything together.                                       |
| `hartex_env`      | The wrappers around a collection of useful environment variables used by the bot.                |
| `hartex_eventsys` | The event system of the bot, contains custom events and emitters.                                |
| `hartex_model`    | The various convenience models for the bot to use.                                               |
| `hartex_plugins`  | The plugins (command modules, functionality modules) of the bot.                                 |
| `hartex_utils`    | The various utilities that are useful for the bot.                                               |

### Can I Contribute?

Definitely! Feel free to file issues and pull requests, or even start discussions so discuss various issues or ideas to further
improve the code of the bot or even its functionalities. Contributions are highly welcomed!

### Community Guild

[There is an official Discord community guild for this project!](https://discord.gg/Xu8453VBAv)

You may follow the latest developments as well as ask for support in the guild.

Made with :heart:, using [The Rust Programming Language](https://www.rust-lang.org/)