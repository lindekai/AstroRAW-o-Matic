namespace AstroRawOMatic.Nina.Plugin;

/// <summary>
/// AstroRAW-o-Matic N.I.N.A. Bridge Plugin entry point.
///
/// Implementation note: This scaffold shows the intended structure.
/// The actual N.I.N.A. IPlugin interface and hook registration depends
/// on the N.I.N.A. Plugin SDK version — adapt accordingly.
///
/// Key principle: NO conversion logic here. This class only:
///   1. Registers with N.I.N.A.
///   2. Listens for image-saved events
///   3. Delegates to BridgeService
/// </summary>
public class AstroRawOMaticPlugin /* : IPlugin */
{
    public static readonly string PluginId      = "de.ir-media-ad.astroraw-o-matic-nina";
    public static readonly string PluginName    = "AstroRAW-o-Matic Bridge";
    public static readonly string PluginVersion = "0.1.0";
    public static readonly string Author        = "Kai Linde";
    public static readonly string Homepage      = "https://github.com/lindekai/AstroRAW-o-Matic";

    private readonly PluginOptions _options;
    private readonly BridgeService _bridge;

    public AstroRawOMatic.Nina.Plugin.AstroRawOMaticPlugin()
    {
        _options = LoadOptions();
        _bridge  = new BridgeService(_options, new NinaPluginLogger());
    }

    /// <summary>
    /// Called by N.I.N.A. after each image is saved to disk.
    /// Adapt signature to match actual N.I.N.A. event args.
    /// </summary>
    public void OnImageSaved(string rawFilePath, string targetName, string frameType,
                              string? filterName, string? cameraName, string? telescopeName,
                              double? focalLength, double? aperture, string? sequenceName,
                              DateTime captureTimeUtc)
    {
        // Only handle RAW files
        var ext = Path.GetExtension(rawFilePath).ToUpperInvariant();
        if (ext is not (".CR2" or ".CR3" or ".NEF" or ".ARW"))
            return;

        var ctx = new NinaContext
        {
            RawFilePath    = rawFilePath,
            TargetName     = targetName,
            FrameType      = frameType,
            FilterName     = filterName,
            CameraName     = cameraName,
            TelescopeName  = telescopeName,
            FocalLength    = focalLength,
            Aperture       = aperture,
            SequenceName   = sequenceName,
            CaptureTimeUtc = captureTimeUtc,
        };

        // Fire and forget — N.I.N.A. sequence continues immediately
        _bridge.ConvertInBackground(ctx);
    }

    private static PluginOptions LoadOptions()
    {
        // TODO: Load from N.I.N.A. settings persistence
        return new PluginOptions();
    }
}

/// <summary>Adapter for N.I.N.A.'s logging system.</summary>
internal class NinaPluginLogger : IPluginLogger
{
    // Replace Console with N.I.N.A.'s actual ILogger/Logger.Instance
    public void Info(string message)    => Console.WriteLine($"[INFO]  {message}");
    public void Warning(string message) => Console.WriteLine($"[WARN]  {message}");
    public void Error(string message)   => Console.Error.WriteLine($"[ERROR] {message}");
}
