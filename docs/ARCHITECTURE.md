# Architecture

## Workspace crates

```
AstroRAW-o-Matic (Cargo workspace)
├── crates/astroraw-models   — shared data types, no logic
├── crates/astroraw-core     — engine: RAW in, FITS out
└── crates/astroraw-cli      — CLI frontend
```

Future:
```
└── crates/astroraw-gui      — Tauri GUI frontend (same engine)
```

---

## astroraw-models

Pure data types. No I/O, no logic. Shared by core and all frontends.

- `SessionMetadata` — the session JSON schema
- `FitsHeader` / `FitsHeaderRecord` / `FitsValue` — FITS header representation
- `RawMetadata` — metadata extracted from RAW EXIF
- `FrameType` — Light / Dark / Flat / Bias
- `EquipmentMetadata`, `CameraMetadata`, `LocationMetadata`

---

## astroraw-core

The engine. No CLI or GUI dependencies.

### raw::reader
Reads EXIF from RAW files using `kamadak-exif`. Returns `RawMetadata`.

Future: LibRaw FFI for actual pixel data.

### metadata::resolver
Merges four metadata sources into one `FitsHeader` + warning list.

Priority chain:
1. `file_overrides[filename]` in session JSON
2. session-level JSON fields
3. RAW EXIF (`RawMetadata`)
4. hard defaults

### fits::writer
Writes a FITS 2880-byte-block-aligned file from a `FitsHeader` + `RawPixelData`.
- Mandatory header block
- Data block (16-bit big-endian integers, Bayer mosaic)
- Pad to block boundary

### session::loader
Reads and deserializes a session JSON file into `SessionMetadata`.

### session::validator
Checks a `SessionMetadata` for missing or suspicious fields. Returns a `ValidationReport`.

### Top-level API
- `collect_raw_files(path, recursive)` — enumerate supported RAW files
- `convert_single(path, request)` — convert one file, return `ConvertResult`
- `convert_batch(request)` — convert many files, return `BatchSummary`
- `load_session_file(path)` — load session JSON

---

## astroraw-cli

Thin layer. Parses `clap` args, builds a `ConvertRequest`, calls the engine, renders output.

### Subcommands
- `convert` — batch or single file conversion
- `inspect` — print RAW metadata (human or JSON)
- `validate` — check session JSON

### Exit codes
- `0` — success
- `1` — one or more errors (files failed, invalid JSON, etc.)

---

## Data flow (convert)

```
CLI args + session.json
        │
        ▼
  ConvertRequest
        │
        ├─► collect_raw_files()
        │         │
        │         ▼
        │   [path, path, ...]
        │
        └─► convert_batch()
                  │
                  for each file:
                  │
                  ├─► RawReader::read_metadata()  →  RawMetadata
                  ├─► MetadataResolver::resolve()  →  FitsHeader + warnings
                  ├─► RawReader::read_raw_bayer()  →  RawPixelData
                  └─► FitsWriter::write()          →  .fits file
```

---

## Metadata priority (detailed)

For each output field, the resolver walks this chain and takes the first non-None value:

| Priority | Source |
|----------|--------|
| 1 | `session.file_overrides[filename].field` |
| 2 | `session.field` |
| 3 | EXIF extracted from RAW file |
| 4 | Hard default (documented per field) |

---

## FITS format details

- BITPIX = 16 (signed 16-bit integers)
- RAW Bayer pixel values stored as-is (no demosaicing)
- BZERO / BSCALE for proper unsigned-to-signed mapping: planned for next milestone
- All header blocks aligned to 2880 bytes
- END card always present
- HISTORY record added by default

---

## LibRaw integration plan

The current `RawReader::read_raw_bayer()` returns a zero-filled placeholder.

The interface is:

```rust
pub fn read_raw_bayer(path: &Path) -> Result<RawPixelData>
```

Replacing the body with LibRaw FFI calls (via `libraw-sys` or a hand-written binding) is the next major milestone. The rest of the codebase does not need to change.

---

## GUI preparation

The GUI will:
1. Use `astroraw-core` directly (same engine, no subprocess)
2. Build a `SessionMetadata` from form fields
3. Call `convert_batch()` with progress callbacks (to be added to `ConvertRequest`)
4. Display warnings and errors from `ConvertResult`

The separation between `astroraw-models`, `astroraw-core`, and the frontend is the key design invariant.
