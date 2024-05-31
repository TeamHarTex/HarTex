# [Next Version, Sodium I](https://github.com/TeamHarTex/HarTex/compare/0.10.0...nightly)

> **x** contributor made contributions to this release

> **y** commits since 2024-04-30

## API Backend

- **Changed:** updated `rust-version` to 1.80

## Buildsystem

- **Changed:** updated `rust-version` to 1.80

## Database Infrastructure

- **Added:** added database migration tool utilising `refinery`
- **Added:** database queries to work with configuration as `JSONB` objects
- **Changed:** updated `rust-version` to 1.80

## Discord Frontend

- **Added:** `plugin` attribute macro and `Plugin` traits
- **Added:** plugin-related functions to `CommandMetadata` trait
- **Added:** checks for whether plugins are enabled before running one of their commands
- **Changed:** lookup tables are now used for command dispatch rather than `match`
- **Changed:** extracted configuration models to its own crate (`serde` support included)
- **Changed:** updated `rust-version` to 1.80
- **Changed:** `/info emoji` now sends embeds
- **Changed:** more information has been added to `/info emoji`
- **Changed:** `/info emoji` now handles the error of an emoji not being found from database
- **Changed:** removed previously deprecated functions from `CommandMetadata`
- **Changed:** deprecated `command_type` and `interaction_only` in `CommandMetadata`
- **Removed:** `redis` dependency removed

## Localization Infrastructure

- **Added:** added messages to Fluent for more commands
- **Changed:** updated `rust-version` to 1.80

## Rust Utilities

- **Added:** added utility functions for working with `CommandDataOption`
- **Changed:** updated `rust-version` to 1.80

## Web Frontend

# [0.10.0, Neon I](https://github.com/TeamHarTex/HarTex/compare/0.9.0..0.10.0)

> **3** contributors made contributions to this release

> **277** commits since 2024-03-08

## API Backend

- **Added:** code documentation
- **Changed:** updated `rust-version` to 1.79

## Buildsystem

- **Added:** code documentation
- **Added:** setup procedure for the Zed editor for working with HarTex
- **Changed:** updated `rust-version` to 1.79

## Database Queries

- **Added:** code documentation
- **Changed:** updated `rust-version` to 1.79

## Discord Frontend

- **Added:** `/info emoji` command
- **Added:** error handling for interaction commands only
- **Added:** panic handling via unwinding for interaction commands only
- **Added:** error reporting in internal channels and user-facing messages
- **Added:** code documentation
- **Changed:** updated `rust-version` to 1.79
- **Changed:** updated cache entities and repositories

## Localization Infrastructure

- **Added:** code documentation
- **Changed:** updated `rust-version` to 1.79

## Rust Utilities

- **Added:** code documentation
- **Changed:** updated `rust-version` to 1.79

## Web Frontend

- **Changed:** migrated to the Bun runtime

# [0.9.0, Fluorine I](https://github.com/TeamHarTex/HarTex/compare/0.8.0..0.9.0)

> **4** contributors made contributions to this release

> **205** commits since 2024-02-05

## API Backend

- **Changed:** renamed `.cargo/config` to `.cargo/config.toml`

## Buildsystem

- **Changed:** renamed `.cargo/config` to `.cargo/config.toml`

## Database Queries

- **Changed:** renamed `.cargo/config` to `.cargo/config.toml`

## Discord Frontend

- **Added:** added role creation relative timestamp to `/info role`
- **Added:** added more information to `/info user`
- **Changed:** renamed `.cargo/config` to `.cargo/config.toml`
- **Changed:** migrated from `rlua` to `mlua`
- **Changed:** extracted caching process from worker

## Localization Infrastructure

- **Changed:** renamed `.cargo/config` to `.cargo/config.toml`

## Rust Utilities

- **Changed:** renamed `.cargo/config` to `.cargo/config.toml`

# [0.8.0, Oxygen I](https://github.com/TeamHarTex/HarTex/compare/0.7.1..0.8.0)

> **4** contributors made contributions to this release

> **566** commits since 2024-01-01

## API Backend

- **Changed:** bumped `rust-version` to 1.78.0

## Buildsystem

- **Added:** added x.py and related bootstrap barebones
- **Added:** added various x.py commands
- **Added:** sample configuration files
- **Changed:** bumped `rust-version` to 1.78.0
- **Removed:** removed old buildsystem

## Database Queries

- **Changed:** updated database queries

## Discord Frontend

- **Added:** added more conversions to entity macro
- **Added:** added inter-entity relations to entity macro
- **Changed:** bumped `rust-version` to 1.78.0
- **Changed:** migrated to Lua configuration
- **Changed:** modified cached guild entity fields
- **Changed:** modified cached user entity fields
- **Changed:** updated `/info server` command to add extra information
- **Changed:** migrated tests to the new testing infrastructure

## Localization Infrastructure

- **Added:** added more localization keys
- **Changed:** bumped `rust-version` to 1.78.0

## Rust Utilities

- **Added:** added more types to be localizable
- **Changed:** bumped `rust-version` to 1.78.0

## Web Frontend

- **Added:** reimplementation of the main page

# [0.7.1, Nitrogen II](https://github.com/TeamHarTex/HarTex/compare/0.7.0..0.7.1)

> **1** contributor made contributions to this release

> **11** commits since 2024-01-01

## Discord Frontend

- **Changed:** de-`Arc` the bot gateway queue as per changes in twilight

# [0.7.0, Nitrogen I](https://github.com/TeamHarTex/HarTex/compare/0.6.0..0.7.0)

> **4** contributor made contributions to this release

> **360** commits since 2023-12-10

## API Backend

- **Added:** added `PATCH /stats/uptime` API
- **Changed:** use database pools for database queries
- **Changed:** bumped `rust-version` to 1.77.0
- **Changed:** increment copyright year to 2024

## Buildsystem

- **Added:** added `update` commands to update dependencies of (a) project(s)
- **Added:** added task completion duration
- **Changed:** bumped `rust-version` to 1.77.0
- **Changed:** increment copyright year to 2024

## Database Queries

- **Added:** added various queries for cache usage
- **Changed:** bumped `rust-version` to 1.77.0
- **Changed:** increment copyright year to 2024

## Discord Frontend

- **Added:** added framework for `/info user` command
- **Changed:** migrated cache repositories to PostgreSQL
- **Changed:** completed `/info role` command
- **Changed:** use database pools for database queries
- **Changed:** bumped `rust-version` to 1.77.0
- **Changed:** implemented nitro boosting information to `/info server` command
- **Changed:** imlpemented nitro boosting related fields to guild entity
- **Fixed:** fixed the `/info bot` command
- **Fixed:** fixed the requests sent by command manager
- **Fixed:** fixed the `--with-localizations` flag in the command manager
- **Changed:** increment copyright year to 2024

## Localization Infrastructure

- **Added:** added various localizatoin entries for more features
- **Fixed:** fixed path related bugs when running the bot and looking for the localization bundles
- **Changed:** bumped `rust-version` to 1.77.0
- **Changed:** increment copyright year to 2024

## Rust Utilities

- **Added:** added database pool
- **Added:** added functionality to create a TCP stream with TLS for use with hyper to use HTTPS
- **Added:** make a twilight `PremiumTier` localizable
- **Changed:** bumped `rust-version` to 1.77.0
- **Changed:** increment copyright year to 2024

## Web Frontend

- **Changed:** increment copyright year to 2024

# [0.6.0, Carbon I](https://github.com/TeamHarTex/HarTex/compare/0.5.1..0.6.0)

> **1** contributor made contributions to this release

> **339** commits since 2023-11-05

## API Backend

- **Added:** added `hartex-backend-layers` crate
- **Changed:** updated Rust Version to 1.76
- **Changed:** changed `Response<T>` to allow `DeserializedOwned` requirements
- **Changed:** initiated and partial migration to `axum`
- **Changed:** leverage parallel rustc compilation frontend

## Bors Buildbot

- **Removed:** bors in-tree is now EOL

## Buildsystem

- **Added:** added `clean` command to clean build artifacts
- **Changed:** updated Rust Version to 1.76
- **Changed:** leverage parallel rustc compilation frontend

## Database Queries

- **Changed:** updated Rust Version to 1.76
- **Changed:** leverage parallel rustc compilation frontend

## Discord Frontend

- **Changed:** migrated the commands to use the new localization API
- **Changed:** updated `hyper` crate to `1.0` and performed corresponding migrations
- **Changed:** leverage parallel rustc compilation frontend

## Localization Infrastructure

- **Added:** added `hartex-localizations-bindings` and `hartex-localization-loader` crates
- **Changed:** new API in `hartex-localization-core`
- **Changed:** updated Rust Version to 1.76
- **Changed:** leverage parallel rustc compilation frontend
- **Removed:** removed old `bundle_get` and `bundle_get_args` macros
- **Removed:** removed old types from `hartex-localization-core`

## Rust Utilities

- **Changed:** updated Rust Version to 1.76
- **Changed:** leverage parallel rustc compilation frontend

# [0.5.1, Boron II](https://github.com/TeamHarTex/HarTex/compare/0.5.0..0.5.1)

> **1** contributor made contributions to this release

> **57** commits since 2023-11-03

## API Backend

- **Added:** introduced API v2
- **Removed:** discontinued API v1

## Bors Buildbot

- **Changed:** updated `hartex-bors-dashboard` website
- **Changed:** migrated to API v2

## Discord Frontend

- **Changed:** migrated to API v2

## Web Frontend

- **Changed:** app components updated

# [0.5.0, Boron I](https://github.com/TeamHarTex/HarTex/compare/0.4.0..0.5.0)

> **2** contributors made contributions to this release

> **272** commits since 2023-09-29

## API Backend

- **Changed:** updated Rust Version to 1.75
- **Changed:** migrated to generated type-checked SQL queries
- **Removed:** phased out `sqlx` deoendency

## Bors Buildbot

- **Added:** initiated leptos-based `hartex-bors-dashboard`
- **Changed:** updated Rust Version to 1.75
- **Changed:** deprecated `hartex-bors-website`

## Buildsystem

- **Changed:** updated Rust Version to 1.75

## Database Queries

- **Changed:** rewritten in Rust

## Discord Frontend

- **Added:** complete yet feature-incomplete implementation of the new `entity` macro
- **Changed:** implemented version and type metadata for `twilight-model`
- **Changed:** updated Rust Version to 1.75
- **Changed:** migrated to generated type-checked SQL queries
- **Removed:** removed deprecated `Entity` derive macro
- **Removed:** phased out `sqlx` dependency

## Localization Infrastructure

- **Changed:** updated Rust Version to 1.75

## Rust Utilities

- **Added:** new crate for macro utilities
- **Changed:** updated Rust Version to 1.75

# [Version 0.4.0, Beryllium I (2023-09-29)](https://github.com/TeamHarTex/HarTex/compare/0.3.0..0.4.0)

> **4** contributors made contributions to this release

> **321** commits since 2023-08-23

## API Backend

- **Changed:** unpinned `serde`

## Bors Buildbot

- **Fixed:** fixed `octocrab` dependency
- **Changed:** unpinned `serde`

## Buildsystem

- **Changed:** unpinned `serde`

## Discord Frontend

- **Added:** `metadata` macro for writing command metadata, replacing `CommandMetadata` derive
- **Added:** `/info role` command
- **Changed:** pinned serde to version 1.0.185
- **Changed:** deprecated `minimum_level` function of the `CommandMetadata` trait
- **Changed:** deprecated `Entity` macro derive trait
- **Changed:** updated `/info server` command with more information
- **Removed:** removed deprecated `CommandMetadata` derive macro

## Rust Utilities

- **Added**: some APIs for localizing data types

# [Version 0.3.0, Lithium I (2023-08-23)](https://github.com/TeamHarTex/HarTex/compare/0.2.1...0.3.0)

> **4** contributors made contributions to this release

> **516** commits since 2023-07-16

## API Backend

- **Changed:** pinned serde to version 1.0.185
- **Fixed:** returning in the uptime route

## Bors Buildbot

- **Changed:** pinned serde to version 1.0.185
- **Changed:** migrated from eyre to miette for error reporting

## Buildsystem

- **Changed:** migrated from eyre to miette for error reporting
- **Changed:** pinned serde to version 1.0.185

## Discord Frontend

- **Added:** `/info server` and verbose filtering
- **Changed:** migrated from eyre to miette for error reporting
- **Changed:** allowed workers to send gateway commands to leaders
- **Changed:** updated cache entities and repositories
- **Changed:** replaced `/latency` and `/uptime` with `/info bot`
- **Changed:** deprecated `CommandMetadata` derive macro
- **Changed:** pinned serde to version 1.0.185

## Localization Infrastructure

- **Added:** fallback to `en-GB` if locale is not specified or locale is not found in `locales` folder
- **Added:** translations for locale `ja` (Japanese)
- **Changed:** migrated from eyre to miette for error reporting
- **Changed:** updated localizations
- **Fixed:** do not panic if key is not found

## Rust Utilities

- **Changed:** removed hartex-eyre

# [Version 0.2.1, Helium II (2023-07-16)](https://github.com/TeamHarTex/HarTex/compare/0.2.0...0.2.1)

> **1** contributor made contributions to this release

> **16** commits since 2023-07-16

## Bors Buildbot

- **Fixed:** compiler errors

# [Version 0.2.0, Helium I (2023-07-16)](https://github.com/TeamHarTex/HarTex/compare/0.1.0...0.2.0)

> **5** contributors made contributions to this release

> **388** commits since 2023-06-14

## API Backend

- **Changed:** switched from ScyllaDB to PostgreSQL
- **Changed:** migrated from eyre to miette for error reporting
- **Changed:** updated rust version to 1.73.0

## Bors Buildbot

- **Added:** pull request queue
- **Changed:** updated enqueued pull request model
- **Changed:** updated command parsing to support passing parameters to commands
- **Changed:** modified try command to optionally accept a parent commit for the try build creation
- **Changed:** migrated from eyre to miette for error reporting
- **Changed:** updated rust version to 1.73.0

## Buildsystem

- **Added:** support for passing multiple projects to individual commands
- **Changed:** changed the algorithm to not bail out when a project is not found; skipping it and continue instead
- **Changed:** migrated from eyre to miette for error reporting
- **Changed:** updated rust version to 1.73.0

## Database Queries

- **Added:** basic queries for usage in backend and Discord frontend

## Discord Frontend

- **Added:** `/serverinfo` command
- **Changed:** updated configuration API
- **Changed:** updated configuration version computation algorithm
- **Changed:** corresponding changes to uptime updates with the API
- **Changed:** switched from ScyllaDB to PostgreSQL
- **Changed:** updated rust version to 1.73.0

## Localization Infrastructure

- **Changed:** updated rust version to 1.73.0

## Rust Utilities

- **Added:** a dedicated crate for errors
- **Changed:** updated rust version to 1.73.0

## Web Frontend

- **Added:** documentation website
- **Added:** introduction page

# [Version 0.1.0, Hydrogen I (2023-06-14)](https://github.com/TeamHarTex/HarTex/commits/0.1.0)

> **5** contributors made contributions to this release

> **2983** commits since 2022-08-31

## API Backend

- **Added:** `/bors` and `/uptime` APIs

## Bors Buildbot

- **Added:** commands: `try`, `try-`, `r+`, `r=`
- **Added:** pull request label modification on various events

## Buildsystem

- **Added:** project types: `rust`, `jsts`
- **Added:** support for building and linting projects

## Discord Frontend

- **Added:** commands implemented: `/about`, `/contributors`, `latency`, `/uptime`
- **Added:** database caching
- **Added:** configuration API draft
- **Changed:** major infrastructure and codebase structure revamp

## Localization Infrastructure

- **Added:** Project Fluent as foundation

## Rust Utilities

- **Added:** various utilities

## Web Frontend

- **Added:** basic landing page
