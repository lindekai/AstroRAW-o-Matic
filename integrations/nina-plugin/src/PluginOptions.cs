using System.ComponentModel;
using System.Runtime.CompilerServices;

namespace AstroRawOMatic.Nina.Plugin;

/// <summary>
/// User-configurable settings for the AstroRAW-o-Matic N.I.N.A. Bridge Plugin.
/// Persisted by N.I.N.A.'s settings system.
/// </summary>
public class PluginOptions : INotifyPropertyChanged
{
    private string _cliPath = @"C:\Program Files\AstroRAW-o-Matic\astroraw-o-matic.exe";
    private string _outputFolder = string.Empty;
    private bool _fitsNextToRaw = true;
    private bool _keepRaw = true;
    private bool _saveMetadataJson = false;
    private string _headerMode = "astro";
    private string _observer = string.Empty;
    private double? _siteLatitude;
    private double? _siteLongitude;
    private double? _siteElevation;
    private string _siteName = string.Empty;

    /// <summary>Full path to astroraw-o-matic.exe</summary>
    public string CliPath
    {
        get => _cliPath;
        set { _cliPath = value; OnPropertyChanged(); }
    }

    /// <summary>FITS output folder. Used when FitsNextToRaw is false.</summary>
    public string OutputFolder
    {
        get => _outputFolder;
        set { _outputFolder = value; OnPropertyChanged(); }
    }

    /// <summary>Write FITS file next to the original RAW file.</summary>
    public bool FitsNextToRaw
    {
        get => _fitsNextToRaw;
        set { _fitsNextToRaw = value; OnPropertyChanged(); }
    }

    /// <summary>Keep the original RAW file after successful conversion.</summary>
    public bool KeepRaw
    {
        get => _keepRaw;
        set { _keepRaw = value; OnPropertyChanged(); }
    }

    /// <summary>Save the generated session.json next to the FITS file.</summary>
    public bool SaveMetadataJson
    {
        get => _saveMetadataJson;
        set { _saveMetadataJson = value; OnPropertyChanged(); }
    }

    /// <summary>"astro" or "minimal" FITS header mode.</summary>
    public string HeaderMode
    {
        get => _headerMode;
        set { _headerMode = value; OnPropertyChanged(); }
    }

    /// <summary>Observer name written to FITS OBSERVER keyword.</summary>
    public string Observer
    {
        get => _observer;
        set { _observer = value; OnPropertyChanged(); }
    }

    public double? SiteLatitude
    {
        get => _siteLatitude;
        set { _siteLatitude = value; OnPropertyChanged(); }
    }

    public double? SiteLongitude
    {
        get => _siteLongitude;
        set { _siteLongitude = value; OnPropertyChanged(); }
    }

    public double? SiteElevation
    {
        get => _siteElevation;
        set { _siteElevation = value; OnPropertyChanged(); }
    }

    public string SiteName
    {
        get => _siteName;
        set { _siteName = value; OnPropertyChanged(); }
    }

    public event PropertyChangedEventHandler? PropertyChanged;

    protected void OnPropertyChanged([CallerMemberName] string? name = null)
        => PropertyChanged?.Invoke(this, new PropertyChangedEventArgs(name));
}
