using System.Text.Json;
using System.Text.Json.Serialization;

namespace AstroRawOMatic.Nina.Plugin;

/// <summary>
/// Builds an AstroRAW-o-Matic session.json from N.I.N.A. session data.
/// This is the only place where N.I.N.A. data is mapped to the CLI contract.
/// </summary>
public class MetadataBuilder
{
    private readonly PluginOptions _options;

    public MetadataBuilder(PluginOptions options)
    {
        _options = options;
    }

    /// <summary>
    /// Build a SessionMetadata object from N.I.N.A. context.
    /// All fields are optional — missing values are omitted, not defaulted to wrong values.
    /// </summary>
    public SessionMetadata Build(NinaContext ctx)
    {
        var now = DateTime.UtcNow;

        return new SessionMetadata
        {
            SchemaVersion = "1.0",
            Object        = ctx.TargetName,
            Observer      = string.IsNullOrWhiteSpace(_options.Observer) ? null : _options.Observer,
            Notes         = ctx.SequenceName,
            SessionDate   = ctx.CaptureTimeUtc?.ToString("yyyy-MM-dd") ?? now.ToString("yyyy-MM-dd"),
            SessionTime   = ctx.CaptureTimeUtc?.ToString("HH:mm:ss") ?? now.ToString("HH:mm:ss"),
            FrameType     = MapFrameType(ctx.FrameType),
            Equipment     = BuildEquipment(ctx),
            Location      = BuildLocation(),
            Output        = new OutputOptions
            {
                RawMode       = "raw_bayer",
                HeaderMode    = _options.HeaderMode,
                Overwrite     = true,
                WriteHistory  = true,
            },
        };
    }

    public string ToJson(NinaContext ctx)
    {
        var metadata = Build(ctx);
        return JsonSerializer.Serialize(metadata, new JsonSerializerOptions
        {
            WriteIndented        = true,
            DefaultIgnoreCondition = JsonIgnoreCondition.WhenWritingNull,
        });
    }

    private static string? MapFrameType(string? ninaType) => ninaType?.ToLowerInvariant() switch
    {
        "light" or "light frame"  => "light",
        "dark"  or "dark frame"   => "dark",
        "flat"  or "flat frame"   => "flat",
        "bias"  or "bias frame"   => "bias",
        _ => null,
    };

    private EquipmentMetadata? BuildEquipment(NinaContext ctx)
    {
        if (ctx.CameraName is null && ctx.TelescopeName is null &&
            ctx.FocalLength is null && ctx.FilterName is null)
            return null;

        return new EquipmentMetadata
        {
            Telescope   = ctx.TelescopeName,
            FocalLength = ctx.FocalLength,
            Aperture    = ctx.Aperture,
            Filter      = ctx.FilterName,
            Camera      = ctx.CameraName is not null
                ? new CameraMetadata { Model = ctx.CameraName }
                : null,
        };
    }

    private LocationMetadata? BuildLocation()
    {
        if (_options.SiteLatitude is null && _options.SiteLongitude is null)
            return null;

        return new LocationMetadata
        {
            Latitude  = _options.SiteLatitude,
            Longitude = _options.SiteLongitude,
            Elevation = _options.SiteElevation,
            SiteName  = string.IsNullOrWhiteSpace(_options.SiteName) ? null : _options.SiteName,
        };
    }
}

// ── N.I.N.A. context DTO ─────────────────────────────────────────────────────

/// <summary>
/// Data passed from N.I.N.A. plugin hooks into the MetadataBuilder.
/// All fields are nullable — the plugin never crashes on missing data.
/// </summary>
public record NinaContext
{
    public string? TargetName     { get; init; }
    public string? FrameType      { get; init; }
    public string? FilterName     { get; init; }
    public string? CameraName     { get; init; }
    public string? TelescopeName  { get; init; }
    public double? FocalLength    { get; init; }
    public double? Aperture       { get; init; }
    public string? SequenceName   { get; init; }
    public DateTime? CaptureTimeUtc { get; init; }
    public string? RawFilePath    { get; init; }
}

// ── Session JSON model (mirrors astroraw-models) ──────────────────────────────

public class SessionMetadata
{
    [JsonPropertyName("schema_version")] public string SchemaVersion { get; set; } = "1.0";
    [JsonPropertyName("object")]         public string? Object        { get; set; }
    [JsonPropertyName("observer")]       public string? Observer      { get; set; }
    [JsonPropertyName("notes")]          public string? Notes         { get; set; }
    [JsonPropertyName("session_date")]   public string? SessionDate   { get; set; }
    [JsonPropertyName("session_time")]   public string? SessionTime   { get; set; }
    [JsonPropertyName("frame_type")]     public string? FrameType     { get; set; }
    [JsonPropertyName("equipment")]      public EquipmentMetadata? Equipment { get; set; }
    [JsonPropertyName("location")]       public LocationMetadata?  Location  { get; set; }
    [JsonPropertyName("output")]         public OutputOptions?     Output    { get; set; }
}

public class EquipmentMetadata
{
    [JsonPropertyName("telescope")]    public string? Telescope   { get; set; }
    [JsonPropertyName("focal_length")] public double? FocalLength { get; set; }
    [JsonPropertyName("aperture")]     public double? Aperture    { get; set; }
    [JsonPropertyName("filter")]       public string? Filter      { get; set; }
    [JsonPropertyName("camera")]       public CameraMetadata? Camera { get; set; }
}

public class CameraMetadata
{
    [JsonPropertyName("model")] public string? Model { get; set; }
}

public class LocationMetadata
{
    [JsonPropertyName("latitude")]   public double? Latitude  { get; set; }
    [JsonPropertyName("longitude")]  public double? Longitude { get; set; }
    [JsonPropertyName("elevation")]  public double? Elevation { get; set; }
    [JsonPropertyName("site_name")]  public string? SiteName  { get; set; }
}

public class OutputOptions
{
    [JsonPropertyName("raw_mode")]      public string RawMode      { get; set; } = "raw_bayer";
    [JsonPropertyName("header_mode")]   public string HeaderMode   { get; set; } = "astro";
    [JsonPropertyName("overwrite")]     public bool   Overwrite    { get; set; } = true;
    [JsonPropertyName("write_history")] public bool   WriteHistory { get; set; } = true;
}
