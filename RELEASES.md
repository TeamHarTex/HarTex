HarTex 1.25.0 (19 December 2021)
================================

Key Changes
-----------
- implemented basic localisation providing three locales: `en_GB`, `en_US` and `zh_HK`
- added `locale` field in `GuildConfiguration` TOML Configuration API object
- introduction of `*.langcfg` language configuration files for localisation purposes
- various updates to usage documentation

HarTex 1.24.0 (12 December 2021)
================================

Key Changes
-----------
- re-licensed with AGPL-3.0
- make use of `query_as<T>` for the `guildconf` dbmani future
- implement branch checks to guard against nightly feature usage on stable

HarTex 1.23.1 (6 December 2021)
================================

Key Changes
-----------
- use bulk overwrite for interaction commands

HarTex 1.23.0 (5 December 2021)
================================

Key Changes
-----------
- implement `refroles` command
- add `PermissionLevels` configuration object
- introduce `nightly` and `stable` branches

HarTex 1.22.0 (10 November 2021)
================================

Key Changes
-----------
- code improvements (rustfmt & clippy)
- create `hartex_env` crate
- panic handler added

HarTex 1.21.0 (8 October 2021)
================================

Key Changes
-----------

- removed `hartex_logging` crate in favour of `tracing`
- removed the pre-command check API
- use the stabilized Rust 2021 Edition
- add `userinfo` command
- add `guildinfo` command
