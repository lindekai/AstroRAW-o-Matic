# AstroRAW-o-Matic // Benutzerhandbuch

**Version 0.3.0** | [English version](USER_GUIDE.md) | [Dark Matters Community](https://discord.gg/mvgC6aXY)

---

## Inhaltsverzeichnis

1. [Was ist AstroRAW-o-Matic?](#was-ist-astroraw-o-matic)
2. [Installation](#installation)
3. [Empfohlener Workflow](#empfohlener-workflow)
4. [GUI-Bedienung](#gui-bedienung)
5. [CLI-Referenz](#cli-referenz)
6. [Session-JSON Referenz](#session-json-referenz)
7. [FITS-Header Referenz](#fits-header-referenz)
8. [Häufige Fragen](#häufige-fragen)

---

## Was ist AstroRAW-o-Matic?

AstroRAW-o-Matic konvertiert DSLR-RAW-Dateien (Canon CR2) in FITS-Dateien für die Astrofotografie. Es ist der saubere Schritt **vor** dem Stacking // kein Ersatz für PixInsight, Siril oder DeepSkyStacker, sondern die Vorbereitung dafür.

**Was das Tool tut:**
- RAW-Bayer-Daten ohne Debayering erhalten
- Alle relevanten Metadaten in den FITS-Header schreiben
- Reproduzierbare, dokumentierte Konvertierungen ermöglichen

**Was das Tool nicht tut:**
- Kein Stacking
- Keine Bildbearbeitung
- Keine Kalibrierung (Darks/Flats)
- Kein Debayering (bleibt dem Stacking-Tool überlassen)

---

## Installation

### Voraussetzungen

- Rust 1.75+ → [rustup.rs](https://rustup.rs)

### Build

```bash
git clone https://github.com/lindekai/AstroRAW-o-Matic.git
cd AstroRAW-o-Matic
cargo build --release
```

Binary: `target/release/astroraw-o-matic`

### Alias einrichten (empfohlen)

```bash
# In ~/.zshrc oder ~/.bashrc
alias arom='astroraw-o-matic'
```

---

## Empfohlener Workflow

```
DSLR-Aufnahmen (CR2)
        │
        ▼
 AstroRAW-o-Matic
 ┌─────────────────────────────┐
 │ Session-JSON erstellen      │
 │ Metadaten eintragen         │
 │ CR2 → FITS konvertieren     │
 └─────────────────────────────┘
        │
        ▼
   FITS-Dateien
   (16-bit, GBRG, mit vollem Header)
        │
        ▼
   Siril / PixInsight / APP
   ┌─────────────────────────────┐
   │ Kalibrieren (Darks, Flats) │
   │ Debayern                   │
   │ Stacken                    │
   │ Bildbearbeitung            │
   └─────────────────────────────┘
```

### Wichtige Hinweise für Siril

- Bayer-Muster für Canon 600D/600Da: **GBRG** (automatisch gesetzt)
- `PEDESTAL`-Keyword ist gesetzt → Siril subtrahiert den Schwarzpegel automatisch
- Photometrische Farbkalibrierung ergibt < 3% Abweichung zum CR2-Direktimport

---

## GUI-Bedienung

### Dateien laden

**Option 1 // Drag & Drop:** CR2-Dateien aus dem Finder direkt ins Fenster ziehen.

**Option 2 // Datei-Dialog:** Auf **+ Add Files** klicken.

Session Date und Session Time werden automatisch aus dem EXIF der ersten geladenen Datei vorausgefüllt.

### Session-Formular

| Feld | Beschreibung |
|------|-------------|
| Object | Zielobjekt (M31, NGC 7000, Horsehead Nebula …) |
| Observer | Name des Beobachters |
| Session Date | Beobachtungsdatum (aus EXIF vorausgefüllt) |
| Session Time | Beobachtungszeit UTC (aus EXIF vorausgefüllt) |
| Frame Type | Light / Dark / Flat / Bias |
| Filter | UV/IR, DNB, Halpha, OIII, SII, L, R, G, B |
| Camera Make/Model | Kamerahersteller und -modell |
| Pixel X/Y (µm) | Pixelgröße (Canon 600D/600Da: 4.3 µm) |
| Telescope | Teleskop oder Objektiv |
| Focal Length (mm) | Brennweite |
| Aperture (mm) | Öffnungsdurchmesser |
| Focal Ratio | Automatisch berechnet (Brennweite ÷ Öffnung) |
| Site Name | Name des Beobachtungsorts |
| Latitude / Longitude | Koordinaten des Standorts |
| Elevation (m) | Höhe über NN |

### JSON laden und speichern

- **📂 Load JSON** // bestehende Session-JSON laden
- **💾 Save JSON** // aktuelle Session-Einstellungen speichern

**Dateinamen-Muster** (Feld im OUTPUT-Bereich):

| Platzhalter | Bedeutung |
|-------------|-----------|
| `{object}` | Objektname (Leerzeichen → Unterstriche) |
| `{date}` | Aktuelles Datum (YYYY-MM-DD) |
| `{observer}` | Name des Beobachters |

Beispiel: `session_{object}_{date}` → `session_Horsehead_Nebula_2026-05-17.json`

### Konvertieren

1. Output-Ordner wählen: **Output Folder**
2. **Convert to FITS** klicken
3. Ergebnisse werden unten angezeigt (Succeeded / Failed)

---

## CLI-Referenz

### `inspect` // Metadaten anzeigen

```bash
astroraw-o-matic inspect bild.CR2
astroraw-o-matic inspect bild.CR2 --json   # maschinenlesbar
```

### `validate` // Session-JSON prüfen

```bash
astroraw-o-matic validate session.json
```

### `convert` // Konvertieren

```bash
# Einzeldatei
astroraw-o-matic convert bild.CR2 \
  --output ./FITS \
  --object "M31" \
  --type light

# Verzeichnis mit Session-JSON
astroraw-o-matic convert ./RAW \
  --output ./FITS \
  --metadata session.json

# Dry-Run (nichts schreiben)
astroraw-o-matic convert ./RAW \
  --output ./FITS \
  --dry-run

# Rekursiv mit Überschreiben
astroraw-o-matic convert ./RAW \
  --output ./FITS \
  --recursive \
  --overwrite
```

#### Alle Optionen

| Option | Beschreibung |
|--------|-------------|
| `--output <DIR>` | Ausgabeverzeichnis (Pflicht) |
| `--metadata <FILE>` | Session-JSON-Datei |
| `--type <TYPE>` | light, dark, flat, bias |
| `--object <NAME>` | Objektname |
| `--observer <NAME>` | Beobachtername |
| `--telescope <NAME>` | Teleskop/Objektiv |
| `--filter <NAME>` | Filtername |
| `--date-obs <DATETIME>` | Datum/Zeit überschreiben (ISO 8601) |
| `--header-mode <MODE>` | minimal oder astro (Standard) |
| `--recursive` | Unterverzeichnisse einschließen |
| `--overwrite` | Vorhandene FITS überschreiben |
| `--dry-run` | Nur anzeigen, nichts schreiben |
| `--verbose` | Ausführliche Ausgabe |

---

## Session-JSON Referenz

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
    "site_name": "Gartensternwarte Berlin"
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

### Metadaten-Priorität

1. `file_overrides[dateiname]`
2. Session-JSON-Werte
3. EXIF aus der RAW-Datei
4. Standardwerte

---

## FITS-Header Referenz

| Keyword | Inhalt | Quelle |
|---------|--------|--------|
| SIMPLE | T | Standard |
| BITPIX | 16 | Standard |
| NAXIS1/2 | Bildbreite/-höhe | rawler |
| INSTRUME | Kameramodell | EXIF |
| EXPTIME | Belichtungszeit [s] | EXIF |
| ISOSPEED | ISO-Einstellung | EXIF |
| DATE-OBS | Aufnahmedatum/-zeit | EXIF / Session |
| BAYERPAT | Bayer-Muster (GBRG) | rawler |
| CFATYPE | CFA-Typ | rawler |
| COLORTYP | RAW | fest |
| XPIXSZ/YPIXSZ | Pixelgröße [µm] | Session |
| BITDEPTH | ADC-Bittiefe (14) | rawler |
| IMAGETYP | LIGHT/DARK/FLAT/BIAS | Session / CLI |
| OBJECT | Zielobjektname | Session / CLI |
| TELESCOP | Teleskop/Objektiv | Session |
| FOCALLEN | Brennweite [mm] | EXIF / Session |
| APERTURE | Öffnung [mm] | Session |
| APTDIA | Öffnung [mm] | Session |
| FOCRATIO | Fokalverhältnis | berechnet |
| FILTER | Filtername | Session |
| OBSERVER | Beobachtername | Session |
| SITELAT/LONG | Standortkoordinaten | Session |
| SITEELEV | Standorthöhe [m] | Session |
| BLACKLVL | Schwarzpegel [ADU] | rawler |
| PEDESTAL | Schwarzpegel [ADU] | rawler |
| WHITELEV | Sättigungspegel [ADU] | rawler |
| CBLACK_R/G/B | WB-Koeffizienten | rawler |
| SWCREATE | AstroRAW-o-Matic v0.3.0 | fest |
| HISTORY | Konvertierungsnachweis | automatisch |

---

## Häufige Fragen

**Warum sieht das FITS im Vorschaumodus anders aus als das CR2?**
Das FITS enthält rohe lineare Sensordaten ohne Weißabgleich oder Schwarzpegel-Korrektur. Nach Debayering und photometrischer Kalibrierung in Siril sind die Ergebnisse identisch (< 3% Abweichung).

**Welches Bayer-Muster hat die Canon 600D/600Da?**
GBRG // wird automatisch korrekt gesetzt. Nicht manuell ändern.

**Was tun wenn das Datum in der Kamera falsch war?**
`--date-obs "2024-01-09T21:34:00"` in der CLI, oder `date_obs` im Session-JSON, oder `session_date` + `session_time` in der GUI.

**Kann ich mehrere verschiedene Objekte in einer Batch-Konvertierung haben?**
Ja // über `file_overrides` im Session-JSON kannst du für einzelne Dateien `object`, `frame_type`, `filter` und `date_obs` überschreiben.

---

*„Die Antwort ist 42. Das Bayer-Muster ist GBRG."*
