using System.Diagnostics;

namespace AstroRawOMatic.Nina.Plugin;

/// <summary>
/// Asynchronously invokes the AstroRAW-o-Matic CLI.
/// This is the only class that knows about the CLI binary — no conversion logic lives here.
/// </summary>
public class BridgeService
{
    private readonly PluginOptions _options;
    private readonly MetadataBuilder _metadataBuilder;
    private readonly IPluginLogger _logger;

    public BridgeService(PluginOptions options, IPluginLogger logger)
    {
        _options         = options;
        _metadataBuilder = new MetadataBuilder(options);
        _logger          = logger;
    }

    /// <summary>
    /// Convert a single RAW file asynchronously.
    /// Returns immediately — conversion runs in background.
    /// N.I.N.A. acquisition sequence is never blocked.
    /// </summary>
    public void ConvertInBackground(NinaContext ctx)
    {
        _ = Task.Run(async () =>
        {
            try
            {
                await ConvertAsync(ctx, CancellationToken.None);
            }
            catch (Exception ex)
            {
                _logger.Error($"AstroRAW-o-Matic: Unerwarteter Fehler: {ex.Message}");
            }
        });
    }

    public async Task ConvertAsync(NinaContext ctx, CancellationToken ct)
    {
        if (ctx.RawFilePath is null)
        {
            _logger.Warning("AstroRAW-o-Matic: Kein RAW-Dateipfad übergeben.");
            return;
        }

        if (!File.Exists(_options.CliPath))
        {
            _logger.Error(
                $"AstroRAW-o-Matic CLI nicht gefunden: {_options.CliPath}\n" +
                "Bitte Pfad in Plugin-Einstellungen → AstroRAW-o-Matic → CLI-Pfad prüfen.");
            return;
        }

        // 1. Write session.json
        var jsonPath = Path.ChangeExtension(ctx.RawFilePath, "_session.json");
        try
        {
            var json = _metadataBuilder.ToJson(ctx);
            await File.WriteAllTextAsync(jsonPath, json, ct);
        }
        catch (Exception ex)
        {
            _logger.Warning($"AstroRAW-o-Matic: session.json konnte nicht geschrieben werden: {ex.Message}");
            _logger.Warning("Konvertierung wird ohne Metadaten-Datei fortgesetzt.");
            jsonPath = null;
        }

        // 2. Determine output directory
        var outputDir = _options.FitsNextToRaw
            ? Path.GetDirectoryName(ctx.RawFilePath)!
            : _options.OutputFolder;

        if (!Directory.Exists(outputDir))
        {
            try { Directory.CreateDirectory(outputDir); }
            catch
            {
                _logger.Error($"AstroRAW-o-Matic: Zielordner nicht erreichbar: {outputDir}");
                return;
            }
        }

        // 3. Build CLI arguments
        var args = $"convert \"{ctx.RawFilePath}\" --output \"{outputDir}\" --overwrite";
        if (jsonPath is not null)
            args += $" --metadata \"{jsonPath}\"";

        // 4. Run CLI
        var result = await RunCliAsync(_options.CliPath, args, ct);
        if (result.ExitCode != 0)
        {
            _logger.Error($"AstroRAW-o-Matic: Konvertierung fehlgeschlagen (Exit {result.ExitCode})");
            if (!string.IsNullOrWhiteSpace(result.Stderr))
                _logger.Error(result.Stderr);
            // RAW is NEVER deleted on failure
            return;
        }

        _logger.Info($"AstroRAW-o-Matic: Konvertiert → {Path.GetFileNameWithoutExtension(ctx.RawFilePath)}.fits");

        // 5. Cleanup
        if (!_options.KeepRaw)
        {
            try { File.Delete(ctx.RawFilePath); }
            catch (Exception ex) { _logger.Warning($"RAW konnte nicht gelöscht werden: {ex.Message}"); }
        }

        if (jsonPath is not null && !_options.SaveMetadataJson)
        {
            try { File.Delete(jsonPath); }
            catch { /* non-critical */ }
        }
    }

    private static async Task<CliResult> RunCliAsync(string exe, string args, CancellationToken ct)
    {
        var psi = new ProcessStartInfo(exe, args)
        {
            RedirectStandardOutput = true,
            RedirectStandardError  = true,
            UseShellExecute        = false,
            CreateNoWindow         = true,
        };

        using var process = new Process { StartInfo = psi };
        process.Start();

        var stdout = process.StandardOutput.ReadToEndAsync();
        var stderr = process.StandardError.ReadToEndAsync();

        await process.WaitForExitAsync(ct);

        return new CliResult(process.ExitCode, await stdout, await stderr);
    }
}

public record CliResult(int ExitCode, string Stdout, string Stderr);

/// <summary>Minimal logger interface — implement using N.I.N.A.'s ILogger.</summary>
public interface IPluginLogger
{
    void Info(string message);
    void Warning(string message);
    void Error(string message);
}
