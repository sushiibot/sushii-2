**sushii2 migration**

This covers data migration from sushii-bot (sushiiDev) to the new rewritten sushii2. This has been an (almost daily) work in progress for the past 4 months to rebuild sushii.

**DATA**

> Most of the sushiiDev data will be migrated over.
> 
> This includes:
> 
> - [x] Server configurations
> - [x] Server tags
> - [x] User ranks, rep, fishies
> - [x] User reminders
> 
> If you have reps and/or fishies on sushii2, they have been added onto your existing reps and fishies. :smile:

---

**FEATURES**

Since the entire bot has been rewritten, there have been some features that have been changed or removed. This is a brief list of the more significant changes in sushii2. As always, all commands and detailed information is listed on the also new website @ <https://2.sushii.xyz>

**Added Features**

- Timed mutes
- `reason latest~n` to set the last n mod log case reasons
- Refer to website <https://2.sushii.xyz> for more info

**Changed Features**

- Notifications are server specific, no more global notifications for performance reasons. Please set notifications in each guild separately for now.
- `vlivenotif` is now under the `feed` commands: `feed list` and `feed add`

**Removed Features**

A few features have been removed as they are either bloat or commonly available
in other bots to reduce the amount of future code maintenance required. It is
possible they will be reimplemented in the future.

- Sushiiboard / Starboard
- Gallery
- urban, crypto, weather commands

## Migration

**If you are using sushii2**

If you are using the sushii2 preview bot, sushii2 will no longer respond to commands and will be running on @sushiiDev. Please make sure @sushiiDev has the same Discord permissions and access as sushii2.

If any are set, sushii2 server tags will be merged with the old tags. If there is a tag name conflict, the old tag will have a `old` appended to the end.

For example, if a tag `meow` exists on sushiiDev and there is also a `meow` on sushii2, `meowold` will be the old tag and `meow` will be the newer sushii2 tag. If there is a `meowold` already, it will be renamed to `meowold2` and so on.

After you've ensured @sushiiDev has the correct permissions and is working properly, you may kick @sushii2 as it will no longer be used.

**If you are not using sushii2**

You do not have to do anything extra, just check the website for changed command names.

---

If you have any questions, feel free to use #sushii-help.
Found a bug or have something to suggest? Please submit any problems to #sushii-suggests-and-fixes.

