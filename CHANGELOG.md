<!-- markdownlint-disable blanks-around-headings blanks-around-lists no-duplicate-heading -->

# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

<!-- next-header -->
## [Unreleased] - ReleaseDate
## [0.15.5] - 2023-09-08
### Changed
- [PR#64](https://github.com/EmbarkStudios/cfg-expr/pull/64) updated the builtin target list to 1.72.0. It also changed the MSRV to 1.70.0.

## [0.15.4] - 2023-07-28
### Changed
- [PR#62](https://github.com/EmbarkStudios/cfg-expr/pull/62) updated the builtin target list to 1.71.0.

## [0.15.3] - 2023-06-19
### Fixed
- [PR#61](https://github.com/EmbarkStudios/cfg-expr/pull/61) fixed an issue where `target_os = "none"` was not matching target triplets where `os = None`. Thanks [@sunshowers](https://github.com/sunshowers)!

## [0.15.2] - 2023-06-02
### Changed
- [PR#59](https://github.com/EmbarkStudios/cfg-expr/pull/60) updated the builtin target list to 1.70.0.

## [0.15.1] - 2023-04-20
### Changed
- [PR#59](https://github.com/EmbarkStudios/cfg-expr/pull/59) updated the builtin target list to 1.69.0.

## [0.15.0] - 2023-04-04
### Changed
- [PR#58](https://github.com/EmbarkStudios/cfg-expr/pull/58) updated the builtin target list to 1.68.2.

## [0.14.0] - 2023-01-27
### Changed
- [PR#57](https://github.com/EmbarkStudios/cfg-expr/pull/57) updated the builtin target list to 1.67.0.

## [0.13.0] - 2022-12-19
### Changed
- [PR#56](https://github.com/EmbarkStudios/cfg-expr/pull/56) updated the builtin target list to 1.66.0. Thanks [@sunshowers](https://github.com/sunshowers)!

## [0.12.0] - 2022-11-07
### Changed
- [PR#53](https://github.com/EmbarkStudios/cfg-expr/pull/53) updated the builtin target list to 1.65.0. Thanks [@sunshowers](https://github.com/sunshowers)!

### Added
- [PR#54](https://github.com/EmbarkStudios/cfg-expr/pull/54) added support for `abi`, which is currently nightly only, but should have no affect on stable. Thanks [@carols10cents](https://github.com/carols10cents)!

## [0.11.0] - 2022-09-27
### Changed
- [PR#51](https://github.com/EmbarkStudios/cfg-expr/pull/51) updated the builtin target list to 1.64.0. Thanks [@sunshowers](https://github.com/sunshowers)!
- [PR#51](https://github.com/EmbarkStudios/cfg-expr/pull/51) bumped the MSRV to 1.58.0.

## [0.10.3] - 2022-05-19
### Added
- [PR#49](https://github.com/EmbarkStudios/cfg-expr/pull/49) added support for the `has_target_atomic = "<ptr | integer>"` and `panic = "<strategy>"` predicates. Thanks [@sunshowers](https://github.com/sunshowers)!

## [0.10.2] - 2022-02-25
### Changed
- [PR#48](https://github.com/EmbarkStudios/cfg-expr/pull/48) updated the builtin target list to 1.59.0.

## [0.10.1] - 2022-02-07
### Fixed
- [PR#46](https://github.com/EmbarkStudios/cfg-expr/pull/46) fixed comparison of dynamic target families.

## [0.10.0] - 2022-02-04
### Changed
- [PR#44](https://github.com/EmbarkStudios/cfg-expr/pull/44) added support for multiple target families that are available from Rust 1.58.0+. Thanks [@sunshowers](https://github.com/sunshowers)!

## [0.9.1] - 2022-02-01
### Changed
- [PR#42](https://github.com/EmbarkStudios/cfg-expr/pull/42) updated the builtin target list to 1.58.0. Thanks [@sunshowers](https://github.com/sunshowers)!

## [0.9.0] - 2021-08-31
### Changed
- [PR#35](https://github.com/EmbarkStudios/cfg-expr/pull/35) changed `TargetInfo`, `Os`, `Arch`, `Env`, and `Vendor` to use a `Cow<'static, str>` to avoid the need for lifetime parameters for the common case of statically known target information, but still support arbitrary/future variants. Thanks [@sunshowers](https://github.com/sunshowers)!
- [PR#38](https://github.com/EmbarkStudios/cfg-expr/pull/38) updated the built-in target list to `1.54.0`, which notably includes the addition of the new `wasm` variant to `target_family`. Thanks [@sunshowers](https://github.com/sunshowers)!

### Fixed
- [PR#33](https://github.com/EmbarkStudios/cfg-expr/pull/33) added clippy.toml with an `msrv` so clippy lints are consistent across environments. Thanks [@remilauzier](https://github.com/remilauzier)!

## [0.8.1] - 2021-08-05
### Changed
- [PR#31](https://github.com/EmbarkStudios/cfg-expr/pull/31) reverted the usage of "or patterns" that were only added in 1.53.0. We now state the MSRV as 1.52.0. Thanks [@cgwalters](https://github.com/cgwalters)!

## [0.8.0] - 2021-07-16
### Changed
- [PR#28](https://github.com/EmbarkStudios/cfg-expr/pull/28) updated target-lexicon to 0.12. Thanks [@remilauzier](https://github.com/remilauzier)!
- [PR#29](https://github.com/EmbarkStudios/cfg-expr/pull/29) updated the built-in target list to 1.53.0.

## [0.7.4] - 2021-03-16
### Added
- [PR#26](https://github.com/EmbarkStudios/cfg-expr/pull/26) added `Expression::original` to get the original string the expression was parsed from. Thanks [@gdesmott](https://github.com/gdesmott)!

## [0.7.3] - 2021-03-16
### Added
- [PR#25](https://github.com/EmbarkStudios/cfg-expr/pull/25) added `Clone` for `Expression`. Thanks [@gdesmott](https://github.com/gdesmott)!

## [0.7.2] - 2021-03-16
### Added
- [PR#23](https://github.com/EmbarkStudios/cfg-expr/pull/23) added a `PartialEq` implementation for `Expression`, primarily for cases where an `Expression` is stored in a type that itself requires `PartialEq`. This is only a simple syntactical equality check. Thanks [@gdesmott](https://github.com/gdesmott)!

## [0.7.1] - 2021-02-17
### Fixed
- Fixed support for the `uclibceabi` environment added for one target in rust 1.50.0.

## [0.7.0] - 2021-02-12
### Changed
- Updated the builtin target list to Rust 1.50.0. Again, somewhat of a breaking change as many targets were removed or changed.

### Fixed
- Update smallvec to fix an [advisory](https://rustsec.org/advisories/RUSTSEC-2021-0003)

## [0.6.0] - 2021-01-04
### Changed
- Updated the builtin target list to Rust 1.49.0, this is somewhat of a breaking change, as rustc now considers all `android` targets to have the `gnu` environment, where previously, it was unspecified.

## [0.5.1] - 2020-12-15
### Changed
- Updated the builtin target list to Rust 1.48.0

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
- [PR#9](https://github.com/EmbarkStudios/cfg-expr/pull/9) changed the `Arch`, `Vendor`, `Os`, and `Env` types to not be longer enums, and are instead thin wrappers around strings. This allows for custom targets where one or more components of the target triple are not built-in to rustc. Resolved [#8](https://github.com/EmbarkStudios/cfg-expr/issues/8).
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
- Added `targets::rustc_version` which can be used to retrieve the version string of the rustc used to generate the list of targets.

### Changed
- `targets::ALL` now uses the built-in targets for rustc 1.41.0

## [0.1.0] - 2020-01-09
### Added
- Initial add of all the things

<!-- next-url -->
[Unreleased]: https://github.com/EmbarkStudios/cfg-expr/compare/0.15.5...HEAD
[0.15.5]: https://github.com/EmbarkStudios/cfg-expr/compare/0.15.4...0.15.5
[0.15.4]: https://github.com/EmbarkStudios/cfg-expr/compare/0.15.3...0.15.4
[0.15.3]: https://github.com/EmbarkStudios/cfg-expr/compare/0.15.2...0.15.3
[0.15.2]: https://github.com/EmbarkStudios/cfg-expr/compare/0.15.1...0.15.2
[0.15.1]: https://github.com/EmbarkStudios/cfg-expr/compare/0.15.0...0.15.1
[0.15.0]: https://github.com/EmbarkStudios/cfg-expr/compare/0.14.0...0.15.0
[0.14.0]: https://github.com/EmbarkStudios/cfg-expr/compare/0.13.0...0.14.0
[0.13.0]: https://github.com/EmbarkStudios/cfg-expr/compare/0.12.0...0.13.0
[0.12.0]: https://github.com/EmbarkStudios/cfg-expr/compare/0.11.0...0.12.0
[0.11.0]: https://github.com/EmbarkStudios/cfg-expr/compare/0.10.3...0.11.0
[0.10.3]: https://github.com/EmbarkStudios/cfg-expr/compare/0.10.2...0.10.3
[0.10.2]: https://github.com/EmbarkStudios/cfg-expr/compare/0.10.1...0.10.2
[0.10.1]: https://github.com/EmbarkStudios/cfg-expr/compare/0.10.0...0.10.1
[0.10.0]: https://github.com/EmbarkStudios/cfg-expr/compare/0.9.1...0.10.0
[0.9.1]: https://github.com/EmbarkStudios/cfg-expr/compare/0.9.0...0.9.1
[0.9.0]: https://github.com/EmbarkStudios/cfg-expr/compare/0.8.1...0.9.0
[0.8.1]: https://github.com/EmbarkStudios/cfg-expr/compare/0.8.0...0.8.1
[0.8.0]: https://github.com/EmbarkStudios/cfg-expr/compare/0.7.4...0.8.0
[0.7.4]: https://github.com/EmbarkStudios/cfg-expr/compare/0.7.3...0.7.4
[0.7.3]: https://github.com/EmbarkStudios/cfg-expr/compare/0.7.2...0.7.3
[0.7.2]: https://github.com/EmbarkStudios/cfg-expr/compare/0.7.1...0.7.2
[0.7.1]: https://github.com/EmbarkStudios/cfg-expr/compare/0.7.0...0.7.1
[0.7.0]: https://github.com/EmbarkStudios/cfg-expr/compare/0.6.0...0.7.0
[0.6.0]: https://github.com/EmbarkStudios/cfg-expr/compare/0.5.1...0.6.0
[0.5.1]: https://github.com/EmbarkStudios/cfg-expr/compare/0.5.0...0.5.1
[0.5.0]: https://github.com/EmbarkStudios/cfg-expr/compare/0.4.1...0.5.0
[0.4.1]: https://github.com/EmbarkStudios/cfg-expr/compare/0.4.0...0.4.1
[0.4.0]: https://github.com/EmbarkStudios/cfg-expr/compare/0.3.0...0.4.0
[0.3.0]: https://github.com/EmbarkStudios/cfg-expr/compare/0.2.1...0.3.0
[0.2.1]: https://github.com/EmbarkStudios/cfg-expr/compare/0.2.0...0.2.1
[0.2.0]: https://github.com/EmbarkStudios/cfg-expr/compare/0.1.0...0.2.0
[0.1.0]: https://github.com/EmbarkStudios/cfg-expr/releases/tag/0.1.0
