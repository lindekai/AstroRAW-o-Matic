# AstroRAW-o-Matic Bridge Plugin // UI-Vorschau

## Wo das Plugin in N.I.N.A. erscheint

```
N.I.N.A.
├── Options
│   └── Plugins
│       └── [AstroRAW-o-Matic Bridge]  ← Plugin-Einstellungen
└── Imaging
    └── Image Panel
        └── [Conversion: ✓ IMG_0042.fits]  ← Status-Meldungen im Log
```

---

## Plugin-Einstellungen (Options → Plugins → AstroRAW-o-Matic Bridge)

```
╔══════════════════════════════════════════════════════════════════════╗
║  🔭  AstroRAW-o-Matic Bridge Plugin  v0.1.0                         ║
║  Mostly harmless RAW conversion                                      ║
╠══════════════════════════════════════════════════════════════════════╣
║                                                                      ║
║  ── CONVERTER ─────────────────────────────────────────────────────  ║
║                                                                      ║
║  CLI Pfad                                                            ║
║  ┌─────────────────────────────────────────────┐  [Durchsuchen...]  ║
║  │ C:\Programme\AstroRAW-o-Matic\arom.exe      │                    ║
║  └─────────────────────────────────────────────┘                    ║
║  ● Status: ✓ CLI gefunden (v0.3.0)                                  ║
║                                                                      ║
║  Header-Modus      ┌──────────────┐                                 ║
║                    │ Astro (voll) ▼│                                 ║
║                    └──────────────┘                                 ║
║                                                                      ║
║  ── AUSGABE ───────────────────────────────────────────────────────  ║
║                                                                      ║
║  ☑  FITS neben RAW speichern (gleicher Ordner)                      ║
║  ☐  Eigener FITS-Zielordner:                                        ║
║     ┌─────────────────────────────────────────┐  [Durchsuchen...]  ║
║     │                                         │                    ║
║     └─────────────────────────────────────────┘                    ║
║                                                                      ║
║  ☑  RAW-Datei nach Konvertierung behalten                           ║
║  ☐  session.json neben FITS speichern                               ║
║                                                                      ║
║  ── BEOBACHTER / STANDORT ─────────────────────────────────────────  ║
║                                                                      ║
║  Beobachter        ┌──────────────────────────┐                    ║
║                    │ Kai Linde                │                    ║
║                    └──────────────────────────┘                    ║
║                                                                      ║
║  Observatorium     ┌──────────────────────────┐                    ║
║                    │ Gartensternwarte Berlin  │                    ║
║                    └──────────────────────────┘                    ║
║                                                                      ║
║  Breitengrad       ┌──────────┐  Längengrad  ┌──────────┐         ║
║                    │ 52.5200  │              │ 13.4050  │         ║
║                    └──────────┘              └──────────┘         ║
║                                                                      ║
║  Höhe (m)          ┌──────────┐                                    ║
║                    │ 34       │                                    ║
║                    └──────────┘                                    ║
║                                                                      ║
║  ── STATUS ────────────────────────────────────────────────────────  ║
║                                                                      ║
║  ● Plugin aktiv    ☑ Automatisch konvertieren                       ║
║                                                                      ║
║  Letzte Konvertierung:                                              ║
║  ✓ Horsehead_LIGHT_120s_0042.fits  [21:34:12]                      ║
║  ✓ Horsehead_LIGHT_120s_0041.fits  [21:32:08]                      ║
║  ✓ Horsehead_LIGHT_120s_0040.fits  [21:30:04]                      ║
║                                                                      ║
║                    [Test-Konvertierung starten]                     ║
║                                                                      ║
╠══════════════════════════════════════════════════════════════════════╣
║  Community: discord.gg/mvgC6aXY  //  github.com/lindekai/AstroRAW  ║
╚══════════════════════════════════════════════════════════════════════╝
```

---

## Workflow während einer N.I.N.A.-Aufnahmesession

```
N.I.N.A. Sequenz läuft
│
│  [21:34:00] Belichtung 120s startet → Canon 600Da
│  [21:36:00] Belichtung beendet
│  [21:36:01] N.I.N.A. speichert RAW: Horsehead_LIGHT_120s_0042.cr2
│
│  ┌─── AstroRAW-o-Matic Bridge wird getriggert (asynchron) ──────┐
│  │                                                               │
│  │  1. Metadaten aus N.I.N.A.-Session sammeln:                  │
│  │     Object:     "Horsehead Nebula"  (aus Sequence Target)    │
│  │     Filter:     "UV/IR"             (aus Filter Wheel)       │
│  │     Telescope:  "Skywatcher 80ED"   (aus Equipment Profile)  │
│  │     Focuser:    Position 12450      (aus Focuser)            │
│  │     CCD-Temp:   -15.2°C            (aus Camera)             │
│  │     Humidity:   68%                (aus Weather Station)     │
│  │     Mount RA:   83.82°             (aus Mount)              │
│  │                                                               │
│  │  2. session.json erzeugen                                     │
│  │                                                               │
│  │  3. CLI aufrufen:                                             │
│  │     arom.exe convert "..._0042.cr2"                          │
│  │              --output "D:\FITS\Horsehead"                    │
│  │              --metadata "..._0042_session.json"              │
│  │                                                               │
│  │  4. FITS-Header enthält dann:                                 │
│  │     OBJECT   = 'Horsehead Nebula'                            │
│  │     FILTER   = 'UV/IR'                                       │
│  │     TELESCOP = 'Skywatcher 80ED'                             │
│  │     FOCPOS   = 12450                                         │
│  │     CCD-TEMP = -15.2                                         │
│  │     HUMIDITY = 68.0                                          │
│  │     RA       = 83.82                                         │
│  │     BAYERPAT = 'GBRG'                                        │
│  │     ... (62 Keywords gesamt)                                  │
│  │                                                               │
│  └───────────────────────────────────────────────────────────────┘
│
│  [21:36:03] N.I.N.A. Log: "✓ Converted: ..._0042.fits"
│  [21:36:04] Nächste Belichtung 120s startet → N.I.N.A. läuft weiter
│
│  ⚡ Konvertierung lief PARALLEL zur nächsten Belichtung
│     Die Aufnahmesequenz wurde NICHT unterbrochen
```

---

## N.I.N.A. Log-Ausgabe (Imaging-Panel)

```
[21:36:03] [AstroRAW-o-Matic] Konvertierung gestartet: Horsehead_0042.cr2
[21:36:03] [AstroRAW-o-Matic] ✓ Horsehead_0042.fits (36 MB, 1.2s)
[21:36:03] [AstroRAW-o-Matic]   BAYERPAT: GBRG // FOCPOS: 12450 // CCD-TEMP: -15.2°C
[21:38:05] [AstroRAW-o-Matic] Konvertierung gestartet: Horsehead_0043.cr2
[21:38:06] [AstroRAW-o-Matic] ✓ Horsehead_0043.fits (36 MB, 1.1s)
```

---

## Fehlerfall im Log

```
[21:40:07] [AstroRAW-o-Matic] ⚠ Konvertierung fehlgeschlagen: Horsehead_0044.cr2
[21:40:07] [AstroRAW-o-Matic]   Fehler: Output-Ordner nicht schreibbar: D:\FITS\Horsehead
[21:40:07] [AstroRAW-o-Matic]   RAW-Datei wird behalten. Bitte Pfad prüfen.
[21:40:08] [AstroRAW-o-Matic]   Nächste Aufnahme läuft weiter (kein Abbruch der Sequenz)
```

---

## Erzeugte Dateistruktur

```
D:\Aufnahmen\2024-01-09\Horsehead\
├── RAW\
│   ├── Horsehead_LIGHT_120s_0040.cr2
│   ├── Horsehead_LIGHT_120s_0041.cr2
│   └── Horsehead_LIGHT_120s_0042.cr2
└── FITS\
    ├── Horsehead_LIGHT_120s_0040.fits   ← vollständiger Astro-Header
    ├── Horsehead_LIGHT_120s_0041.fits
    └── Horsehead_LIGHT_120s_0042.fits
```

Direkt bereit für Siril / PixInsight / APP — ohne weiteren manuellen Schritt.
