# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Changed

-   Listen on sub paths instead of subdomains for Prometheus and Grafana services
-   Start interval tasks only once

### Added

-   Traefik, PostgreSQL backup, cAdvisor, Node Exporter docker-compose services
-   `listmutes` command
-   Mod action reason in response message
-   Info when roles group is run without sub command

## [0.1.4] - 2020-10-01

### Changed

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
-   Add libssl-dev in Docker image
-   Fix correct NaiveDateTime format string
-   Prevent role handler from running outside of role channels
-   Fix mod log entry saving
-   Prevent saving placeholder modlog reason
-   Dedupe mod action ids
-   Handle shutdown signals cleanly

[unreleased]: https://github.com/drklee3/sushii-2/compare/v0.1.4...HEAD
[0.1.4]: https://github.com/drklee3/sushii-2/compare/v0.1.3...v0.1.4
[0.1.3]: https://github.com/drklee3/sushii-2/compare/v0.1.2...v0.1.3
[c:27120af]: https://github.com/drklee3/sushii-2/commit/27120af575aed5f7a437152d8b4d16b3fcc7e7c1
