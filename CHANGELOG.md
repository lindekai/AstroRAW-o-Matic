# Changelog

All notable changes to AstroRAW-o-Matic will be documented here.

Format follows [Keep a Changelog](https://keepachangelog.com/en/1.0.0/).
Versioning follows [Semantic Versioning](https://semver.org/).

---

## [Unreleased]

### Added
- `rawler` integration: real Canon CR2 Bayer pixel extraction (14-bit, RGGB)
- Black level and white level from rawler written to FITS header (BLACKLVL, WHITELEV)
- Bayer pattern now read directly from rawler CFA data, not just inferred
- Conversion pipeline: pixel data read before metadata resolve so all rawler values flow into FITS header

### Changed
- `read_raw_bayer()` no longer returns zero-filled placeholder; now returns real pixel data
- Metadata resolution moved after pixel read so black/white levels are available

---

## [0.1.0] — 2026-05-15

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
- Only Canon CR2 tested; CR3/NEF/ARW pending
