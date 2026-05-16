# AstroRAW-o-Matic

**Mostly harmless RAW conversion.**

> **⚠️ Frühes Alpha-Stadium // noch nicht für den Produktivbetrieb geeignet.**
> Die Kernarchitektur und CLI sind funktionsfähig, aber das Projekt befindet sich in aktiver Entwicklung. Breaking Changes sind möglich. Feedback und Beiträge sind willkommen.

> *„Der Weltraum ist groß. Du wirst nicht glauben, wie unvorstellbar, unfassbar, schwindelerregend groß er ist. Und CR2-Dateien sind es auch."*

AstroRAW-o-Matic ist ein reproduzierbares, workflow-orientiertes RAW-zu-FITS-Konvertierungstool für Astrofotografen. Es ist der saubere erste Schritt vor dem Stacking und der Bildbearbeitung // kein Konkurrent zu PixInsight, Siril oder DeepSkyStacker, sondern das Tool, das du *davor* einsetzt.

Es nimmt deine DSLR-RAW-Dateien und verwandelt sie in sauber geheaderte, reproduzierbare FITS-Dateien, die direkt von deiner Stacking-Software verarbeitet werden können.

---

## Community

**Dark Matters // Astro Community**
Fragen, Feedback und Diskussionen rund um AstroRAW-o-Matic findest du auf dem Dark Matters Discord:

> 🔗 [discord.gg/mvgC6aXY](https://discord.gg/mvgC6aXY)

---

## Warum AstroRAW-o-Matic?

Bestehende RAW-zu-FITS-Tools sind technisch veraltet, schlecht dokumentiert und ohne Workflow-Bewusstsein entwickelt. AstroRAW-o-Matic ist modern, CLI-first und session-orientiert aufgebaut:

- **Reproduzierbar**: Die Session-JSON speichert deine Equipment- und Beobachtungsdaten // führe dieselbe Konvertierung Monate später erneut aus und erhalte dasselbe Ergebnis.
- **Workflow-orientiert**: Entwickelt als der Schritt *vor* dem Stacking, nicht als Ersatz dafür.
- **CLI-first**: Von Anfang an in Skripten, Pipelines und Automatisierungen einsetzbar.
- **GitHub-first**: Professionelle Struktur, Open Source, MIT-Lizenz.
- **Engine / UI getrennt**: Dieselbe Engine wird später eine GUI antreiben // kein Spaghetti-Code.

---

## Features

- Canon CR2 RAW-Dateien in FITS konvertieren
- RAW-Bayer-Daten ohne Debayering erhalten (astrofreundlich)
- EXIF-Metadaten automatisch extrahieren (Kamera, Belichtung, ISO, Datum, Brennweite)
- Session-Metadaten aus JSON anwenden (Objekt, Teleskop, Filter, Standort, Beobachter …)
- Vollständiger FITS-Header mit Standard- und Astro-spezifischen Keywords
- Frame-Typ-Klassifizierung: Light, Dark, Flat, Bias
- Batch-Verzeichniskonvertierung
- Dry-Run-Modus
- Datei-spezifische Metadaten-Overrides

---

## Unterstützte Formate

| Format | Lesen | Hinweis |
|--------|-------|---------|
| Canon CR2 | ✅ | Canon 600D/60Da und ähnliche DSLRs |
| Canon CR3 | 🔜 | Geplant |
| Nikon NEF | 🔜 | Geplant |
| Sony ARW | 🔜 | Geplant |

Ausgabe: FITS 16-bit (BITPIX=16), RAW-Bayer-Mosaik.

---

## Installation

### Aus dem Quellcode (erfordert Rust-Toolchain)

```bash
git clone https://github.com/lindekai/AstroRAW-o-Matic.git
cd AstroRAW-o-Matic
cargo build --release
```

Die fertige Binärdatei liegt unter `target/release/astroraw-o-matic`.

Zum PATH hinzufügen oder Alias anlegen:

```bash
alias arom='astroraw-o-matic'
```

### macOS (Homebrew) // demnächst
### Windows // Binary-Releases geplant
### Linux // Binary-Releases geplant

---

## Schnellstart

### RAW-Datei inspizieren

```bash
astroraw-o-matic inspect bild.CR2
```

### Einzeldatei konvertieren

```bash
astroraw-o-matic convert bild.CR2 --output ./FITS --object "M31" --type light
```

### Verzeichnis mit Session-Metadaten konvertieren

```bash
astroraw-o-matic convert ./RAW --output ./FITS --metadata session.json
```

### Session-JSON validieren

```bash
astroraw-o-matic validate session.json
```

### Dry-Run (zeigt was passieren würde, ohne Dateien zu schreiben)

```bash
astroraw-o-matic convert ./RAW --output ./FITS --metadata session.json --dry-run
```

---

## Session-JSON

Die Session-JSON ist der Vertrag zwischen dir, der CLI und den FITS-Headern. Sie erfasst alles, was das RAW-EXIF nicht kennt.

```json
{
  "schema_version": "1.0",
  "object": "M31 Andromedagalaxie",
  "observer": "Kai Linde",
  "notes": "Erste Beobachtungsnacht mit neuem Teleskop",
  "session_date": "2026-05-15",
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
    "site_name": "Gartensternwarte Berlin"
  },
  "output": {
    "raw_mode": "raw_bayer",
    "header_mode": "astro",
    "overwrite": false,
    "write_history": true
  },
  "file_overrides": {
    "dark_001.CR2": {
      "frame_type": "dark"
    }
  }
}
```

### Metadaten-Priorität (höchste zuerst)

1. Datei-spezifischer Override (`file_overrides` in der Session-JSON)
2. Session-weite JSON-Werte
3. EXIF-Daten aus der RAW-Datei
4. Interne Standardwerte

---

## CLI-Referenz

### `convert`

```
astroraw-o-matic convert <EINGABE> --output <VERZEICHNIS> [OPTIONEN]

Optionen:
  --output <DIR>        Ausgabeverzeichnis für FITS-Dateien (erforderlich)
  --metadata <FILE>     Session-JSON-Datei
  --recursive           Unterverzeichnisse einschließen
  --type <TYPE>         Frame-Typ: light, dark, flat, bias
  --object <NAME>       Zielobjektname
  --telescope <NAME>    Teleskop oder Objektiv
  --filter <NAME>       Filter (z. B. L, Ha, OIII)
  --observer <NAME>     Name des Beobachters
  --header-mode <MODE>  minimal oder astro (Standard: astro)
  --overwrite           Vorhandene FITS-Dateien überschreiben
  --dry-run             Zeigt was passieren würde, schreibt nichts
  --log-file <FILE>     Log in Datei schreiben
  --verbose             Ausführliche Debug-Ausgabe
```

### `inspect`

```
astroraw-o-matic inspect <DATEI> [--json]
```

Gibt die aus einer RAW-Datei extrahierten Metadaten aus. Mit `--json` maschinenlesbar.

### `validate`

```
astroraw-o-matic validate <session.json>
```

Validiert eine Session-JSON-Datei und gibt Warnungen aus.

---

## FITS-Header

### Pflichtfelder (immer geschrieben)

| Keyword  | Beschreibung |
|----------|-------------|
| SIMPLE   | FITS-Standard-Konformität |
| BITPIX   | Bits pro Pixel (16) |
| NAXIS    | Anzahl der Dimensionen |
| NAXIS1   | Bildbreite |
| NAXIS2   | Bildhöhe |
| EXTEND   | Erweiterungen erlaubt |

### Astro-Modus (Standard)

| Keyword  | Quelle | Beschreibung |
|----------|--------|-------------|
| INSTRUME | RAW EXIF / JSON | Kameramodell |
| EXPTIME  | RAW EXIF | Belichtungszeit [s] |
| ISOSPEED | RAW EXIF | ISO-Einstellung |
| DATE-OBS | RAW EXIF | Datum/Uhrzeit der Aufnahme |
| BAYERPAT | rawler / JSON | Bayer-CFA-Muster |
| CFATYPE  | rawler / JSON | CFA-Typ |
| COLORTYP | Fix | RAW (nicht debayert) |
| XPIXSZ   | JSON | Pixelbreite [µm] |
| YPIXSZ   | JSON | Pixelhöhe [µm] |
| BITDEPTH | Berechnet | Sensor-ADC-Bittiefe |
| IMAGETYP | JSON / CLI | Frame-Typ (LIGHT/DARK/FLAT/BIAS) |
| OBJECT   | JSON / CLI | Zielobjektname |
| TELESCOP | JSON | Teleskop / Objektiv |
| FOCALLEN | RAW EXIF / JSON | Brennweite [mm] |
| APERTURE | RAW EXIF / JSON | Blendenzahl |
| FILTER   | JSON | Filtername |
| OBSERVER | JSON | Name des Beobachters |
| SITELAT  | JSON | Standortbreite [°] |
| SITELONG | JSON | Standortlänge [°] |
| SITEELEV | JSON | Standorthöhe [m] |
| BLACKLVL | rawler | Schwarzpegel (ADU) |
| WHITELEV | rawler | Sättigungspegel (ADU) |
| SWCREATE | Fix | AstroRAW-o-Matic Version |
| HISTORY  | Automatisch | Konvertierungs-Provenienz |

---

## Architektur

```
AstroRAW-o-Matic
├── crates/
│   ├── astroraw-models/     # Gemeinsame Datenmodelle
│   ├── astroraw-core/       # Engine: RAW lesen, Metadaten auflösen, FITS schreiben
│   └── astroraw-cli/        # CLI-Frontend (nutzt Core, keine UI-Abhängigkeiten)
└── (zukünftig)
    └── astroraw-gui/        # Tauri-GUI // gleiche Engine, anderes Frontend
```

---

## Roadmap

- [x] Canon CR2 EXIF-Metadaten-Extraktion
- [x] FITS 16-bit-Writer mit vollständigem Header
- [x] Session-JSON-Metadatenmodell
- [x] CLI: convert, inspect, validate
- [x] Batch-Verzeichniskonvertierung
- [x] Optische Schwarzränder automatisch zuschneiden
- [x] Datei-spezifische Metadaten-Overrides
- [ ] Canon CR3-Unterstützung
- [ ] Nikon NEF-Unterstützung
- [ ] Sony ARW-Unterstützung
- [ ] Shell-Vervollständigungen (bash, zsh, fish)
- [ ] Homebrew-Formel
- [ ] Tauri-GUI

---

## Build

```bash
# Debug-Build
cargo build

# Release-Build
cargo build --release

# Tests ausführen
cargo test

# CLI direkt ausführen
cargo run --bin astroraw-o-matic -- --help
```

### Voraussetzungen

- Rust 1.75+ (Installation via [rustup](https://rustup.rs))
- Keine C-Abhängigkeiten für den aktuellen Build erforderlich

---

## Plattform-Unterstützung

| Plattform | Status |
|-----------|--------|
| macOS (Apple Silicon) | ✅ |
| macOS (Intel) | ✅ |
| Linux (x86_64) | ✅ |
| Windows 10/11 | ✅ |

---

## Lizenz

MIT // Copyright (c) 2026 Kai Linde // siehe [LICENSE](LICENSE).

---

## Mitwirken

Siehe [CONTRIBUTING.md](CONTRIBUTING.md).

---

*„Die Antwort ist 42. Die Frage ist, welches FITS-Keyword man dafür verwendet. Wir empfehlen OBJECT."*
