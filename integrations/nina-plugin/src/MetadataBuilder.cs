using System.Text.Json;
using System.Text.Json.Serialization;

namespace AstroRawOMatic.Nina.Plugin;

/// <summary>
/// Maps N.I.N.A. session data to AstroRAW-o-Matic session.json.
/// This is the only translation layer — no conversion logic here.
/// </summary>
public class MetadataBuilder
{
    private readonly PluginOptions _options;

    public MetadataBuilder(PluginOptions options) => _options = options;

    public string ToJson(NinaContext ctx)
    {
        var metadata = Build(ctx);
        return JsonSerializer.Serialize(metadata, new JsonSerializerOptions
        {
            WriteIndented = true,
            DefaultIgnoreCondition = JsonIgnoreCondition.WhenWritingNull,
        });
    }

    public SessionMetadata Build(NinaContext ctx)
    {
        var now = ctx.CaptureTimeUtc ?? DateTime.UtcNow;

        return new SessionMetadata
        {
            SchemaVersion = "1.1",

            // ── Observation context ───────────────────────────────────────
            Object       = ctx.TargetName,
            Observer     = NullIfEmpty(_options.Observer),
            Notes        = ctx.SequenceName,
            SessionDate  = now.ToString("yyyy-MM-dd"),
            SessionTime  = now.ToString("HH:mm:ss.fff"),

            // ── Frame type ────────────────────────────────────────────────
            FrameType = MapFrameType(ctx.FrameType),

            // ── Target coordinates ────────────────────────────────────────
            Target = ctx.TargetRaHms is not null || ctx.TargetDecDms is not null
                ? new TargetMetadata
                {
                    Name     = ctx.TargetName,
                    RaHms    = ctx.TargetRaHms,
                    DecDms   = ctx.TargetDecDms,
                    Rotation = ctx.TargetRotation,
                }
                : null,

            // ── Equipment ─────────────────────────────────────────────────
            Equipment = new EquipmentMetadata
            {
                Telescope   = ctx.TelescopeName,
                FocalLength = ctx.FocalLength,
                Aperture    = ctx.Aperture,
                Filter      = ctx.FilterName,

                Camera = new CameraMetadata
                {
                    Model       = ctx.CameraName,
                    CameraId    = ctx.CameraId,
                    PixelSizeX  = ctx.PixelSizeX,
                    PixelSizeY  = ctx.PixelSizeY,
                    Gain        = ctx.Gain,
                    Offset      = ctx.Offset >= 0 ? ctx.Offset : (int?)null,
                    Egain       = ctx.Egain,
                    BinningX    = ctx.BinningX > 1 ? ctx.BinningX : (int?)null,
                    BinningY    = ctx.BinningY > 1 ? ctx.BinningY : (int?)null,
                    SetTemp     = ctx.SetTemp,
                    CcdTemp     = ctx.CcdTemp,
                    ReadoutMode = ctx.ReadoutMode,
                    UsbLimit    = ctx.UsbLimit >= 0 ? ctx.UsbLimit : (int?)null,
                },

                Mount = ctx.MountRaDeg.HasValue ? new MountMetadata
                {
                    RaDeg     = ctx.MountRaDeg,
                    DecDeg    = ctx.MountDecDeg,
                    Altitude  = ctx.Altitude,
                    Azimuth   = ctx.Azimuth,
                    PierSide  = ctx.PierSide,
                    Airmass   = ctx.Airmass,
                } : null,

                Focuser = ctx.FocuserName is not null ? new FocuserMetadata
                {
                    Name        = ctx.FocuserName,
                    Position    = ctx.FocuserPosition,
                    StepSize    = ctx.FocuserStepSize,
                    Temperature = ctx.FocuserTemperature,
                } : null,

                Rotator = ctx.RotatorName is not null ? new RotatorMetadata
                {
                    Name             = ctx.RotatorName,
                    MechanicalAngle  = ctx.RotatorMechanicalAngle,
                    StepSize         = ctx.RotatorStepSize,
                } : null,

                FilterWheel = ctx.FilterWheelName is not null
                    ? new FilterWheelMetadata { Name = ctx.FilterWheelName }
                    : null,
            },

            // ── Location ──────────────────────────────────────────────────
            Location = _options.SiteLatitude.HasValue ? new LocationMetadata
            {
                Latitude        = _options.SiteLatitude,
                Longitude       = _options.SiteLongitude,
                Elevation       = _options.SiteElevation,
                SiteName        = NullIfEmpty(_options.SiteName),
                ObservatoryName = NullIfEmpty(_options.ObservatoryName),
            } : null,

            // ── Weather ───────────────────────────────────────────────────
            Weather = ctx.HasWeather ? new WeatherMetadata
            {
                CloudCover    = ctx.CloudCover,
                DewPoint      = ctx.DewPoint,
                Humidity      = ctx.Humidity,
                Pressure      = ctx.Pressure,
                SkyBrightness = ctx.SkyBrightness,
                Mpsas         = ctx.Mpsas,
                SkyTemp       = ctx.SkyTemp,
                StarFwhm      = ctx.StarFwhm,
                AmbientTemp   = ctx.AmbientTemp,
                WindDirection = ctx.WindDirection,
                WindGust      = ctx.WindGust,
                WindSpeed     = ctx.WindSpeed,
            } : null,

            // ── Output options ────────────────────────────────────────────
            Output = new OutputOptions
            {
                RawMode      = "raw_bayer",
                HeaderMode   = _options.HeaderMode,
                Overwrite    = true,
                WriteHistory = true,
            },
        };
    }

    private static string? MapFrameType(string? t) => t?.ToLowerInvariant() switch
    {
        "light" or "light frame" or "science"  => "light",
        "dark"  or "dark frame"                => "dark",
        "flat"  or "flat frame" or "sky flat"  => "flat",
        "bias"  or "bias frame" or "offset"    => "bias",
        _ => null,
    };

    private static string? NullIfEmpty(string? s) =>
        string.IsNullOrWhiteSpace(s) ? null : s;
}

// ── N.I.N.A. context DTO ─────────────────────────────────────────────────────

public record NinaContext
{
    // Image
    public string? RawFilePath    { get; init; }
    public string? FrameType      { get; init; }
    public DateTime? CaptureTimeUtc { get; init; }
    public string? SequenceName   { get; init; }

    // Target
    public string? TargetName     { get; init; }
    public string? TargetRaHms    { get; init; }
    public string? TargetDecDms   { get; init; }
    public double? TargetRotation { get; init; }

    // Camera
    public string? CameraName     { get; init; }
    public string? CameraId       { get; init; }
    public double? PixelSizeX     { get; init; }
    public double? PixelSizeY     { get; init; }
    public int     Gain           { get; init; } = -1;
    public int     Offset         { get; init; } = -1;
    public double? Egain          { get; init; }
    public int     BinningX       { get; init; } = 1;
    public int     BinningY       { get; init; } = 1;
    public double? SetTemp        { get; init; }
    public double? CcdTemp        { get; init; }
    public string? ReadoutMode    { get; init; }
    public int     UsbLimit       { get; init; } = -1;

    // Telescope / mount
    public string? TelescopeName  { get; init; }
    public double? FocalLength    { get; init; }
    public double? Aperture       { get; init; }
    public double? MountRaDeg     { get; init; }
    public double? MountDecDeg    { get; init; }
    public double? Altitude       { get; init; }
    public double? Azimuth        { get; init; }
    public string? PierSide       { get; init; }
    public double? Airmass        { get; init; }

    // Filter
    public string? FilterName     { get; init; }
    public string? FilterWheelName { get; init; }

    // Focuser
    public string? FocuserName      { get; init; }
    public int?    FocuserPosition  { get; init; }
    public double? FocuserStepSize  { get; init; }
    public double? FocuserTemperature { get; init; }

    // Rotator
    public string? RotatorName           { get; init; }
    public double? RotatorMechanicalAngle { get; init; }
    public double? RotatorStepSize       { get; init; }

    // Weather
    public bool    HasWeather     { get; init; }
    public double? CloudCover     { get; init; }
    public double? DewPoint       { get; init; }
    public double? Humidity       { get; init; }
    public double? Pressure       { get; init; }
    public double? SkyBrightness  { get; init; }
    public double? Mpsas          { get; init; }
    public double? SkyTemp        { get; init; }
    public double? StarFwhm       { get; init; }
    public double? AmbientTemp    { get; init; }
    public double? WindDirection  { get; init; }
    public double? WindGust       { get; init; }
    public double? WindSpeed      { get; init; }
}

// ── Session JSON model (mirrors astroraw-models schema v1.1) ─────────────────

public class SessionMetadata
{
    [JsonPropertyName("schema_version")] public string SchemaVersion { get; set; } = "1.1";
    [JsonPropertyName("object")]         public string? Object       { get; set; }
    [JsonPropertyName("observer")]       public string? Observer     { get; set; }
    [JsonPropertyName("notes")]          public string? Notes        { get; set; }
    [JsonPropertyName("session_date")]   public string? SessionDate  { get; set; }
    [JsonPropertyName("session_time")]   public string? SessionTime  { get; set; }
    [JsonPropertyName("frame_type")]     public string? FrameType    { get; set; }
    [JsonPropertyName("target")]         public TargetMetadata?    Target    { get; set; }
    [JsonPropertyName("equipment")]      public EquipmentMetadata? Equipment { get; set; }
    [JsonPropertyName("location")]       public LocationMetadata?  Location  { get; set; }
    [JsonPropertyName("weather")]        public WeatherMetadata?   Weather   { get; set; }
    [JsonPropertyName("output")]         public OutputOptions?     Output    { get; set; }
}

public class TargetMetadata
{
    [JsonPropertyName("name")]     public string? Name     { get; set; }
    [JsonPropertyName("ra_hms")]   public string? RaHms    { get; set; }
    [JsonPropertyName("dec_dms")]  public string? DecDms   { get; set; }
    [JsonPropertyName("rotation")] public double? Rotation { get; set; }
}

public class EquipmentMetadata
{
    [JsonPropertyName("telescope")]    public string?           Telescope   { get; set; }
    [JsonPropertyName("focal_length")] public double?           FocalLength { get; set; }
    [JsonPropertyName("aperture")]     public double?           Aperture    { get; set; }
    [JsonPropertyName("filter")]       public string?           Filter      { get; set; }
    [JsonPropertyName("camera")]       public CameraMetadata?   Camera      { get; set; }
    [JsonPropertyName("mount")]        public MountMetadata?    Mount       { get; set; }
    [JsonPropertyName("focuser")]      public FocuserMetadata?  Focuser     { get; set; }
    [JsonPropertyName("rotator")]      public RotatorMetadata?  Rotator     { get; set; }
    [JsonPropertyName("filter_wheel")] public FilterWheelMetadata? FilterWheel { get; set; }
}

public class CameraMetadata
{
    [JsonPropertyName("model")]        public string? Model       { get; set; }
    [JsonPropertyName("camera_id")]    public string? CameraId    { get; set; }
    [JsonPropertyName("pixel_size_x")] public double? PixelSizeX  { get; set; }
    [JsonPropertyName("pixel_size_y")] public double? PixelSizeY  { get; set; }
    [JsonPropertyName("gain")]         public int?    Gain        { get; set; }
    [JsonPropertyName("offset")]       public int?    Offset      { get; set; }
    [JsonPropertyName("egain")]        public double? Egain       { get; set; }
    [JsonPropertyName("binning_x")]    public int?    BinningX    { get; set; }
    [JsonPropertyName("binning_y")]    public int?    BinningY    { get; set; }
    [JsonPropertyName("set_temp")]     public double? SetTemp     { get; set; }
    [JsonPropertyName("ccd_temp")]     public double? CcdTemp     { get; set; }
    [JsonPropertyName("readout_mode")] public string? ReadoutMode { get; set; }
    [JsonPropertyName("usb_limit")]    public int?    UsbLimit    { get; set; }
}

public class MountMetadata
{
    [JsonPropertyName("ra_deg")]    public double? RaDeg    { get; set; }
    [JsonPropertyName("dec_deg")]   public double? DecDeg   { get; set; }
    [JsonPropertyName("altitude")]  public double? Altitude { get; set; }
    [JsonPropertyName("azimuth")]   public double? Azimuth  { get; set; }
    [JsonPropertyName("pier_side")] public string? PierSide { get; set; }
    [JsonPropertyName("airmass")]   public double? Airmass  { get; set; }
}

public class FocuserMetadata
{
    [JsonPropertyName("name")]        public string? Name        { get; set; }
    [JsonPropertyName("position")]    public int?    Position    { get; set; }
    [JsonPropertyName("step_size")]   public double? StepSize    { get; set; }
    [JsonPropertyName("temperature")] public double? Temperature { get; set; }
}

public class RotatorMetadata
{
    [JsonPropertyName("name")]              public string? Name            { get; set; }
    [JsonPropertyName("mechanical_angle")]  public double? MechanicalAngle { get; set; }
    [JsonPropertyName("step_size")]         public double? StepSize        { get; set; }
}

public class FilterWheelMetadata
{
    [JsonPropertyName("name")] public string? Name { get; set; }
}

public class LocationMetadata
{
    [JsonPropertyName("latitude")]         public double? Latitude        { get; set; }
    [JsonPropertyName("longitude")]        public double? Longitude       { get; set; }
    [JsonPropertyName("elevation")]        public double? Elevation       { get; set; }
    [JsonPropertyName("site_name")]        public string? SiteName        { get; set; }
    [JsonPropertyName("observatory_name")] public string? ObservatoryName { get; set; }
}

public class WeatherMetadata
{
    [JsonPropertyName("cloud_cover")]    public double? CloudCover    { get; set; }
    [JsonPropertyName("dew_point")]      public double? DewPoint      { get; set; }
    [JsonPropertyName("humidity")]       public double? Humidity      { get; set; }
    [JsonPropertyName("pressure")]       public double? Pressure      { get; set; }
    [JsonPropertyName("sky_brightness")] public double? SkyBrightness { get; set; }
    [JsonPropertyName("mpsas")]          public double? Mpsas         { get; set; }
    [JsonPropertyName("sky_temp")]       public double? SkyTemp       { get; set; }
    [JsonPropertyName("star_fwhm")]      public double? StarFwhm      { get; set; }
    [JsonPropertyName("ambient_temp")]   public double? AmbientTemp   { get; set; }
    [JsonPropertyName("wind_direction")] public double? WindDirection { get; set; }
    [JsonPropertyName("wind_gust")]      public double? WindGust      { get; set; }
    [JsonPropertyName("wind_speed")]     public double? WindSpeed     { get; set; }
}

public class OutputOptions
{
    [JsonPropertyName("raw_mode")]      public string RawMode      { get; set; } = "raw_bayer";
    [JsonPropertyName("header_mode")]   public string HeaderMode   { get; set; } = "astro";
    [JsonPropertyName("overwrite")]     public bool   Overwrite    { get; set; } = true;
    [JsonPropertyName("write_history")] public bool   WriteHistory { get; set; } = true;
}
