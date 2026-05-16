<script>
  import { openUrl } from "@tauri-apps/plugin-opener";
  export let open = false;
  function close() { open = false; }

  let activeTab = "workflow";
  let lang = "de";
  const version = "0.3.0";
  const discordUrl = "https://discord.gg/mvgC6aXY";
  const githubUrl = "https://github.com/lindekai/AstroRAW-o-Matic";

  const t = {
    de: {
      title: "Hilfe",
      tabs: ["Workflow", "Session-Felder", "JSON-Muster", "FITS-Header", "CLI", "About"],
      close: "Schließen",
      workflow: {
        h: "Empfohlener Workflow",
        steps: [
          "CR2-Dateien per Drag & Drop oder <strong>+ Add Files</strong> laden",
          "Session-Formular ausfüllen (Objekt, Teleskop, Kamera …)",
          "Session als JSON speichern: <strong>💾 Save JSON</strong>",
          "Output-Ordner wählen: <strong>Output Folder</strong>",
          "<strong>Convert to FITS</strong> klicken",
          "FITS-Dateien in Siril öffnen → Debayern → Stacken",
        ],
        note: "Das Bayer-Pattern wird automatisch aus den RAW-Daten ermittelt. Für Canon 600D / 600Da ist das korrekte Muster <strong>GBRG</strong>.",
      },
      session: {
        h: "Session-Felder erklärt",
        rows: [
          ["Object", "Zielobjekt (z.B. M31, NGC 7000, Horsehead Nebula)"],
          ["Observer", "Name des Beobachters // erscheint im FITS-Header als OBSERVER"],
          ["Session Date", "Beobachtungsdatum // wird aus EXIF vorausgefüllt"],
          ["Session Time", "Beobachtungszeit (UTC) // wird aus EXIF vorausgefüllt"],
          ["Frame Type", "Light / Dark / Flat / Bias → IMAGETYP im FITS-Header"],
          ["Filter", "Verwendeter Filter (UV/IR, DNB, Halpha, OIII, SII, L, R, G, B)"],
          ["Camera Model", "Kameramodell → INSTRUME im FITS-Header"],
          ["Pixel X/Y (µm)", "Pixelgröße des Sensors (Canon 600D: 4.3 µm)"],
          ["Telescope", "Teleskop oder Objektiv → TELESCOP im FITS-Header"],
          ["Focal Length", "Brennweite in mm → FOCALLEN im FITS-Header"],
          ["Aperture", "Öffnung in mm → APTDIA + FOCRATIO im FITS-Header"],
          ["Focal Ratio", "Automatisch berechnet: Brennweite ÷ Öffnung"],
          ["Site Name", "Name des Beobachtungsorts"],
          ["Lat / Lon", "Koordinaten → SITELAT / SITELONG im FITS-Header"],
          ["Elevation", "Höhe über NN in Metern → SITEELEV im FITS-Header"],
        ],
      },
      json: {
        h: "JSON Dateinamen-Muster",
        intro: "Das Feld <strong>JSON Filename Pattern</strong> im OUTPUT-Bereich steuert den Vorschlagsnamen beim Speichern.",
        rows: [
          ["{object}", "Objektname (Leerzeichen → Unterstriche)"],
          ["{date}", "Aktuelles Datum (YYYY-MM-DD)"],
          ["{observer}", "Name des Beobachters"],
        ],
        exLabel: "Beispiele:",
      },
      fits: {
        h: "Wichtige FITS-Keywords",
        th: ["Keyword", "Inhalt", "Quelle"],
        note: "<strong>Metadaten-Priorität:</strong><br/>file_overrides > Session-JSON > EXIF aus RAW-Datei > Standardwerte",
      },
      cli: {
        h: "CLI-Befehle",
        alias: "Kurzname: <code>arom</code> (Alias anlegen mit <code>alias arom='astroraw-o-matic'</code>)",
      },
      about: {
        author: "Autor",
        community: "Community",
        github: "GitHub",
        license: "Lizenz",
        quote: "„Die Antwort ist 42. Das Bayer-Muster ist GBRG."",
      },
    },
    en: {
      title: "Help",
      tabs: ["Workflow", "Session Fields", "JSON Pattern", "FITS Header", "CLI", "About"],
      close: "Close",
      workflow: {
        h: "Recommended Workflow",
        steps: [
          "Load CR2 files via Drag & Drop or <strong>+ Add Files</strong>",
          "Fill in the session form (object, telescope, camera …)",
          "Save session as JSON: <strong>💾 Save JSON</strong>",
          "Choose output folder: <strong>Output Folder</strong>",
          "Click <strong>Convert to FITS</strong>",
          "Open FITS files in Siril → Debayer → Stack",
        ],
        note: "The Bayer pattern is determined automatically from the RAW data. For Canon 600D / 600Da the correct pattern is <strong>GBRG</strong>.",
      },
      session: {
        h: "Session Fields Explained",
        rows: [
          ["Object", "Target object (e.g. M31, NGC 7000, Horsehead Nebula)"],
          ["Observer", "Observer name // written to FITS header as OBSERVER"],
          ["Session Date", "Observation date // pre-filled from EXIF"],
          ["Session Time", "Observation time (UTC) // pre-filled from EXIF"],
          ["Frame Type", "Light / Dark / Flat / Bias → IMAGETYP in FITS header"],
          ["Filter", "Filter used (UV/IR, DNB, Halpha, OIII, SII, L, R, G, B)"],
          ["Camera Model", "Camera model → INSTRUME in FITS header"],
          ["Pixel X/Y (µm)", "Sensor pixel size (Canon 600D: 4.3 µm)"],
          ["Telescope", "Telescope or lens → TELESCOP in FITS header"],
          ["Focal Length", "Focal length in mm → FOCALLEN in FITS header"],
          ["Aperture", "Aperture in mm → APTDIA + FOCRATIO in FITS header"],
          ["Focal Ratio", "Calculated automatically: focal length ÷ aperture"],
          ["Site Name", "Observation site name"],
          ["Lat / Lon", "Coordinates → SITELAT / SITELONG in FITS header"],
          ["Elevation", "Elevation above sea level in metres → SITEELEV in FITS header"],
        ],
      },
      json: {
        h: "JSON Filename Pattern",
        intro: "The <strong>JSON Filename Pattern</strong> field in the OUTPUT section controls the suggested filename when saving.",
        rows: [
          ["{object}", "Object name (spaces → underscores)"],
          ["{date}", "Current date (YYYY-MM-DD)"],
          ["{observer}", "Observer name"],
        ],
        exLabel: "Examples:",
      },
      fits: {
        h: "Important FITS Keywords",
        th: ["Keyword", "Content", "Source"],
        note: "<strong>Metadata priority:</strong><br/>file_overrides > Session JSON > EXIF from RAW file > Defaults",
      },
      cli: {
        h: "CLI Commands",
        alias: "Short name: <code>arom</code> (set alias with <code>alias arom='astroraw-o-matic'</code>)",
      },
      about: {
        author: "Author",
        community: "Community",
        github: "GitHub",
        license: "License",
        quote: '"The answer is 42. The Bayer pattern is GBRG."',
      },
    },
  };

  const fitsRows = [
    ["BAYERPAT", "GBRG (Canon 600D)", "rawler"],
    ["PEDESTAL", "Black level ADU (~2046)", "rawler"],
    ["BLACKLVL", "Black level ADU", "rawler"],
    ["WHITELEV", "Saturation level ADU", "rawler"],
    ["EXPTIME", "Exposure time [s]", "EXIF"],
    ["ISOSPEED", "ISO setting", "EXIF"],
    ["DATE-OBS", "Observation date/time (UTC)", "EXIF / Session"],
    ["FOCRATIO", "Focal ratio (f/number)", "Session (calculated)"],
    ["CBLACK_R/G/B", "WB coefficients", "rawler"],
    ["IMAGETYP", "LIGHT / DARK / FLAT / BIAS", "Session / CLI"],
  ];

  $: tx = t[lang];
  $: tabIds = ["workflow", "session", "json", "fits", "cli", "about"];
</script>

{#if open}
<div class="overlay" on:click={close} role="button" tabindex="0"
     on:keydown={(e) => e.key === 'Escape' && close()}>
  <div class="dialog" on:click|stopPropagation role="document">

    <div class="dialog-header">
      <span class="title">AstroRAW-o-Matic // {tx.title}</span>
      <div class="header-right">
        <div class="lang-toggle">
          <button class:active={lang === "de"} on:click={() => lang = "de"}>DE</button>
          <button class:active={lang === "en"} on:click={() => lang = "en"}>EN</button>
        </div>
        <button class="close-btn" on:click={close}>✕</button>
      </div>
    </div>

    <div class="tabs">
      {#each tabIds as id, i}
        <button class="tab" class:active={activeTab === id} on:click={() => activeTab = id}>
          {tx.tabs[i]}
        </button>
      {/each}
    </div>

    <div class="content">

      {#if activeTab === "workflow"}
        <h3>{tx.workflow.h}</h3>
        <ol>
          {#each tx.workflow.steps as step}
            <li>{@html step}</li>
          {/each}
        </ol>
        <div class="note">{@html tx.workflow.note}</div>

      {:else if activeTab === "session"}
        <h3>{tx.session.h}</h3>
        <table>
          <tbody>
          {#each tx.session.rows as [field, desc]}
            <tr><td>{field}</td><td>{desc}</td></tr>
          {/each}
          </tbody>
        </table>

      {:else if activeTab === "json"}
        <h3>{tx.json.h}</h3>
        <p>{@html tx.json.intro}</p>
        <table><tbody>
          {#each tx.json.rows as [ph, desc]}
            <tr><td><code>{ph}</code></td><td>{desc}</td></tr>
          {/each}
        </tbody></table>
        <div class="example">
          <strong>{tx.json.exLabel}</strong><br/>
          <code>{"session_{object}_{date}"}</code> → <em>session_Horsehead_Nebula_2026-05-17.json</em><br/>
          <code>{"{observer}_{object}"}</code> → <em>Kai_Linde_M31.json</em>
        </div>

      {:else if activeTab === "fits"}
        <h3>{tx.fits.h}</h3>
        <table>
          <thead><tr>{#each tx.fits.th as h}<th>{h}</th>{/each}</tr></thead>
          <tbody>
          {#each fitsRows as [kw, content, src]}
            <tr><td>{kw}</td><td>{content}</td><td>{src}</td></tr>
          {/each}
          </tbody>
        </table>
        <div class="note">{@html tx.fits.note}</div>

      {:else if activeTab === "cli"}
        <h3>{tx.cli.h}</h3>
        <pre>
# {lang === "de" ? "Metadaten anzeigen" : "Show metadata"}
astroraw-o-matic inspect image.CR2

# {lang === "de" ? "Einzeldatei konvertieren" : "Convert single file"}
astroraw-o-matic convert image.CR2 \
  --output ./FITS \
  --object "M31" --type light

# {lang === "de" ? "Verzeichnis mit Session-JSON" : "Directory with session JSON"}
astroraw-o-matic convert ./RAW \
  --output ./FITS \
  --metadata session.json

# {lang === "de" ? "Dry-Run" : "Dry run"}
astroraw-o-matic convert ./RAW \
  --output ./FITS --dry-run

# {lang === "de" ? "Session-JSON validieren" : "Validate session JSON"}
astroraw-o-matic validate session.json</pre>
        <div class="note">{@html tx.cli.alias}</div>

      {:else if activeTab === "about"}
        <div class="about">
          <div class="about-logo">🔭</div>
          <h2>AstroRAW-o-Matic</h2>
          <div class="about-version">Version {version}</div>
          <div class="about-tagline">Mostly harmless RAW conversion</div>
          <div class="about-section">
            <strong>{tx.about.author}</strong>
            <p>Kai Linde</p>
          </div>
          <div class="about-section">
            <strong>{tx.about.community}</strong>
            <p><button class="link-btn" on:click={() => openUrl(discordUrl)}>🔭 Dark Matters // Astro Community</button></p>
          </div>
          <div class="about-section">
            <strong>{tx.about.github}</strong>
            <p><button class="link-btn" on:click={() => openUrl(githubUrl)}>github.com/lindekai/AstroRAW-o-Matic</button></p>
          </div>
          <div class="about-section">
            <strong>{tx.about.license}</strong>
            <p>MIT License // © 2026 Kai Linde</p>
          </div>
          <div class="about-quote">{tx.about.quote}</div>
        </div>
      {/if}

    </div>

    <div class="dialog-footer">
      <span class="footer-left">AstroRAW-o-Matic v{version} // Mostly harmless RAW conversion</span>
      <button class="footer-community" on:click={() => openUrl(discordUrl)}>🔭 Dark Matters Community</button>
      <span class="footer-right">Author: Kai Linde</span>
      <button class="close-btn-bottom" on:click={close}>{tx.close}</button>
    </div>

  </div>
</div>
{/if}

<style>
  .overlay {
    position: fixed; inset: 0; background: rgba(0,0,0,0.6);
    display: flex; align-items: center; justify-content: center; z-index: 100;
  }
  .dialog {
    background: #1a1f2e; border: 1px solid #2d3748; border-radius: 8px;
    width: 720px; max-height: 80vh; display: flex; flex-direction: column;
    box-shadow: 0 8px 32px rgba(0,0,0,0.5);
  }
  .dialog-header {
    display: flex; justify-content: space-between; align-items: center;
    padding: 10px 16px; border-bottom: 1px solid #2d3748; flex-shrink: 0;
  }
  .header-right { display: flex; align-items: center; gap: 10px; }
  .title { font-size: 14px; font-weight: 600; color: #63b3ed; }

  .lang-toggle { display: flex; gap: 2px; }
  .lang-toggle button {
    background: #2d3748; border: 1px solid #4a5568; color: #718096;
    border-radius: 3px; padding: 2px 8px; font-size: 11px; cursor: pointer;
    font-weight: 600;
  }
  .lang-toggle button.active { background: #2b6cb0; color: white; border-color: #2b6cb0; }
  .lang-toggle button:hover:not(.active) { background: #3d4f66; color: #e2e8f0; }

  .close-btn { background: none; border: none; color: #718096; cursor: pointer; font-size: 14px; padding: 2px 6px; }
  .close-btn:hover { color: #e2e8f0; }

  .tabs { display: flex; border-bottom: 1px solid #2d3748; flex-shrink: 0; }
  .tab {
    background: none; border: none; color: #718096; padding: 8px 12px;
    font-size: 12px; cursor: pointer; border-bottom: 2px solid transparent; margin-bottom: -1px;
  }
  .tab:hover { color: #e2e8f0; }
  .tab.active { color: #63b3ed; border-bottom-color: #63b3ed; }

  .content { padding: 16px; overflow-y: auto; flex: 1; font-size: 12px; line-height: 1.6; color: #a0aec0; }

  h3 { color: #e2e8f0; margin-bottom: 10px; font-size: 13px; }
  ol { padding-left: 18px; display: flex; flex-direction: column; gap: 5px; }
  li { color: #a0aec0; }
  p { margin: 6px 0; }
  code { background: #0f1117; padding: 1px 5px; border-radius: 3px; color: #63b3ed; font-family: monospace; }
  pre { background: #0f1117; padding: 10px; border-radius: 5px; font-size: 11px; color: #68d391; line-height: 1.7; overflow-x: auto; }

  table { width: 100%; border-collapse: collapse; margin-top: 8px; }
  thead th { text-align: left; color: #63b3ed; padding: 5px 8px; border-bottom: 1px solid #2d3748; font-size: 11px; }
  td { padding: 5px 8px; border-bottom: 1px solid #1a202c; vertical-align: top; }
  td:first-child { color: #e2e8f0; white-space: nowrap; font-family: monospace; width: 140px; }

  .note { margin-top: 12px; padding: 8px 10px; background: #0f1117; border-left: 3px solid #63b3ed; border-radius: 3px; font-size: 11px; color: #718096; }
  .example { margin-top: 12px; padding: 8px 10px; background: #0f1117; border-radius: 3px; font-size: 11px; line-height: 1.8; }
  em { color: #68d391; font-style: normal; }

  .about { text-align: center; padding: 16px 0; display: flex; flex-direction: column; align-items: center; gap: 12px; }
  .about-logo { font-size: 48px; }
  .about h2 { font-size: 20px; font-weight: 700; color: #63b3ed; margin: 0; }
  .about-version { font-size: 13px; color: #718096; }
  .about-tagline { font-size: 12px; color: #4a5568; font-style: italic; }
  .about-section { text-align: center; }
  .about-section strong { font-size: 11px; color: #63b3ed; text-transform: uppercase; letter-spacing: 0.8px; display: block; margin-bottom: 3px; }
  .about-section p { font-size: 13px; color: #a0aec0; margin: 0; }
  .link-btn { background: none; border: none; color: #63b3ed; cursor: pointer; font-size: 13px; padding: 0; text-decoration: underline; text-underline-offset: 2px; }
  .link-btn:hover { color: #90cdf4; }
  .about-quote { font-size: 11px; color: #4a5568; font-style: italic; margin-top: 8px; border-top: 1px solid #2d3748; padding-top: 12px; width: 100%; text-align: center; }

  .dialog-footer { display: flex; align-items: center; gap: 12px; padding: 8px 16px; border-top: 1px solid #2d3748; flex-shrink: 0; }
  .footer-left { font-size: 11px; color: #4a5568; font-style: italic; flex: 1; white-space: nowrap; overflow: hidden; text-overflow: ellipsis; }
  .footer-right { font-size: 11px; color: #4a5568; white-space: nowrap; }
  .footer-community { background: none; border: none; cursor: pointer; font-size: 11px; color: #63b3ed; opacity: 0.8; padding: 0; }
  .footer-community:hover { opacity: 1; }
  .close-btn-bottom { background: #2d3748; color: #a0aec0; border: 1px solid #4a5568; border-radius: 4px; padding: 4px 14px; font-size: 12px; cursor: pointer; }
  .close-btn-bottom:hover { background: #3d4f66; color: #e2e8f0; }
</style>
