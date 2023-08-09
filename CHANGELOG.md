# [Next Version](https://github.com/TeamHarTex/HarTex/compare/0.2.1...nightly)

> **3** contributor made contributions to this release

> **y** commits since 2023-07-16

## API Backend

- **Fixed:** returning in the uptime route

## Bors Buildbot

- **Changed:** migrated from eyre to miette for error reporting

## Buildsystem

- **Changed:** migrated from eyre to miette for error reporting

## Database Queries

## Discord Frontend

- **Added:** `/info server` and verbose filtering
- **Changed:** migrated from eyre to miette for error reporting
- **Changed:** allowed workers to send gateway commands to leaders
- **Changed:** updated cache entities and repositories
- **Changed:** replaced `/latency` and `/uptime` with `/info bot`

## Localization Infrastructure

- **Changed:** migrated from eyre to miette for error reporting
- **Changed:** updated localizations

## Rust Utilities

- **Changed:** removed hartex-eyre

## Web Frontend

# [Version 0.2.1 (2023-07-16)](https://github.com/TeamHarTex/HarTex/compare/0.2.0...0.2.1)

> **1** contributor made contributions to this release

> **16** commits since 2023-07-16

## Bors Buildbot

- **Fixed:** compiler errors

# [Version 0.2.0 (2023-07-16)](https://github.com/TeamHarTex/HarTex/compare/0.1.0...0.2.0)

> **5** contributors made contributions to this release

> **388** commits since 2023-06-14

## API Backend

- **Changed:** switched from ScyllaDB to PostgreSQL
- **Changed:** migrated from eyre to miette for error reporting
- **Changed:** updated rust version to 1.73.0

## Bors Buildbot

- implemented pull request queue
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

- added a dedicated crate for errors
- **Changed:** updated rust version to 1.73.0

## Web Frontend

- **Added:** documentation website
- **Added:** introduction page

# [Version 0.1.0 (2023-06-14)](https://github.com/TeamHarTex/HarTex/commits/0.1.0)

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
