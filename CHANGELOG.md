# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).


## [0.4.0] - 2022-11-08

### Added
- reset world button
- GeoNavigation component added (as action item)
- editor page (right now it assigns location to scenes - using OSM)
- triggering event via geolocation
- allow characters without scene (move to initial location)
- setting world name in intro
- self-trigger events - events which can be triggered by user right away
- added refresh button for speech syntesis
- backlink to upper path `/doggie_and_kitie/cake/` -> `/doggie_and_kitie/`
- display title when scanning QR code

### Changed
- pabitell-root moved to a separate repo
- refactored event translations
- refactored event conditions
- refactored event updates
- better file name (contains story code now)
- pritable qr-codes should be in A7 format and have same size
- scene contains geolocation field

### Fixed
- speech synthesis should work for google chrome
- skim update to deal with security warnings


## [0.3.0] - 2022-08-30

### Changed
- stories moved to a separate repository
- pabitell-cli renamed to pabitell-webserver
- cli stuff moved from pabitell-cli to pabitell-lib

### Fixed
- category in Cargo.toml

## [0.2.0] - 2022-08-15

### Added
- uploading and downloading world from/into file
- pabitell-root - add QR link to chapters

### Changed
- clippy-based refactoring
- dependency updates and remove unused dependencies

### Fixed
- stories/doggie_and_kitie/cake - meal can be consumed only once


## [0.1.0] - 2022-07-10

### Added
- Shared library
- CLI app (WS server and some basic story testing)
- Root webapp added
- Doggie and Kitie - Doll webapp added
- Doggie and Kitie - Cake webapp added
