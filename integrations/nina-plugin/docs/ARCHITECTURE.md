# N.I.N.A. Bridge Plugin // Architektur

## Designprinzip

Das Plugin ist eine dünne Bridge. Es enthält **keine eigene Konvertierungslogik**.

```
┌─────────────────────────────────────────────────────────────────┐
│                        N.I.N.A. (Windows)                       │
│  ┌──────────────────────────────────────────────────────────┐   │
│  │           AstroRAW-o-Matic Bridge Plugin (C#)            │   │
│  │                                                          │   │
│  │  PluginOptions    MetadataBuilder    BridgeService        │   │
│  │  (Einstellungen)  (JSON-Erzeugung)   (CLI-Aufruf)        │   │
│  └────────────────────────┬─────────────────────────────────┘   │
└───────────────────────────│─────────────────────────────────────┘
                            │  astroraw-o-matic.exe convert
                            │  --metadata session.json
                            │  --output <zielordner>
                            ▼
         ┌──────────────────────────────────────────┐
         │   AstroRAW-o-Matic CLI (Rust, Windows)   │
         │                                          │
         │   Core Engine:                           │
         │   - RAW lesen (rawler)                   │
         │   - Metadaten auflösen                   │
         │   - FITS schreiben                       │
         └──────────────────────────────────────────┘
                            │
                            ▼
                      FITS-Datei ✅
```

---

## Datenfluss im Detail

### 1. Auslöser

N.I.N.A. ruft nach jeder abgeschlossenen Aufnahme einen Plugin-Hook auf
(z.B. `IImageSaveMediator` oder `AfterExposure` Event).

### 2. MetadataBuilder

```csharp
// Pseudocode
var metadata = new NinaSessionMetadata
{
    Object       = target.Name,
    FrameType    = sequence.SequenceType.ToAromFrameType(),
    Filter       = filter?.Name,
    Telescope    = telescope?.Name,
    FocalLength  = equipment?.FocalLength,
    Aperture     = equipment?.Aperture,
    CameraModel  = camera?.Name,
    SessionDate  = DateTime.UtcNow.ToString("yyyy-MM-dd"),
    SessionTime  = DateTime.UtcNow.ToString("HH:mm:ss"),
    Observer     = pluginOptions.Observer,
    Location     = pluginOptions.ToLocation(),
    Notes        = sequence.Name,
};

string json = metadata.ToSessionJson();
File.WriteAllText(sessionJsonPath, json);
```

### 3. BridgeService (asynchron)

```csharp
// Pseudocode
public async Task ConvertAsync(string rawPath, string sessionJsonPath,
                                CancellationToken ct)
{
    var outputDir = _options.FitsNextToRaw
        ? Path.GetDirectoryName(rawPath)
        : _options.OutputFolder;

    var args = $"convert \"{rawPath}\" " +
               $"--output \"{outputDir}\" " +
               $"--metadata \"{sessionJsonPath}\" " +
               $"--overwrite";

    var result = await RunCliAsync(_options.CliPath, args, ct);

    if (result.ExitCode != 0)
    {
        _logger.LogError("AstroRAW-o-Matic failed: {0}", result.Stderr);
        // RAW wird NICHT gelöscht
        return;
    }

    if (!_options.KeepRaw)
        File.Delete(rawPath);

    if (!_options.SaveMetadataJson)
        File.Delete(sessionJsonPath);

    _logger.LogInfo("Converted: {0}", Path.GetFileName(rawPath));
}
```

### 4. Fehlerbehandlung

```csharp
// Pseudocode
try
{
    await ConvertAsync(rawPath, sessionJson, ct);
}
catch (FileNotFoundException)
{
    ShowError($"astroraw-o-matic.exe nicht gefunden: {_options.CliPath}\n" +
              "Bitte Pfad in Plugin-Einstellungen prüfen.");
}
catch (OperationCanceledException)
{
    _logger.LogWarning("Konvertierung abgebrochen (N.I.N.A. beendet).");
}
catch (Exception ex)
{
    _logger.LogError("Unerwarteter Fehler: {0}", ex.Message);
    // RAW wird in keinem Fall gelöscht
}
```

---

## Asynchrones Design

Die Konvertierung läuft in einem separaten `Task` damit N.I.N.A. **nicht blockiert** wird:

```csharp
// Aufnahme-Hook:
_ = Task.Run(() => ConvertAsync(rawPath, sessionJson, _cts.Token), _cts.Token);
// N.I.N.A. setzt die Sequenz sofort fort
```

Optionale Warteschlange: Für Burst-Aufnahmen kann eine `Channel<ConversionJob>`-Queue
eingesetzt werden, die Konvertierungen serialisiert ohne zu blockieren.

---

## CLI-Schnittstelle (stabil)

Das Plugin verlässt sich ausschließlich auf diese CLI-API:

```
astroraw-o-matic convert <INPUT> --output <DIR> --metadata <JSON> [--overwrite]
```

**Exit Codes:**
- `0` = Erfolg
- `1` = Ein oder mehrere Fehler (Details in stderr)

**Rückwärtskompatibilität:** Die CLI-Schnittstelle ist stabil ab v0.3.0.

---

## N.I.N.A. Plugin API (Hooks)

Relevante N.I.N.A. Interfaces für die Integration:

| Interface | Zweck |
|-----------|-------|
| `IPlugin` | Plugin-Registrierung, Lifecycle |
| `IPluginOptionsAccessor` | Zugriff auf Einstellungen |
| `IImageSaveMediator` | Hook nach jedem Bild-Save |
| `IProfileService` | Equipment-Profil (Teleskop, Kamera) |
| `ITargetController` | Aktuelles Zielobjekt |
| `IFilterWheelMediator` | Aktueller Filter |

---

## Verzeichnisse während der Laufzeit

```
<N.I.N.A. Sequence Output>/
├── Light/
│   ├── Target_001.cr2            ← N.I.N.A. Original
│   ├── Target_001.fits           ← von Plugin erzeugt (wenn FitsNextToRaw=true)
│   └── Target_001_session.json   ← optional (wenn SaveMetadataJson=true)
└── ...
```

Alternativ in konfiguriertem FITS-Zielordner:

```
<CliPath Output>/
├── Target_001.fits
└── Target_001_session.json
```
