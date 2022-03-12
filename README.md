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

### Community Guild

[There is an official Discord community guild for this project!](https://discord.gg/Xu8453VBAv)

You may follow the latest developments as well as ask for support in the guild.

### Minimum Supported Rust Version (MSRV)

This project has an MSRV of `1.61.0`. What this means is that, a Rust compiler of version
`1.61.0` or above is required to build and run the entire project.

`1.61.0` is typically the latest nightly version of the compiler - we use the nightly release
channel because there are certain features we need are gated behind a feature flag which is
only usable when using the nightly compiler.

### Project Structure

This project is organized with a Cargo workspace which includes all the crates in the root
directory of the repository.

#### `base`

The base library for the bot. Contains the most minimal features for the codebase to make use of

#### `cache_base`

Base framework for caching. Custom backends are supported by implementing the `Backend` trait in
the library.

#### `cache_discord`

The cache for Discord objects.

#### `env`

Convenience wrappers for interacting with the process environment. Environment variables, for example.

#### `event`

The standalone event handler and processor.

#### `ext`

Useful extensions to various libraries used by the bot.

#### `gateway`

The standalone process that connects to the Discord gateway; sends requests to the event handler processor
when events are received from the Discord gateway.

#### `manidb`

Database manipulation procedures for the codebase. These are used to interact with the databases used by
the bot.

#### `model`

Common and useful models used throughout the codebase.

#### `rest`

The standalone REST process that acts as a proxy over the Discord API.

Made with :heart:, using [The Rust Programming Language](https://www.rust-lang.org/)
