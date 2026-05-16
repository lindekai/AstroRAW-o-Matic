# AstroRAW-o-Matic // N.I.N.A. Bridge Plugin

**Status: Planned // Architecture & Scaffold**

---

## Konzept

Dieses Plugin ist eine **Bridge**, kein Konverter.

Es enthält **keine eigene RAW-Konvertierungslogik**. Die gesamte Konvertierung übernimmt der AstroRAW-o-Matic CLI-Core (Rust). Das Plugin sammelt Metadaten aus der laufenden N.I.N.A.-Session, erzeugt eine `session.json` und ruft die CLI asynchron auf.

```
N.I.N.A. Aufnahmesequenz
        │
        │  neue CR2-Datei erkannt
        ▼
AstroRAW-o-Matic N.I.N.A. Bridge Plugin
  ┌────────────────────────────────────────┐
  │ 1. N.I.N.A.-Metadaten sammeln         │
  │ 2. session.json erzeugen              │
  │ 3. astroraw-o-matic CLI aufrufen      │  ← asynchron, kein Block
  │ 4. Exit-Code + Log auswerten          │
  │ 5. Erfolg/Fehler in N.I.N.A. melden  │
  └────────────────────────────────────────┘
        │
        ▼
   FITS-Datei (neben RAW oder in Zielordner)
```

---

## Architektur-Prinzip

| Komponente | Technologie | Plattform |
|------------|-------------|-----------|
| Core Engine + CLI | Rust | Cross-Platform |
| N.I.N.A. Bridge Plugin | C# / .NET | Windows |
| Kommunikation | CLI-Aufruf + JSON-Datei | - |

**Keine harte Abhängigkeit** des Rust-Core auf N.I.N.A.  
**Keine Konvertierungslogik** im C#-Plugin.

---

## Plugin-Einstellungen (MVP)

| Einstellung | Typ | Beschreibung |
|-------------|-----|-------------|
| `CliPath` | Pfad | Pfad zur `astroraw-o-matic.exe` |
| `OutputFolder` | Pfad | Zielordner für FITS-Dateien |
| `FitsNextToRaw` | Bool | FITS neben RAW speichern |
| `KeepRaw` | Bool | RAW nach Konvertierung behalten |
| `SaveMetadataJson` | Bool | session.json neben FITS speichern |
| `HeaderMode` | Enum | `astro` oder `minimal` |
| `Observer` | String | Name des Beobachters |
| `SiteLatitude` | Double | Standortbreite |
| `SiteLongitude` | Double | Standortlänge |
| `SiteElevation` | Double | Standhöhe [m] |

---

## Metadaten-Mapping N.I.N.A. → session.json

| N.I.N.A. Quelle | session.json Feld |
|-----------------|-------------------|
| `Target.Name` | `object` |
| `CaptureSequence.SequenceType` | `frame_type` |
| `Filter.Name` | `equipment.filter` |
| `ExposureTime` | *(aus EXIF, Fallback)* |
| `Camera.Name` | `equipment.camera.model` |
| `Telescope.Name` | `equipment.telescope` |
| `FocalLength` | `equipment.focal_length` |
| `DateTime.UtcNow` | `session_date` + `session_time` |
| `Sequence.Name` | `notes` |
| Plugin-Einstellung | `observer` |
| Plugin-Einstellung | `location.*` |

---

## CLI-Aufruf

Das Plugin ruft die CLI folgendermaßen auf:

```
astroraw-o-matic.exe convert "<raw-datei>" \
  --output "<zielordner>" \
  --metadata "<session.json>" \
  --overwrite
```

**Asynchron:** Der Aufruf erfolgt in einem separaten Task (`Task.Run`) damit die N.I.N.A.-Aufnahmesequenz nicht blockiert wird.

---

## Fehlerbehandlung

| Situation | Verhalten |
|-----------|-----------|
| CLI nicht gefunden | Fehlermeldung mit Pfad-Hinweis, kein Absturz |
| Konvertierung fehlgeschlagen | RAW wird **nicht** gelöscht, Fehler in N.I.N.A.-Log |
| Metadaten unvollständig | JSON mit verfügbaren Daten, Warnung geloggt |
| FITS-Zielordner nicht schreibbar | Fehler melden, RAW behalten |
| N.I.N.A. wird während Konvertierung beendet | Laufende Konvertierung abwarten (CancellationToken) |

---

## Projektstruktur

```
integrations/nina-plugin/
├── README.md                      # Diese Datei
├── docs/
│   ├── ARCHITECTURE.md            # Detaillierte Architektur
│   └── NINA_INTEGRATION.md        # N.I.N.A.-Plugin-Entwicklung
├── examples/
│   └── nina_session.json          # Beispiel-Metadaten-JSON
└── src/
    ├── AstroRawOMatic.Nina.Plugin.csproj
    ├── Plugin.cs                  # N.I.N.A. IPlugin Implementierung
    ├── BridgeService.cs           # CLI-Aufruf + Async-Logik
    ├── MetadataBuilder.cs         # N.I.N.A. → session.json Mapping
    ├── PluginOptions.cs           # Einstellungen (MVVM)
    └── PluginOptionsView.xaml     # WPF-Einstellungs-UI
```

---

## Abhängigkeiten (geplant)

- N.I.N.A. Plugin API (`NINA.Plugin` NuGet)
- `System.Text.Json` für JSON-Erzeugung
- `System.Diagnostics.Process` für CLI-Aufruf
- Keine externen RAW/FITS-Libraries

---

## Build-Voraussetzungen

- Visual Studio 2022 oder Rider
- .NET 8 SDK
- N.I.N.A. Plugin SDK (wenn verfügbar)
- `astroraw-o-matic.exe` separat installiert

---

## Nicht-Ziele

- Keine eigene RAW-Konvertierung im Plugin
- Kein Debayering
- Kein Stacking, keine Kalibrierung
- Keine Bildbearbeitung
- Keine direkte Änderung des N.I.N.A.-Capture-Prozesses
- Kein Blockieren der Aufnahmesequenz

---

*Teil des AstroRAW-o-Matic Projekts // github.com/lindekai/AstroRAW-o-Matic*
