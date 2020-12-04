# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [unreleased]

### Added

-  Add on/off guild setting aliases "enable" and "disable"
-  Fishy and rep commands
-  Tag commands

### Changed

-  Use updated rank template context variables

### Fixed

-  Actually send roles.txt file with `roles listids` when length is over 2000 chars

## [0.1.9]

### Fixed

-  Parse missing reason when using `mute` command without a duration
-  Check permissions per command instead of only per group

## [0.1.8]

### Added

-  Join reactions
-  Allow setting mute defaultduration to indefinite
-  Sentry error logging
-  Update latest n mod log cases with `reason latest~n [reason]`

### Changed

-  Preserve group and role order to match role configuration
-  Respond with mute subcommands if no args are given
-  Replace TOML role configuration with YAML
-  Display `settings list` nicer in an embed

### Fixed

-  Validate join react setting

## [0.1.7]

### Added

-   `mute setduration` and `mute addduration` commands to update existing mute durations
-   Short prune alias `p` for faster prunes
-   Guild and guild member count metric gauges

### Changed

-   Use case insensitive role name search for `settings mute role` command
-   Help commands now link to website commands list
-   Move mute duration next to action name in mod logs
-   Update serenity to `0.9.1`

### Fixed

-   Respond with an error if attempting to mute already muted members
-   Prevent mutes with only duration provided to consider reason as empty string, causing Discord embeds to fail
-   Mute entries properly save their false pending state so that re-joining will re-assign mute roles
-   Remove serenity's default prefix `~`

## [0.1.6]

### Added

-   Reference to mute mod log case to determine mute executor on unmute
-   Pending status to mutes to allow for mute duration to be set per command
-   Allow duration to be set per mute by the `mute` command
-   Display mute duration in mod logs
-   `ModLogReporter` to send messages to mod logs easier
-   Settings subcommands to `set`, `off`, `on`, `toggle`, `show` the given guild setting

### Changed

-   `reason` responds with number of cases modified and reason
-   Enable guild settings by default
-   Rename `stats` command to `about`

### Fixed

-   `warn` command sends message in mod log

## [0.1.5] - 2020-10-16

### Added

-   Traefik, PostgreSQL backup, cAdvisor, Node Exporter docker-compose services
-   `listmutes` command
-   Mod action reason in response message
-   Info when roles group is run without sub command
-   Respond with error if no valid IDs are given to mod action executor

### Changed

-   Listen on sub paths instead of subdomains for Prometheus and Grafana services
-   Start interval tasks only once

### Fixed

-   Parse mod action executor IDs with length 17-19 instead of just 18-19
-   Handle unmutes when members are no longer in the guild

## [0.1.4] - 2020-10-01

### Fixed

-   Remove target-cpu codegen option

## [0.1.3] - 2020-10-01

### Added

-   Configurable metrics interface
-   Add colors to mod actions
-   Embed DB migrations
-   Add toggles for guild config settings
-   Remute users if leaving with a mute role
-   Auto unmute users after set time
-   Save moderator tag and id to audit log
-   Add case history summary

### Changed

-   Move Docker sushii binary to /usr/local/bin
-   Dedupe mod action ids
-   Handle shutdown signals cleanly

### Fixed

-   Add libssl-dev in Docker image
-   Fix correct NaiveDateTime format string
-   Prevent saving placeholder modlog reason
-   Prevent role handler from running outside of role channels
-   Fix mod log entry saving

[unreleased]: https://github.com/sushiibot/sushii-2/compare/v0.1.8...HEAD
[0.1.8]: https://github.com/sushiibot/sushii-2/compare/v0.1.7...v0.1.8
[0.1.7]: https://github.com/sushiibot/sushii-2/compare/v0.1.6...v0.1.7
[0.1.6]: https://github.com/sushiibot/sushii-2/compare/v0.1.5...v0.1.6
[0.1.5]: https://github.com/sushiibot/sushii-2/compare/v0.1.4...v0.1.5
[0.1.4]: https://github.com/sushiibot/sushii-2/compare/v0.1.3...v0.1.4
[0.1.3]: https://github.com/sushiibot/sushii-2/compare/v0.1.2...v0.1.3
[c:27120af]: https://github.com/sushiibot/sushii-2/commit/27120af575aed5f7a437152d8b4d16b3fcc7e7c1
