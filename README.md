# sushii-2

Rewrite of [sushii-bot](https://github.com/drklee3/sushii-bot) with
[~~twilight~~](https://github.com/twilight-rs/twilight) async
[serenity-rs](https://github.com/serenity-rs/serenity/).

wip... again.

## Features

Slimmed down feature set of original sushii-bot with a focus on moderation tools

-   [ ] moderation tools
    -   [x] ban / unban
    -   [x] kick
    -   [x] prune
    -   [x] mute / unmute
        -   [ ] timed mutes
        -   [ ] mute users who joined in last x minutes
    -   [x] cases
        -   [x] reason
        -   [x] history
    -   [x] roles
        -   [x] json support
        -   [x] toml support
    -   [ ] settings
        -   [ ] dm users on action with reason
            -   [ ] toggle
            -   [ ] set message
        -   [ ] default mute duration (0 to disable)
        -   [ ] default num messages delete messages on ban
-   user
    -   avatar
    -   userinfo
    -   notifications
