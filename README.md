# HarTex, a Rust Discord Bot

[![HarTex Community](https://img.shields.io/discord/886101109331075103?color=%237289DA&label=HarTex%20Community&logo=discord&style=for-the-badge)](https://discord.gg/Xu8453VBAv)

[![GitHub Badge](https://img.shields.io/badge/github-HarTex-6f42c1.svg?style=for-the-badge&logo=github)](https://github.com/HT-Studios/HarTex-rust-discord-bot)
[![License](https://img.shields.io/github/license/HarTexTeam/HarTex-rust-discord-bot?style=for-the-badge&logo=pastebin)](https://www.apache.org/licenses/LICENSE-2.0.txt)
![Minimum Supported Rust Version](https://img.shields.io/badge/rust-1.60-93450a.svg?style=for-the-badge&logo=rust)

![Alt](https://repobeats.axiom.co/api/embed/19c38ac467e75c4e7bb533031896ac6e299321c6.svg "Repobeats analytics image")

A Discord bot primarily for moderation and administration; built to be stable, flexible and
customizable.

## Honourable Mention

<img src="https://resources.jetbrains.com/storage/products/company/brand/logos/jb_beam.png" alt="JetBrains Logo (Main) logo." width="100">

This project is built with the amazing JetBrains products. [Check them out!](https://www.jetbrains.com/)

## Project Information

### Minimum Supported Rust Version (MSRV)

This project has an MSRV of `1.60.0`. What this means is that, a Rust compiler of version
`1.60.0` or above is required to build and run the entire project.

`1.60.0` is typically the latest nightly version of the compiler - we use the nightly release
channel because there are certain features we need are gated behind a feature flag which is
only usable when using the nightly compiler.

### Project Structure

This project is organized with a Cargo workspace which includes all the crates in the root
directory of the repository.

Individual crates have their own detailed description and README (except the `hartex` binary
crate, which such description is included in this main README). Additionally, brief and simple
descriptions of each crate will be provided in this root README.

#### `hartex`

This is the main binary of the bot. This crate contains the code that briefly sets up the
runtime environment, including:
- loading the environment variables from a `.env` file for later manipulation;
- initializing the `tokio` runtime for asynchronous and multithreaded execution

After that, the "actual" main entry point in the `hartex_driver` crate is executed, which
handles the rest.

#### `hartex_base`

This is the base crate for the bot, which contains minimum and abstract functionality for the
bot.

#### `hartex_cache_base`

This is the base framework for the caching implementation of the bot.

#### `hartex_cache_discord`

This is a Discord object cache for the bot, built upon `hartex_cache_base`.

#### `hartex_cmdsys`

This is an implementation of a command system for the bot.

#### `hartex_conftoml`

This crate implements the TOML Configuration API of the bot.

#### `hartex_dbmani`

This crate contains various database manipulation functions for the bot.

#### `hartex_driver`

This crate is the actual "main entry point" of the bot.

#### `hartex_env`

This crate is a wrapper over the various environment variables that the bot makes use of.

#### `hartex_locale`

This crate contains a base framework, and provides various definitions for locales.

#### `hartex_localization_impl`

This crate contains an implementation of localization for the bot, built upon `hartex_locale`.

#### `hartex_model`

This crate contains various models for the bot.

#### `hartex_plugins`

This crate implements the various modules of the bot (commands, background tasks, etc.).

#### `hartex_utils`

This crate provides utilities for the bot codebase.

### Community Guild

[There is an official Discord community guild for this project!](https://discord.gg/Xu8453VBAv)

You may follow the latest developments as well as ask for support in the guild.

Made with :heart:, using [The Rust Programming Language](https://www.rust-lang.org/)