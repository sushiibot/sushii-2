# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).


## [0.1.5] - 2021-06-27

### Fixed

-  Add request timeout

## [0.1.4] - 2021-04-18

### Changed

-  `sushii-feeds` now runs entirely separate without gRPC connection, sends
   Discord feed messages directly via http-proxy

## [0.1.3] - 2021-03-14

### Fixed

-  Fix vlive channel fetching

### Changed

-  Updated to tokio 1.0
-  Better error logging

## [0.1.2] - 2021-02-02

### Added

-  Use new live thumbnail when static is unavailable

### Fixed

-  Use correct Live / Vod labels

### Changed

-  Use default VLive colour for embeds

## [0.1.1] - 2021-01-30

### Added

-  VLive author icon, LIVE / VOD in title, duration, and colour

## [0.1.0] - 2021-01-27

### Added

-  Initial release, VLive feeds and basic caching

[unreleased]: https://github.com/sushiibot/sushii-2/compare/sushii-feeds-v0.3.0...HEAD
[0.1.3]: https://github.com/sushiibot/sushii-2/compare/sushii-feeds-v0.1.2...sushii-feeds-v0.1.3
[0.1.2]: https://github.com/sushiibot/sushii-2/compare/sushii-feeds-v0.1.1...sushii-feeds-v0.1.2
[0.1.1]: https://github.com/sushiibot/sushii-2/compare/sushii-feeds-v0.1.0...sushii-feeds-v0.1.1
