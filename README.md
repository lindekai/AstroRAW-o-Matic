# AstroRAW-o-Matic

**Mostly harmless RAW conversion.**

🇩🇪 [Deutsche Version / German version](README.de.md)

> **⚠️ Early Alpha // not ready for production use.**
> The core architecture and CLI are in place, but RAW pixel extraction is not yet fully implemented. Expect breaking changes. Feedback and contributions welcome.

> *"Space is big. You just won't believe how vastly, hugely, mind-bogglingly big it is. And it turns out, so are CR2 files."*

AstroRAW-o-Matic is a reproducible, workflow-oriented RAW-to-FITS converter for astrophotographers. It is the clean first step before stacking and image processing // not a competitor to PixInsight, Siril, or DeepSkyStacker, but the tool you run *before* those.

It takes your DSLR RAW files and turns them into properly-headered, reproducible FITS files, ready for your stacking software of choice.

---

## Community

**Dark Matters // Astro Community**
Questions, feedback and discussions about AstroRAW-o-Matic:

> 🔗 [discord.gg/mvgC6aXY](https://discord.gg/mvgC6aXY)

---

## Why AstroRAW-o-Matic?

Existing RAW-to-FITS tools tend to be technically dated, weakly documented, and workflow-unaware. AstroRAW-o-Matic is built as a modern, CLI-first, session-driven tool:

- **Reproducible**: session JSON captures your equipment and observation data // re-run the same conversion months later and get the same output.
- **Workflow-oriented**: designed as the step *before* stacking, not a replacement for it.
- **CLI-first**: usable in scripts, pipelines, and automation from day one.
- **GitHub-first**: professional structure, open source, MIT licensed.
- **Engine / UI separated**: same core will power a future GUI // no spaghetti.

---

## Features

- Convert Canon CR2 RAW files to FITS
- Preserve RAW Bayer data without debayering (astronomer-friendly)
- Automatically extract EXIF metadata (camera, exposure, ISO, date, focal length)
- Apply session metadata from JSON (object, telescope, filter, location, observer…)
- Full FITS header with all standard and astro-specific keywords
- Frame type classification: Light, Dark, Flat, Bias
- Batch directory conversion
- Dry-run mode
- Per-file metadata overrides
- Machine-readable JSON logging

---

## Supported Formats

| Format | Read | Notes |
|--------|------|-------|
| Canon CR2 | ✅ | Canon 600D/60Da and similar DSLRs |
| Canon CR3 | 🔜 | Planned |
| Nikon NEF | 🔜 | Planned |
| Sony ARW | 🔜 | Planned |

Output: FITS 16-bit (BITPIX=16), RAW Bayer mosaic.

---

## Installation

### From source (requires Rust toolchain)

```bash
git clone https://github.com/lindekai/AstroRAW-o-Matic.git
cd AstroRAW-o-Matic
cargo build --release
```

The binary will be at `target/release/astroraw-o-matic`.

Add it to your PATH, or create an alias `arom`:

```bash
alias arom='astroraw-o-matic'
```

### macOS (Homebrew) // coming soon
### Windows // binary releases planned
### Linux // binary releases planned

---

## Quickstart

### Inspect a RAW file

```bash
astroraw-o-matic inspect image.CR2
```

### Convert a single file

```bash
astroraw-o-matic convert image.CR2 --output ./FITS --object "M31" --type light
```

### Convert a directory with session metadata

```bash
astroraw-o-matic convert ./RAW --output ./FITS --metadata session.json
```

### Validate a session JSON

```bash
astroraw-o-matic validate session.json
```

### Dry run (see what would happen, write nothing)

```bash
astroraw-o-matic convert ./RAW --output ./FITS --metadata session.json --dry-run
```

---

## Session JSON

The session JSON is the contract between you, the CLI, and the FITS headers. It captures everything the RAW EXIF doesn't know about your observation.

```json
{
  "schema_version": "1.0",
  "object": "M31 Andromeda Galaxy",
  "observer": "K. Linde",
  "notes": "First light with new scope",
  "session_date": "2024-09-15",
  "frame_type": "light",
  "equipment": {
    "telescope": "Skywatcher 80ED",
    "focal_length": 600,
    "aperture": 7.5,
    "filter": "L",
    "camera": {
      "make": "Canon",
      "model": "EOS 600Da",
      "pixel_size_x": 4.3,
      "pixel_size_y": 4.3
    }
  },
  "location": {
    "latitude": 52.5200,
    "longitude": 13.4050,
    "elevation": 34.0,
    "site_name": "Backyard Observatory, Berlin"
  },
  "output": {
    "raw_mode": "raw_bayer",
    "header_mode": "astro",
    "overwrite": false,
    "write_history": true
  },
  "file_overrides": {
    "IMG_0001.CR2": {
      "frame_type": "dark"
    }
  }
}
```

### Metadata priority (highest to lowest)

1. File-specific override (`file_overrides` in session JSON)
2. Session-level JSON values
3. EXIF data extracted from the RAW file
4. Built-in defaults

---

## CLI Reference

### `convert`

```
astroraw-o-matic convert <INPUT> --output <DIR> [OPTIONS]

Options:
  --output <DIR>        Output directory for FITS files (required)
  --metadata <FILE>     Session JSON file
  --recursive           Recurse into subdirectories
  --type <TYPE>         Frame type: light, dark, flat, bias
  --object <NAME>       Target object name
  --telescope <NAME>    Telescope or lens
  --filter <NAME>       Filter (e.g. L, Ha, OIII)
  --observer <NAME>     Observer name
  --header-mode <MODE>  minimal or astro (default: astro)
  --overwrite           Overwrite existing FITS files
  --dry-run             Show what would happen, write nothing
  --log-file <FILE>     Write log to file
  --verbose             Verbose debug output
```

### `inspect`

```
astroraw-o-matic inspect <FILE> [--json]
```

Print metadata extracted from a RAW file. Use `--json` for machine-readable output.

### `validate`

```
astroraw-o-matic validate <session.json>
```

Validate a session JSON file and report warnings.

---

## FITS Headers

### Mandatory (always written)

| Keyword  | Description |
|----------|-------------|
| SIMPLE   | Conforms to FITS standard |
| BITPIX   | Bits per pixel (16) |
| NAXIS    | Number of dimensions |
| NAXIS1   | Image width |
| NAXIS2   | Image height |
| EXTEND   | Extensions permitted |

### Astro mode (default)

| Keyword  | Source | Description |
|----------|--------|-------------|
| INSTRUME | RAW EXIF / JSON | Camera model |
| EXPTIME  | RAW EXIF | Exposure time [s] |
| ISOSPEED | RAW EXIF | ISO setting |
| DATE-OBS | RAW EXIF | Date/time of observation |
| BAYERPAT | Inferred / JSON | Bayer CFA pattern |
| CFATYPE  | Inferred / JSON | CFA type |
| COLORTYP | Fixed | RAW (not debayered) |
| XPIXSZ   | JSON | Pixel width [µm] |
| YPIXSZ   | JSON | Pixel height [µm] |
| BITDEPTH | Inferred | Sensor ADC bit depth |
| IMAGETYP | JSON / CLI | Frame type (LIGHT/DARK/FLAT/BIAS) |
| OBJECT   | JSON / CLI | Target name |
| TELESCOP | JSON | Telescope / lens |
| FOCALLEN | RAW EXIF / JSON | Focal length [mm] |
| APERTURE | RAW EXIF / JSON | Aperture f-number |
| FILTER   | JSON | Filter name |
| OBSERVER | JSON | Observer name |
| SITELAT  | JSON | Site latitude |
| SITELONG | JSON | Site longitude |
| SITEELEV | JSON | Site elevation [m] |
| BLACKLVL | RAW | Black level (ADU) |
| WHITELEV | RAW | Saturation level (ADU) |
| SWCREATE | Fixed | AstroRAW-o-Matic version |
| HISTORY  | Auto | Conversion provenance |

---

## Architecture

```
AstroRAW-o-Matic
├── crates/
│   ├── astroraw-models/     # Shared data models (session JSON, FITS header, frame types)
│   ├── astroraw-core/       # Engine: RAW reading, metadata resolution, FITS writing
│   └── astroraw-cli/        # CLI frontend (uses core, no UI deps)
└── (future) crates/
    └── astroraw-gui/        # Tauri GUI // same core, different frontend
```

The engine is UI-free. The CLI is a thin layer over the engine. The future GUI will be another thin layer over the same engine.

---

## Roadmap

- [x] Canon CR2 EXIF metadata extraction
- [x] FITS 16-bit writer with full header
- [x] Session JSON metadata model
- [x] CLI: convert, inspect, validate
- [x] Batch directory conversion
- [x] Per-file metadata overrides
- [ ] LibRaw integration for actual pixel data extraction
- [ ] BZERO/BSCALE for proper unsigned FITS mapping
- [ ] Canon CR3 support
- [ ] Nikon NEF support
- [ ] Sony ARW support
- [ ] Shell completions (bash, zsh, fish)
- [ ] Homebrew formula
- [ ] Tauri GUI

---

## Building

```bash
# Debug build
cargo build

# Release build
cargo build --release

# Run tests
cargo test

# Run CLI directly
cargo run --bin astroraw-o-matic -- --help
```

### Prerequisites

- Rust 1.75+ (install via [rustup](https://rustup.rs))
- No C dependencies required for current build
- Future LibRaw integration will require a C compiler and LibRaw headers

---

## Platform Support

| Platform | Status |
|----------|--------|
| macOS (Apple Silicon) | ✅ |
| macOS (Intel) | ✅ |
| Linux (x86_64) | ✅ |
| Windows 10/11 | ✅ |

---

## License

MIT // Copyright (c) 2026 Kai Linde // see [LICENSE](LICENSE).

---

## Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md).

---

*"The answer is 42. The question is what FITS keyword to use for it. We recommend OBJECT."*
