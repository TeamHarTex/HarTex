# Nightly Features

This object configures the nightly (unstable) features of the bot. This object switches them on or off for a guild.

<div class="warning">
    <h5>
        <span class="span">
            <svg xmlns="http://www.w3.org/2000/svg" height="24px" viewBox="0 0 24 24" width="24px" fill="#000000"><path d="M1 21h22L12 2 1 21zm12-3h-2v-2h2v2zm0-4h-2v-4h2v4z"/></svg>
        </span>
        <span class="span2">
            WARNING
        </span>
    </h5>
    <p>These features can only be enabled when using the <strong>nightly</strong> version of the bot; hence this object will be ignored on <strong>stable</strong> versions of the bot. For more information, please see (link to be added).</p>
</div>

<div class="warning">
    <h5>
        <span class="span">
            <svg xmlns="http://www.w3.org/2000/svg" height="24px" viewBox="0 0 24 24" width="24px" fill="#000000"><path d="M1 21h22L12 2 1 21zm12-3h-2v-2h2v2zm0-4h-2v-4h2v4z"/></svg>
        </span>
        <span class="span2">
            WARNING
        </span>
    </h5>
    <p>Please be reminded that the features in this list can be very experimental and things are subject to change; alongside with the fact that these features are unstable and may break at any time. It is not recommended to enable these features.</p>
</div>

### Nightly Features Object

#### Nightly Features Structure

| FIELD        | TYPE     | DESCRIPTION                                                                                               |
|--------------|----------|-----------------------------------------------------------------------------------------------------------|
| threads      | boolean? | experimental support for Discord threads; `false` by default                                              |
| localization | boolean? | experimental support for localization facilities, for example timezones and languages; `false` by default |

#### Example Nightly Features Object

```toml
[NightlyFeatures]
threads = false
localization = false
```

#### Feature List

##### Threads

This feature adds experimental support for Discord threads, so that features of the bot can be used in threads.

This feature has not been implemented yet, but is here because it is of course, *unstable* and is subject to massive
changes for when it is implemented.

##### Localization

This feature adds experimental localization facilities, such as timezones for time representation and languages for
the messages and other things.

This feature is a work in progress, unstable and subject to change.
