# Changelog

All notable changes to AstroRAW-o-Matic will be documented here.

Format follows [Keep a Changelog](https://keepachangelog.com/en/1.0.0/).
Versioning follows [Semantic Versioning](https://semver.org/).

---

## [Unreleased]

---

## [0.3.0] — 2026-05-17

### Fixed
- **Critical**: Bayer pattern now correctly computed for cropped sensor area.
  Canon 600D crop at (152,56) yields GBRG, not RGGB as previously assumed.
  Photometric color calibration in Siril now matches CR2 direct import (< 3% deviation).
- PEDESTAL keyword added so Siril correctly subtracts black level before debayering.
- FITS string values no longer truncated (DATE-OBS, SWCREATE complete).
- APERTURE omitted when EXIF returns 0 (manual lenses).
- Crop of optical black sensor borders now applied correctly.

### Added
- FOCRATIO and APTDIA keywords in FITS header.
- WB coefficients in FITS header (CBLACK_R/G/B).
- Tauri + Svelte GUI scaffold with session form, file list, convert panel.
- Drag & drop from macOS Finder in GUI.
- JSON load/save via GUI toolbar with configurable filename pattern.
- Session date/time fields with EXIF pre-fill on file drop.
- Filter datalist: UV/IR, DNB, Halpha, OIII, SII, LRGB.

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
