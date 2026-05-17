# N.I.N.A. Plugin // Build- und Installationsanleitung

## Voraussetzungen

### 1. .NET 8 SDK installieren

```powershell
winget install Microsoft.DotNet.SDK.8
```

Oder manuell: https://dotnet.microsoft.com/download/dotnet/8.0

Prüfen:
```powershell
dotnet --version
# Erwartet: 8.x.x
```

### 2. Visual Studio 2022 (empfohlen)

```powershell
winget install Microsoft.VisualStudio.2022.Community
```

Beim Installieren die Workload **".NET-Desktopentwicklung"** auswählen (enthält WPF-Support).

**Alternative:** VS Code + C# Extension:
```powershell
winget install Microsoft.VisualStudioCode
code --install-extension ms-dotnettools.csharp
```

### 3. N.I.N.A. installieren

Falls noch nicht vorhanden: https://nighttime-imaging.eu/

Standardpfad: `C:\Program Files\N.I.N.A. - Nighttime Imaging 'N' Astronomy\`

### 4. AstroRAW-o-Matic CLI für Windows

Das Plugin ruft die CLI auf — sie muss auf dem Windows-Rechner vorhanden sein.

```powershell
# Rust installieren (falls noch nicht vorhanden)
winget install Rustlang.Rustup

# AstroRAW-o-Matic bauen
git clone https://github.com/lindekai/AstroRAW-o-Matic.git
cd AstroRAW-o-Matic
cargo build --release --bin astroraw-o-matic
# Binary: target\release\astroraw-o-matic.exe
```

---

## Plugin bauen

### Via Visual Studio

1. `integrations/nina-plugin/src/AstroRawOMatic.Nina.Plugin.csproj` öffnen
2. Falls N.I.N.A. nicht in `C:\Program Files\...` liegt, `NinaPath` in der `.csproj` anpassen
3. **Build → Build Solution** (F5 für Debug deployt direkt in N.I.N.A.-Plugins-Ordner)

### Via Kommandozeile

```powershell
cd integrations\nina-plugin\src

# Mit Standard-N.I.N.A.-Pfad
dotnet build -c Debug

# Mit eigenem N.I.N.A.-Pfad
dotnet build -c Debug /p:NinaPath="D:\Programme\NINA\"
```

---

## Plugin installieren (manuell)

Falls Auto-Deploy nicht funktioniert:

```powershell
$dest = "$env:APPDATA\NINA\Plugins\AstroRawOMatic"
New-Item -ItemType Directory -Force -Path $dest
Copy-Item "bin\Debug\net8.0-windows\AstroRawOMatic.Nina.Plugin.dll" $dest
```

N.I.N.A. neu starten → Plugin erscheint unter **Options → Plugins**.

---

## Plugin konfigurieren

Nach dem ersten Start in N.I.N.A.:

1. **Options → Plugins → AstroRAW-o-Matic Bridge**
2. **CLI-Pfad** einstellen: `C:\...\astroraw-o-matic.exe`
3. **Output-Ordner** für FITS wählen (oder "FITS neben RAW" aktivieren)
4. **Observer**, **Standort** eintragen (optional)

---

## Entwicklungsworkflow

```
Änderung in Plugin.cs / BridgeService.cs / MetadataBuilder.cs
        │
        ▼
dotnet build -c Debug
        │ (Auto-Deploy nach %APPDATA%\NINA\Plugins\AstroRawOMatic\)
        ▼
N.I.N.A. neu starten
        │
        ▼
Sequenz starten → CR2 aufnehmen → FITS prüfen
```

---

## Bekannte Einschränkungen (Scaffold-Status)

- `Plugin.cs`: N.I.N.A. `IPlugin`-Interface noch nicht vollständig implementiert (TODO-Kommentare)
- `PluginOptionsView.xaml`: WPF-Einstellungs-UI noch nicht erstellt
- Testen braucht echte N.I.N.A.-Installation + DSLR

---

## Nützliche Ressourcen

- [N.I.N.A. Plugin Template](https://github.com/daleghent/nina-plugin-template)
- [N.I.N.A. Plugin Development Wiki](https://nighttime-imaging.eu/docs/develop/site/plugin_dev/)
- [N.I.N.A. GitHub](https://github.com/isbeorn/nina)
