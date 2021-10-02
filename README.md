# HarTex-rust-discord-bot

[![GitHub Last Commit](https://img.shields.io/github/last-commit/HT-Studios/HarTex-rust-discord-bot?style=for-the-badge)](https://github.com/HT-Studios/HarTex-rust-discord-bot)
![GitHub Top Language](https://img.shields.io/github/languages/top/HT-Studios/HarTex-rust-discord-bot?style=for-the-badge)
![GitHub](https://img.shields.io/github/license/HT-Studios/HarTex-rust-discord-bot?style=for-the-badge)
![GitHub repo size](https://img.shields.io/github/repo-size/HT-Studios/HarTex-rust-discord-bot?style=for-the-badge)
![GitHub Repo stars](https://img.shields.io/github/stars/HT-Studios/HarTex-rust-discord-bot?style=for-the-badge)
![Minimum Supported Rust Version](https://img.shields.io/badge/rust-1.57-93450a.svg?style=for-the-badge&logo=rust)

HarTex is a Discord Bot primarily for moderation and administration. It is built to be stable, flexible and customizable.

## Honourable Mention

<img src="https://resources.jetbrains.com/storage/products/company/brand/logos/jb_beam.png" alt="JetBrains Logo (Main) logo." width="100">

This project is built with the amazing JetBrains products. Check them out! 

https://www.jetbrains.com/

## Should I run HarTex locally?

Probably not. HarTex has so many moving pieces that running a local instance of it is very complicated. The sole purpose of having 
the source code released and updated frequently, is to allow others to understand the inner workings of the bot and audit its functionality.
You *may* run a local instance of HarTex for your server.

## HarTex Development

### Project Structure

| Crate                  | Description                                                                                      |
| ---------------------- | ------------------------------------------------------------------------------------------------ |
| `hartex`               | The binary of the bot.                                                                           |
| `hartex_cmdsys`        | The command system of the bot.                                                                   |
| `hartex_conftoml`      | The TOML configuration API, deserialization and serialization of TOML configuration of the bot.  |
| `hartex_core`          | The core library of the bot, designed to be as minimal as possible but provides essential types. |
| `hartex_dbmani`        | The database manipulation component of the bot.                                                  |
| `hartex_driver`        | The "main" function of the bot. Glues everything together.                                       |
| `hartex_eventsys`      | The event system of the bot, contains custom events and emitters.                                |
| `hartex_logging`       | The logging infrastructure of the bot, contains a custom logger implementation.                  |
| `hartex_model`         | The various convenience models for the bot to use.                                               |
| `hartex_plugins`       | The plugins (command modules, functionality modules) of the bot.                                 |
| `hartex_utils`         | The various utilities that are useful for the bot.                                               |

### Can I Contribute?

Definitely! Feel free to file issues and pull requests, or even start discussions so discuss various issues or ideas to further
improve the code of the bot or even its functionalities. Contributions are highly welcomed!

### Support Guild

The support guild invite link: [invite link](https://discord.gg/Xu8453VBAv)

Made with :heart:, using [The Rust Programming Language](https://www.rust-lang.org/)
