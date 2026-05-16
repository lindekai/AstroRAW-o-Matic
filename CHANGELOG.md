# Changelog

All notable changes to AstroRAW-o-Matic will be documented here.

Format follows [Keep a Changelog](https://keepachangelog.com/en/1.0.0/).
Versioning follows [Semantic Versioning](https://semver.org/).

---

## [Unreleased]

---

## [0.2.0] — 2026-05-16

### Added
- `rawler` integration: real Canon CR2 Bayer pixel extraction (14-bit, RGGB)
- Black/white levels from rawler written to FITS header (BLACKLVL, WHITELEV)
- As-shot white balance coefficients in FITS header (CBLACK_R, CBLACK_G, CBLACK_B)
- Automatic crop of optical black sensor borders (crop_area)
- `date_obs` override via `--date-obs` CLI, session JSON and `file_overrides`
- Bayer pattern sourced directly from rawler CFA data

### Fixed
- FITS string values no longer truncated (DATE-OBS, SWCREATE now complete)
- APERTURE omitted when value is 0 (manual lenses / missing EXIF)
- Stride mismatch between FITS header and pixel data (caused stripes) resolved

### Changed
- `read_raw_bayer()` returns real pixel data instead of zero-filled placeholder
- Metadata resolution happens after pixel read so rawler values flow into header
- FOCALLEN and APERTURE from session JSON override EXIF

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
