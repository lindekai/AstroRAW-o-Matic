# AstroRAW-o-Matic // Übergabe-Dokument

**Stand: 2026-05-19** // Für neue Entwicklungssessions auf anderen Rechnern.

---

## Projekt-Kurzübersicht

AstroRAW-o-Matic konvertiert DSLR-RAW-Dateien (Canon CR2) in FITS für die Astrofotografie.
Rust + Tauri + Svelte // MIT-Lizenz // Autor: Kai Linde

**GitHub:** https://github.com/lindekai/AstroRAW-o-Matic
**Community:** https://discord.gg/mvgC6aXY (Dark Matters)

---

## Aktueller Stand

### Was fertig und verifiziert ist

| Komponente | Status |
|------------|--------|
| CLI: convert / inspect / validate | ✅ produktionsbereit |
| FITS 16-bit Writer | ✅ verifiziert |
| Bayer-Pattern GBRG (Canon 600D) | ✅ photometrisch < 3% |
| 62 N.I.N.A.-kompatible FITS-Keywords | ✅ implementiert |
| Session JSON Schema v1.1 | ✅ stabil |
| GUI: Tauri + Svelte | ✅ macOS .app gebaut |
| GUI: Ordner-Picker + Drag & Drop | ✅ |
| GUI: Hilfe-Dialog DE/EN | ✅ |
| GitHub Actions Release-Workflow | ✅ |
| Homebrew Formula | ✅ |
| N.I.N.A. Plugin Scaffold (C#) | ✅ Scaffold, Impl. offen |

### Photometrische Verifikation (Siril PCC, Gaia DR3)

Horsehead Nebula // Canon EOS 600D // 120s // ISO 800:

```
         CR2      FITS     Diff
K0 (R):  0.730    0.738    +1.1% ✅
K1 (G):  0.584    0.589    +0.9% ✅
K2 (B):  1.000    1.000     0.0% ✅
```

---

## Kritische technische Entscheidungen

### 1. Bayer-Pattern: GBRG (nicht RGGB!)

Canon 600D liefert nach dem rawler-Crop bei (152,56) das Muster **GBRG**.
Berechnung in `crates/astroraw-core/src/raw/reader.rs` via `shift_bayer_pattern()`.

```
rawler camera.cfa.name = "GBRG" (relativ zum vollen Sensor)
Crop bei (152, 56) → beide gerade → kein Pattern-Shift
Effektives Pattern = GBRG ✅
```

**Nicht ändern ohne neuen Siril-PCC-Test.**

### 2. Kein BZERO=32768

Canon 14-bit RAW-Werte (0..16383) passen in signed 16-bit ohne Offset.
BZERO=32768 würde alle Pixel um +32768 verschieben → falsche Farben in Siril.

### 3. Kein ROWORDER="TOP-DOWN"

Verursacht Bild-Flip in Siril → Bayer-Pattern-Verschiebung.

### 4. PEDESTAL=2046

Siril liest diesen Keyword für automatischen Schwarzpegel-Abzug vor dem Debayering.

---

## Workspace-Struktur

```
AstroRAW-o-Matic/
├── Cargo.toml                    # Workspace root (v0.3.0)
├── crates/
│   ├── astroraw-models/          # Shared types: SessionMetadata, FitsHeader, etc.
│   ├── astroraw-core/            # Engine: rawler, EXIF, FITS writer, resolver
│   ├── astroraw-cli/             # CLI binary
│   └── astroraw-gui/             # Tauri + Svelte GUI
│       ├── frontend/             # Svelte app
│       └── src-tauri/            # Rust backend
├── integrations/
│   └── nina-plugin/              # C# N.I.N.A. Bridge Plugin (Scaffold)
│       ├── src/                  # Plugin.cs, BridgeService.cs, MetadataBuilder.cs
│       └── docs/                 # NINA_INTEGRATION.md, UI_PREVIEW.md
├── Formula/
│   └── astroraw-o-matic.rb       # Homebrew Formula
├── .github/workflows/
│   ├── ci.yml                    # Tests auf allen 3 Plattformen
│   └── release.yml               # Cross-Platform Release-Builds auf Tag
├── docs/                         # Architektur, Roadmap, User Guide DE/EN
├── examples/                     # Beispiel session.json
└── logo/                         # Dark Matters Logo (SVG + PNG)
```

---

## Entwicklungssetup (macOS)

```bash
# Voraussetzungen
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
source "$HOME/.cargo/env"
node --version  # Node 20+ nötig

# Repo
git clone https://github.com/lindekai/AstroRAW-o-Matic.git
cd AstroRAW-o-Matic

# CLI bauen
cargo build --release --bin astroraw-o-matic

# GUI starten (Dev-Modus)
cd crates/astroraw-gui
cargo tauri dev
```

---

## GUI starten (nach Klon)

```bash
cd crates/astroraw-gui
npm install --prefix frontend
source "$HOME/.cargo/env"
cargo tauri dev
```

---

## Nächste offene Punkte

### 1. N.I.N.A. Plugin (Windows-Rechner nötig)

Scaffold unter `integrations/nina-plugin/src/` ist fertig.
Braucht noch:
- Echte N.I.N.A. `IPlugin`-Interface-Implementierung in `Plugin.cs`
- WPF-Einstellungs-UI (`PluginOptionsView.xaml` vervollständigen)
- Testing mit echter N.I.N.A.-Installation

**Windows-Setup:** `integrations/nina-plugin/docs/NINA_INTEGRATION.md`

### 2. Weitere Kameraformate

- Canon CR3
- Nikon NEF
- Sony ARW

### 3. Shell-Completions

```bash
astroraw-o-matic completions zsh > ~/.zsh/completions/_astroraw-o-matic
```

### 4. Nächstes Release auslösen

```bash
git tag -a v0.4.0 -m "v0.4.0 — <beschreibung>"
git push --tags
# → GitHub Actions baut automatisch alle Plattformen
```

---

## Testdaten (lokal, nicht im Repo)

```
test-data/
├── Horsehead Nebula_SW PDS_LIGHT_120.00_800_4.74_0022.cr2
├── Horsehead Nebula_SW PDS_LIGHT_120.00_800_4.76_0019.cr2
├── Horsehead Nebula_SW PDS_LIGHT_120.00_800_4.84_0021.cr2
└── session_horsehead.json   (Focal 1000mm, Aperture 200mm, Canon EOS 600Da)
```

Testkonvertierung:
```bash
target/release/astroraw-o-matic convert test-data/ \
  --output test-output \
  --metadata test-data/session_horsehead.json \
  --overwrite
```

---

*AstroRAW-o-Matic // Mostly harmless RAW conversion*
