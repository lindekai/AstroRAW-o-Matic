# Changelog

All notable changes to AstroRAW-o-Matic will be documented here.

Format follows [Keep a Changelog](https://keepachangelog.com/en/1.0.0/).
Versioning follows [Semantic Versioning](https://semver.org/).

---

## [Unreleased]

### Added
- Initial project structure (Cargo workspace)
- `astroraw-models`: shared data models — `SessionMetadata`, `FitsHeader`, `FrameType`, `RawMetadata`
- `astroraw-core`: engine — RAW EXIF extraction, metadata resolution, FITS 16-bit writer
- `astroraw-cli`: CLI — `convert`, `inspect`, `validate` subcommands
- Session JSON schema v1.0
- Metadata priority chain: file override > session JSON > RAW EXIF > defaults
- Bayer pattern inference for Canon cameras
- Astro-mode FITS header with full set of standard and astro-specific keywords
- Per-file metadata overrides via `file_overrides` in session JSON
- Dry-run mode for `convert`
- Batch directory conversion with optional recursion

### Known limitations
- RAW pixel data extraction is a stub (LibRaw FFI not yet integrated)
- Only Canon CR2 EXIF reading is implemented; CR3/NEF/ARW pending
