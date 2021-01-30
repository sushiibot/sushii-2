# sushii2 migration

This covers data migration from sushii-bot (sushiiDev) to sushii2.

## Data 

Most of the sushiiDev data will be migrated over.

This includes:

- User ranks, rep, fishies
- Server configurations
- Server tags

## Features

Since the entire bot has been rewritten, there have been some features that have
been changed or removed. This is a brief list of the more significant changes
in sushii2. As always, all commands and detailed information is listed on the
also new website @ https://2.sushii.xyz

### Added Features

- Timed mutes
- `reason latest~n` to set the last n mod log case reasons

### Changed Features

- Reminders send in channels where the reminder was set instead of DM.
- Notifications are server specific, no more global notifications for
  performance reasons. Please set notifications in each guild separately for
  now.

### Removed Features

- Sushiiboard / Starboard
- Gallery
- urban, crypto, weather commands

## Migration

### If using sushii2

If you are using the sushii2 preview bot, sushii2 will no longer respond to
commands and will be running on @sushiiDev. Please make sure @sushiiDev has the
same Discord permissions and access as sushii2.

If any are set, sushii2 server tags will be merged with the old tags. If there
is a tag name conflict, the new tag will have a `2` appended to the end.

For example, if a tag `meow` exists on sushiiDev and there is also a `meow` on
sushii2, `meow` will be the old tag and `meow2` will be the newer sushii2 tag.

### If not using sushii2

If you are not using sushii2, you do not have to do anything extra.

If you have any questions, feel free to use #sushii-help.  Found a bug or
something to suggest? Please submit any problems to #sushii-suggests-and-fixes.

