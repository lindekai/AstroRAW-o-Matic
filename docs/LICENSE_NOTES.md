# License Notes

AstroRAW-o-Matic is MIT licensed.

## Dependency licenses

| Dependency | License | Notes |
|------------|---------|-------|
| kamadak-exif | MIT | Pure Rust EXIF reader — MIT compatible |
| serde / serde_json | MIT/Apache-2.0 | Standard, fully compatible |
| clap | MIT/Apache-2.0 | Fully compatible |
| thiserror | MIT/Apache-2.0 | Fully compatible |
| anyhow | MIT/Apache-2.0 | Fully compatible |
| tracing | MIT | Fully compatible |
| chrono | MIT/Apache-2.0 | Fully compatible |
| walkdir | MIT/Unlicense | Fully compatible |

## LibRaw (planned)

LibRaw is dual-licensed LGPL 2.1 / CDDL 1.0.

### Impact on MIT distribution

LGPL allows dynamic linking without license propagation to the calling application. As long as we:

1. Link LibRaw **dynamically** (preferred), OR
2. Provide LibRaw source / object files so users can relink

...the MIT license on AstroRAW-o-Matic remains valid and distributable.

**Static linking LibRaw would require releasing AstroRAW-o-Matic under a license compatible with LGPL.**
We will default to dynamic linking and document this clearly in build instructions.

Alternatively, `rawler` (pure Rust RAW decoder, MIT licensed) is being evaluated as a fully MIT-compatible alternative for pixel data extraction.

## FITS libraries

`fitsio` / `cfitsio` bindings: cfitsio itself is BSD/NASA open-source license — compatible with MIT for distribution.

For the MVP we use our own minimal FITS writer, avoiding external C dependencies entirely.
