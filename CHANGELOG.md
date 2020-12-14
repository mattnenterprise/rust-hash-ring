# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.2.0] - 2020-12-13
### Added
- CHANGELOG.md
- Support for switching out the hash function.

### Changed
- get_node function to take '&self' instead of '&mut self'.
- Moved from TravisCI and AppVeyor to Github Actions for CI.
- Moved from Coveralls to CodeCov for code coverage tracking.
- Hashing function changed to xxHash64 from md5.

### Fixed
- A bug that deleted the first node in the ring when trying to delete a node that didn't exist.

[Unreleased]: https://github.com/mattnenterprise/rust-hash-ring/compare/v0.2.0...HEAD
[0.2.0]: https://github.com/mattnenterprise/rust-hash-ring/compare/v0.1.7...v0.2.0
