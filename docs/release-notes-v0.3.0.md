# AstroRAW-o-Matic v0.3.0

**Mostly harmless RAW conversion // now with correct colors.**

---

## Was ist neu

### Kritischer Fix: Bayer-Pattern-Korrektur

Das wichtigste Update dieser Version betrifft die Farbkanalzuweisung im FITS-Output.

Bisherige Versionen schrieben `BAYERPAT=RGGB` für Canon-Kameras // eine falsche Annahme. Die Canon EOS 600D (und ähnliche Modelle) liefern nach rawlers Crop-Berechnung tatsächlich das Muster **GBRG**. Das führte dazu, dass Grün- und Blaukanal beim Debayering in Siril vertauscht wurden.

**Auswirkung der Korrektur**, gemessen mit photometrischer Farbkalibrierung (Siril PCC, Gaia DR3):

| Kanal | CR2 direkt | FITS v0.3.0 | Differenz |
|-------|-----------|-------------|-----------|
| K0 (Rot)  | 0.730 | 0.738 | +1.1% |
| K1 (Grün) | 0.584 | 0.589 | +0.9% |
| K2 (Blau) | 1.000 | 1.000 |  0.0% |
| B0 (Hintergrund R) | 660.7 | 639.1 | −3.3% |
| B1 (Hintergrund G) | 957.0 | 944.2 | −1.3% |
| B2 (Hintergrund B) | 160.0 | 160.7 | +0.4% |

Die Konvertierung ist damit photometrisch verifiziert. Die Restabweichung von unter 3% ist auf unterschiedliche Debayering-Algorithmen zwischen Sirils CR2-Pipeline und der FITS-Pipeline zurückzuführen // im normalen Toleranzbereich für Astrofotografie-Workflows.

### Schwarzpegel-Fix (PEDESTAL)

Siril liest den Schwarzpegel aus dem FITS-Keyword `PEDESTAL`. Bisher wurde nur `BLACKLVL` geschrieben, das Siril nicht kennt. Das fehlende `PEDESTAL` führte dazu, dass der Schwarzpegel (~2046 ADU) nicht vor dem Debayering abgezogen wurde, was die Farbverhältnisse verschob. Beide Keywords werden jetzt geschrieben.

### FITS-Header-Korrekturen

- `DATE-OBS` wurde bisher auf 18 Zeichen abgeschnitten // jetzt vollständig
- `SWCREATE` wurde abgeschnitten // jetzt vollständig
- `APERTURE` wird nicht mehr geschrieben wenn EXIF den Wert 0 liefert (Manualobjektive)

### Neue FITS-Keywords

| Keyword  | Bedeutung |
|----------|-----------|
| `FOCRATIO` | Brennweite / Öffnung (z.B. 5.0 für f/5) |
| `APTDIA`   | Öffnungsdurchmesser in mm |
| `PEDESTAL` | Schwarzpegel für Siril-Kompatibilität |
| `CBLACK_R/G/B` | Aufnahme-Weißabgleich-Koeffizienten (normiert auf G=1) |

### GUI-Grundgerüst (Early Alpha)

Erster Entwurf der Tauri + Svelte-Oberfläche:

- Session-Formular (Objekt, Beobachter, Datum/Uhrzeit, Kamera, Teleskop, Standort)
- Dateiliste mit Drag & Drop aus dem Finder
- Session-Datum und -Uhrzeit automatisch aus EXIF vorausgefüllt
- JSON laden und speichern (konfigurierbares Dateinamen-Muster)
- Filter-Auswahlliste: UV/IR, DNB, Halpha, OIII, SII, L, R, G, B
- Fokalverhältnis wird live aus Brennweite und Öffnung berechnet
- Konvertierung direkt aus der GUI

Die GUI ist noch Early Alpha // für produktive Konvertierungen empfehlen wir weiterhin die CLI.

---

## Unterstützte Plattformen

| Plattform | Status |
|-----------|--------|
| macOS (Apple Silicon) | ✅ getestet |
| macOS (Intel) | ✅ kompilierbar |
| Linux (x86_64) | ✅ kompilierbar |
| Windows 10/11 | ✅ kompilierbar |

---

## Getestete Hardware

- **Kamera**: Canon EOS 600D / 600Da
- **Aufnahme**: Horsehead Nebula, 120s, ISO 800
- **Stacking-Software**: Siril 1.4.1

---

## Installation

```bash
git clone https://github.com/lindekai/AstroRAW-o-Matic.git
cd AstroRAW-o-Matic
cargo build --release
# Binary: target/release/astroraw-o-matic
```

Voraussetzung: Rust 1.75+ via [rustup.rs](https://rustup.rs)

---

## Beispiel

```bash
astroraw-o-matic convert ./RAW --output ./FITS --metadata session.json
```

---

*"The answer is 42. The Bayer pattern is GBRG."*
