# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

<!-- next-header -->
## [Unreleased] - ReleaseDate
## [0.5.0] - 2020-10-20
### Changed
- Updated the builtin target list to Rust 1.47.0

## [0.4.1] - 2020-06-04
### Fixed
- Removed `dbg!` prints accidentally left in.

## [0.4.0] - 2020-06-04
### Added
- [PR#9](https://github.com/EmbarkStudios/cfg-expr/pull/9) added the optional `targets` feature, which allows matching the various `target_` predicates against a [`target_lexicon::Triple`](https://docs.rs/target-lexicon/0.11.0/target_lexicon/struct.Triple.html).

### Changed
- [PR#9](https://github.com/EmbarkStudios/cfg-expr/pull/9) changed the `Arch`, `Vendor`, `Os`, and `Env` types to no be longer enums, and are instead thin wrappers around strings. This allows for custom targets where one or more components of the target triple are not built-in to rustc. Resolved [#8](https://github.com/EmbarkStudios/cfg-expr/issues/8).
- Changed `ParseError` to remove the lifetime and just keep an owned string of the expression that failed to parse.
- Updated the list of built-in rustc targets to 1.43.1.

## [0.3.0] - 2020-04-05
### Changed
- [PR#7](https://github.com/EmbarkStudios/cfg-expr/pull/7) changed `Expression::eval` to take a `Logic` trait, to enable evaluation of 'unknown' predicates. Thanks [@sunshowers](https://github.com/sunshowers)!

## [0.2.1] - 2020-03-30
### Fixed
- [PR#6](https://github.com/EmbarkStudios/cfg-expr/pull/6) fixed nested predicate evalution. Thanks [@sunshowers](https://github.com/sunshowers)!

## [0.2.0] - 2020-02-05
### Added
- Added `targets::rustc_version` which can be used to retrieve the version string of the the rustc used to generate the list of targets.

### Changed
- `targets::ALL` now uses the built-in targets for rustc 1.41.0

## [0.1.0] - 2020-01-09
### Added
- Initial add of all the things

<!-- next-url -->
[Unreleased]: https://github.com/EmbarkStudios/cfg-expr/compare/0.5.0...HEAD
[0.5.0]: https://github.com/EmbarkStudios/cfg-expr/compare/0.4.1...0.5.0
[0.4.1]: https://github.com/EmbarkStudios/cfg-expr/compare/0.4.0...0.4.1
[0.4.0]: https://github.com/EmbarkStudios/cfg-expr/compare/0.3.0...0.4.0
[0.3.0]: https://github.com/EmbarkStudios/cfg-expr/compare/0.2.1...0.3.0
[0.2.1]: https://github.com/EmbarkStudios/cfg-expr/compare/0.2.0...0.2.1
[0.2.0]: https://github.com/EmbarkStudios/cfg-expr/compare/0.1.0...0.2.0
[0.1.0]: https://github.com/EmbarkStudios/cfg-expr/releases/tag/0.1.0
