# AstroRAW-o-Matic // User Guide

**Version 0.3.0** | [Deutsche Version](USER_GUIDE.de.md) | [Dark Matters Community](https://discord.gg/mvgC6aXY)

---

## Table of Contents

1. [What is AstroRAW-o-Matic?](#what-is-astroraw-o-matic)
2. [Installation](#installation)
3. [Recommended Workflow](#recommended-workflow)
4. [GUI Guide](#gui-guide)
5. [CLI Reference](#cli-reference)
6. [Session JSON Reference](#session-json-reference)
7. [FITS Header Reference](#fits-header-reference)
8. [FAQ](#faq)

---

## What is AstroRAW-o-Matic?

AstroRAW-o-Matic converts DSLR RAW files (Canon CR2) into FITS files for astrophotography. It is the clean step **before** stacking // not a replacement for PixInsight, Siril, or DeepSkyStacker, but the preparation for them.

**What it does:**
- Preserves RAW Bayer data without debayering
- Writes all relevant metadata into the FITS header
- Enables reproducible, documented conversions

**What it does not do:**
- No stacking
- No image processing
- No calibration (darks/flats)
- No debayering (left to your stacking tool)

---

## Installation

### Prerequisites

- Rust 1.75+ → [rustup.rs](https://rustup.rs)

### Build

```bash
git clone https://github.com/lindekai/AstroRAW-o-Matic.git
cd AstroRAW-o-Matic
cargo build --release
```

Binary: `target/release/astroraw-o-matic`

### Alias (recommended)

```bash
# In ~/.zshrc or ~/.bashrc
alias arom='astroraw-o-matic'
```

---

## Recommended Workflow

```
DSLR Frames (CR2)
        │
        ▼
 AstroRAW-o-Matic
 ┌─────────────────────────────┐
 │ Create session JSON         │
 │ Enter metadata              │
 │ Convert CR2 → FITS          │
 └─────────────────────────────┘
        │
        ▼
   FITS Files
   (16-bit, GBRG, full header)
        │
        ▼
   Siril / PixInsight / APP
   ┌─────────────────────────────┐
   │ Calibrate (Darks, Flats)   │
   │ Debayer                    │
   │ Stack                      │
   │ Process                    │
   └─────────────────────────────┘
```

### Notes for Siril

- Bayer pattern for Canon 600D/600Da: **GBRG** (set automatically)
- `PEDESTAL` keyword is set → Siril subtracts black level automatically
- Photometric color calibration shows < 3% deviation from CR2 direct import

---

## GUI Guide

### Loading Files

**Option 1 // Drag & Drop:** Drag CR2 files from Finder directly into the window.

**Option 2 // File Dialog:** Click **+ Add Files**.

Session Date and Time are automatically pre-filled from the EXIF of the first loaded file.

### Session Form

| Field | Description |
|-------|-------------|
| Object | Target object (M31, NGC 7000, Horsehead Nebula …) |
| Observer | Observer name |
| Session Date | Observation date (pre-filled from EXIF) |
| Session Time | Observation time UTC (pre-filled from EXIF) |
| Frame Type | Light / Dark / Flat / Bias |
| Filter | UV/IR, DNB, Halpha, OIII, SII, L, R, G, B |
| Camera Make/Model | Camera manufacturer and model |
| Pixel X/Y (µm) | Pixel size (Canon 600D/600Da: 4.3 µm) |
| Telescope | Telescope or lens |
| Focal Length (mm) | Focal length |
| Aperture (mm) | Aperture diameter |
| Focal Ratio | Calculated automatically (focal length ÷ aperture) |
| Site Name | Observation site name |
| Latitude / Longitude | Site coordinates |
| Elevation (m) | Site elevation above sea level |

### JSON Load and Save

- **📂 Load JSON** // load an existing session JSON
- **💾 Save JSON** // save current session settings

**Filename pattern** (field in OUTPUT section):

| Placeholder | Meaning |
|-------------|---------|
| `{object}` | Object name (spaces → underscores) |
| `{date}` | Current date (YYYY-MM-DD) |
| `{observer}` | Observer name |

Example: `session_{object}_{date}` → `session_Horsehead_Nebula_2026-05-17.json`

### Converting

1. Choose output folder: **Output Folder**
2. Click **Convert to FITS**
3. Results shown below (Succeeded / Failed)

---

## CLI Reference

### `inspect` // Show metadata

```bash
astroraw-o-matic inspect image.CR2
astroraw-o-matic inspect image.CR2 --json   # machine-readable
```

### `validate` // Check session JSON

```bash
astroraw-o-matic validate session.json
```

### `convert` // Convert

```bash
# Single file
astroraw-o-matic convert image.CR2 \
  --output ./FITS \
  --object "M31" \
  --type light

# Directory with session JSON
astroraw-o-matic convert ./RAW \
  --output ./FITS \
  --metadata session.json

# Dry run (show what would happen)
astroraw-o-matic convert ./RAW \
  --output ./FITS \
  --dry-run

# Recursive with overwrite
astroraw-o-matic convert ./RAW \
  --output ./FITS \
  --recursive \
  --overwrite
```

#### All options

| Option | Description |
|--------|-------------|
| `--output <DIR>` | Output directory (required) |
| `--metadata <FILE>` | Session JSON file |
| `--type <TYPE>` | light, dark, flat, bias |
| `--object <NAME>` | Object name |
| `--observer <NAME>` | Observer name |
| `--telescope <NAME>` | Telescope or lens |
| `--filter <NAME>` | Filter name |
| `--date-obs <DATETIME>` | Override date/time (ISO 8601) |
| `--header-mode <MODE>` | minimal or astro (default) |
| `--recursive` | Include subdirectories |
| `--overwrite` | Overwrite existing FITS files |
| `--dry-run` | Show only, write nothing |
| `--verbose` | Verbose output |

---

## Session JSON Reference

```json
{
  "schema_version": "1.0",
  "object": "Horsehead Nebula",
  "observer": "Kai Linde",
  "session_date": "2024-01-09",
  "session_time": "21:34:00",
  "frame_type": "light",
  "equipment": {
    "telescope": "Skywatcher 80ED",
    "focal_length": 600,
    "aperture": 80,
    "filter": "UV/IR",
    "camera": {
      "make": "Canon",
      "model": "EOS 600Da",
      "pixel_size_x": 4.3,
      "pixel_size_y": 4.3
    }
  },
  "location": {
    "latitude": 52.52,
    "longitude": 13.40,
    "elevation": 34.0,
    "site_name": "Backyard Observatory, Berlin"
  },
  "output": {
    "raw_mode": "raw_bayer",
    "header_mode": "astro",
    "overwrite": false,
    "write_history": true,
    "json_filename_pattern": "session_{object}_{date}"
  },
  "file_overrides": {
    "dark_001.CR2": { "frame_type": "dark" },
    "flat_001.CR2": { "frame_type": "flat", "filter": "L" }
  }
}
```

### Metadata Priority

1. `file_overrides[filename]`
2. Session JSON values
3. EXIF from RAW file
4. Built-in defaults

---

## FITS Header Reference

| Keyword | Content | Source |
|---------|---------|--------|
| SIMPLE | T | Standard |
| BITPIX | 16 | Standard |
| NAXIS1/2 | Image width/height | rawler |
| INSTRUME | Camera model | EXIF |
| EXPTIME | Exposure time [s] | EXIF |
| ISOSPEED | ISO setting | EXIF |
| DATE-OBS | Observation date/time | EXIF / Session |
| BAYERPAT | Bayer pattern (GBRG) | rawler |
| CFATYPE | CFA type | rawler |
| COLORTYP | RAW | fixed |
| XPIXSZ/YPIXSZ | Pixel size [µm] | Session |
| BITDEPTH | ADC bit depth (14) | rawler |
| IMAGETYP | LIGHT/DARK/FLAT/BIAS | Session / CLI |
| OBJECT | Target object name | Session / CLI |
| TELESCOP | Telescope/lens | Session |
| FOCALLEN | Focal length [mm] | EXIF / Session |
| APERTURE | Aperture [mm] | Session |
| APTDIA | Aperture [mm] | Session |
| FOCRATIO | Focal ratio | calculated |
| FILTER | Filter name | Session |
| OBSERVER | Observer name | Session |
| SITELAT/LONG | Site coordinates | Session |
| SITEELEV | Site elevation [m] | Session |
| BLACKLVL | Black level [ADU] | rawler |
| PEDESTAL | Black level [ADU] | rawler |
| WHITELEV | Saturation level [ADU] | rawler |
| CBLACK_R/G/B | WB coefficients | rawler |
| SWCREATE | AstroRAW-o-Matic v0.3.0 | fixed |
| HISTORY | Conversion provenance | automatic |

---

## FAQ

**Why does the FITS look different from the CR2 in preview?**
The FITS contains raw linear sensor data without white balance or black level correction. After debayering and photometric color calibration in Siril, results are essentially identical (< 3% deviation).

**What Bayer pattern does the Canon 600D/600Da use?**
GBRG // set automatically and correctly. Do not change manually.

**What if the camera clock was wrong?**
Use `--date-obs "2024-01-09T21:34:00"` in the CLI, or `date_obs` in the session JSON, or the `session_date` + `session_time` fields in the GUI.

**Can I have multiple objects in one batch conversion?**
Yes // use `file_overrides` in the session JSON to override `object`, `frame_type`, `filter`, and `date_obs` for individual files.

---

*"The answer is 42. The Bayer pattern is GBRG."*
