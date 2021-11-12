# Setting Up HarTex

## How to set up HarTex

Before inviting HarTex to a guild, it has to be on the whitelist first. See [Getting Started](/HarTex-rust-discord-bot/docs/usage/getting-started)
for more details on that. If the bot is invited to a guild that is not whitelisted, it would immediately leave. However,
if your guild **IS** whitelisted but the bot still leaves your guild when invited, please notify us in the support guild.

Please also make sure to let the owner know, that the bot will message them with some instructions and a welcome message.

Provided below are some example configurations, more detailed documentation about the configuration API can be found in
the [Configuration API Documentation](/HarTex-rust-discord-bot/docs/usage/api-docs) section.

## Example HarTex Configurations

The following examples are barebones examples. It is advised that you go through the documentation to understand each
item and audit the functionality based on your guild's needs.

### Basic Configuration

```toml
[[DashboardAccess]]
userId = 000000000000000000  # your Discord user ID
accessLevel = 3              # the permission level (see the documentation of the DashboardAccess
                             # object for the meanings of individual values)

[GuildConfiguration]
nickname = "HarTex"     # the display nickname of the bot in the guild
timezone = "UTC"             # the timezone to use for timestamps
dmCannotUseCommand = true    # determines whether a DM is sent if a user cannot use a specific command
```
