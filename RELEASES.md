# Version 0.3.0 (2023-08-23)

> **x** contributor made contributions to this release

> **y** commits since 2023-07-16

## API Backend

- fixed returning in the uptime route

## Bors Buildbot

- migrated from eyre to miette for error reporting

## Buildsystem

- migrated from eyre to miette for error reporting

## Database Queries

- no major changes

## Discord Frontend

- migrated from eyre to miette for error reporting
- implemented `/serverinfo`
- allowed workers to send gateway commands to leaders
- updated cache entities and repositories

## Localization Infrastructure

- migrated from eyre to miette for error reporting

## Rust Utilities

- removed hartex-eyre

## Web Frontend

- no major changes

# Version 0.2.1 (2023-07-16)

> **1** contributor made contributions to this release

> **16** commits since 2023-07-16

## API Backend

- no major changes

## Bors Buildbot

- fixed compiler errors

## Buildsystem

- no major changes

## Database Queries

- no major changes

## Discord Frontend

- no major changes

## Localization Infrastructure

- no major changes

## Rust Utilities

- no major changes

## Web Frontend

- no major changes

# Version 0.2.0 (2023-07-16)

> **5** contributors made contributions to this release

> **388** commits since 2023-06-14

## API Backend

- switched from ScyllaDB to PostgreSQL
- migrated from eyre to miette for error reporting
- updated rust version to 1.73.0

## Bors Buildbot

- implemented pull request queue
- updated enqueued pull request model
- updated command parsing to support passing parameters to commands
- modified try command to optionally accept a parent commit for the try build creation
- migrated from eyre to miette for error reporting
- updated rust version to 1.73.0

## Buildsystem

- added support for passing multiple projects to individual commands
- changed the algorithm to not bail out when a project is not found; skipping it and continue instead
- migrated from eyre to miette for error reporting
- updated rust version to 1.73.0

## Database Queries

- initial release
- added basic queries for usage in backend and Discord frontend

## Discord Frontend

- updated configuration API
- updated configuration version computation algorithm
- implemented corresponding changes to uptime updates with the API
- switched from ScyllaDB to PostgreSQL
- added `/serverinfo` command
- updated rust version to 1.73.0

## Localization Infrastructure

- updated rust version to 1.73.0

## Rust Utilities

- added a dedicated crate for errors
- updated rust version to 1.73.0

## Web Frontend

- added documentation website
- added introduction page

# Version 0.1.0 (2023-06-14)

> **5** contributors made contributions to this release

> **2983** commits since 2022-08-31

## API Backend

- initial release
- `/bors` and `/uptime` APIs

## Bors Buildbot

- initial release
- implemented commands: `try`, `try-`, `r+`, `r=`
- pull request label modification on various events

## Buildsystem

- initial release
- project types: `rust`, `jsts`
- supports building and linting projects

## Discord Frontend

- initial release
- major infrastructure and codebase structure revamp
- commands implemented: `/about`, `/contributors`, `latency`, `/uptime`
- database caching
- drafting configuration API

## Localization Infrastructure

- initial release
- uses Project Fluent as foundation

## Rust Utilities

- initial release

## Web Frontend

- initial release
- basic landing page
