# Roadmap

## v0.1 // MVP (current)

- [x] Cargo workspace: models / core / cli
- [x] EXIF metadata extraction from CR2
- [x] FITS 16-bit writer with full astro header
- [x] Session JSON schema v1.0
- [x] Metadata priority chain
- [x] CLI: convert / inspect / validate
- [x] Batch conversion
- [x] Dry-run mode

## v0.2 // Real pixels

- [ ] LibRaw FFI integration (`read_raw_bayer` returns real data)
- [ ] BZERO=32768 + BSCALE=1.0 for correct unsigned 16-bit FITS storage
- [ ] Black level / white level from LibRaw
- [ ] Progress output during batch conversion

## v0.3 // More cameras

- [ ] Canon CR3
- [ ] Nikon NEF
- [ ] Sony ARW
- [ ] Fujifilm RAF (X-Trans pattern)

## v0.4 // Polish

- [ ] Shell completions (bash, zsh, fish, PowerShell)
- [ ] Homebrew formula (macOS)
- [ ] Windows installer / MSI
- [ ] JSON log file output (`--log-file`)
- [ ] CI/CD: GitHub Actions for cross-platform release builds

## v0.5 // GUI prototype

- [ ] Tauri GUI frontend
- [ ] Drag & drop RAW files
- [ ] Session form (object, equipment, location, frame type)
- [ ] Equipment presets
- [ ] FITS header preview
- [ ] Batch progress bar
- [ ] Error list

## Future

- [ ] Optional debayering (for preview / quick-look)
- [ ] Auto-stretch preview in GUI
- [ ] NINA/Ekos/Sequence Generator Pro integration notes
