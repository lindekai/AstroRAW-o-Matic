<script>
  export let open = false;
  function close() { open = false; }

  let activeTab = "workflow";
  const tabs = [
    { id: "workflow", label: "Workflow" },
    { id: "session",  label: "Session-Felder" },
    { id: "json",     label: "JSON-Muster" },
    { id: "fits",     label: "FITS-Header" },
    { id: "cli",      label: "CLI" },
  ];
</script>

{#if open}
<div class="overlay" on:click={close} role="button" tabindex="0" on:keydown={(e) => e.key === 'Escape' && close()}>
  <div class="dialog" on:click|stopPropagation role="document">

    <div class="dialog-header">
      <span class="title">AstroRAW-o-Matic — Hilfe</span>
      <button class="close-btn" on:click={close}>✕</button>
    </div>

    <div class="tabs">
      {#each tabs as tab}
        <button class="tab" class:active={activeTab === tab.id} on:click={() => activeTab = tab.id}>
          {tab.label}
        </button>
      {/each}
    </div>

    <div class="content">

      {#if activeTab === "workflow"}
        <h3>Empfohlener Workflow</h3>
        <ol>
          <li>CR2-Dateien per Drag & Drop oder <strong>+ Add Files</strong> laden</li>
          <li>Session-Formular ausfüllen (Objekt, Teleskop, Kamera …)</li>
          <li>Session als JSON speichern: <strong>💾 Save JSON</strong></li>
          <li>Output-Ordner wählen: <strong>Output Folder</strong></li>
          <li><strong>Convert to FITS</strong> klicken</li>
          <li>FITS-Dateien in Siril öffnen → Debayern → Stacken</li>
        </ol>
        <div class="note">
          Das Bayer-Pattern wird automatisch aus den RAW-Daten ermittelt.
          Für Canon 600D / 600Da ist das korrekte Muster <strong>GBRG</strong>.
        </div>

      {:else if activeTab === "session"}
        <h3>Session-Felder erklärt</h3>
        <table>
          <tr><td>Object</td><td>Zielobjekt (z.B. M31, NGC 7000, Horsehead Nebula)</td></tr>
          <tr><td>Observer</td><td>Name des Beobachters — erscheint im FITS-Header als OBSERVER</td></tr>
          <tr><td>Session Date</td><td>Beobachtungsdatum — wird aus EXIF vorausgefüllt</td></tr>
          <tr><td>Session Time</td><td>Beobachtungszeit (UTC) — wird aus EXIF vorausgefüllt</td></tr>
          <tr><td>Frame Type</td><td>Light / Dark / Flat / Bias → IMAGETYP im FITS-Header</td></tr>
          <tr><td>Filter</td><td>Verwendeter Filter (UV/IR, DNB, Halpha, OIII, SII, L, R, G, B)</td></tr>
          <tr><td>Camera Model</td><td>Kameramodell → INSTRUME im FITS-Header</td></tr>
          <tr><td>Pixel X/Y (µm)</td><td>Pixelgröße des Sensors (Canon 600D: 4.3 µm)</td></tr>
          <tr><td>Telescope</td><td>Teleskop oder Objektiv → TELESCOP im FITS-Header</td></tr>
          <tr><td>Focal Length</td><td>Brennweite in mm → FOCALLEN im FITS-Header</td></tr>
          <tr><td>Aperture</td><td>Öffnung in mm → APTDIA + FOCRATIO im FITS-Header</td></tr>
          <tr><td>Focal Ratio</td><td>Automatisch berechnet: Brennweite ÷ Öffnung</td></tr>
          <tr><td>Site Name</td><td>Name des Beobachtungsorts</td></tr>
          <tr><td>Lat / Lon</td><td>Koordinaten → SITELAT / SITELONG im FITS-Header</td></tr>
          <tr><td>Elevation</td><td>Höhe über NN in Metern → SITEELEV im FITS-Header</td></tr>
        </table>

      {:else if activeTab === "json"}
        <h3>JSON Dateinamen-Muster</h3>
        <p>Das Feld <strong>JSON Filename Pattern</strong> im OUTPUT-Bereich steuert den Vorschlagsnamen beim Speichern.</p>
        <table>
          <tr><td><code>{'{object}'}</code></td><td>Objektname (Leerzeichen → Unterstriche)</td></tr>
          <tr><td><code>{'{date}'}</code></td><td>Aktuelles Datum (YYYY-MM-DD)</td></tr>
          <tr><td><code>{'{observer}'}</code></td><td>Name des Beobachters</td></tr>
        </table>
        <div class="example">
          <strong>Beispiele:</strong><br/>
          <code>session_{'{object}'}_{'{date}'}</code> → <em>session_Horsehead_Nebula_2026-05-17.json</em><br/>
          <code>{'{observer}'}_{'{object}'}</code> → <em>Kai_Linde_M31.json</em>
        </div>

      {:else if activeTab === "fits"}
        <h3>Wichtige FITS-Keywords</h3>
        <table>
          <tr><th>Keyword</th><th>Inhalt</th><th>Quelle</th></tr>
          <tr><td>BAYERPAT</td><td>Bayer-Muster (z.B. GBRG)</td><td>rawler (automatisch)</td></tr>
          <tr><td>PEDESTAL</td><td>Schwarzpegel in ADU (~2046)</td><td>rawler (automatisch)</td></tr>
          <tr><td>BLACKLVL</td><td>Schwarzpegel in ADU</td><td>rawler (automatisch)</td></tr>
          <tr><td>WHITELEV</td><td>Sättigungspegel in ADU</td><td>rawler (automatisch)</td></tr>
          <tr><td>EXPTIME</td><td>Belichtungszeit in Sekunden</td><td>EXIF</td></tr>
          <tr><td>ISOSPEED</td><td>ISO-Einstellung</td><td>EXIF</td></tr>
          <tr><td>DATE-OBS</td><td>Aufnahmedatum/-zeit (UTC)</td><td>EXIF / Session</td></tr>
          <tr><td>FOCRATIO</td><td>Fokalverhältnis (f/Zahl)</td><td>Session (berechnet)</td></tr>
          <tr><td>CBLACK_R/G/B</td><td>Weißabgleich-Koeffizienten</td><td>rawler (automatisch)</td></tr>
          <tr><td>IMAGETYP</td><td>LIGHT / DARK / FLAT / BIAS</td><td>Session / CLI</td></tr>
        </table>
        <div class="note">
          <strong>Metadaten-Priorität:</strong><br/>
          file_overrides > Session-JSON > EXIF aus RAW-Datei > Standardwerte
        </div>

      {:else if activeTab === "cli"}
        <h3>CLI-Befehle</h3>
        <pre>
# Metadaten einer RAW-Datei anzeigen
astroraw-o-matic inspect bild.CR2

# Einzeldatei konvertieren
astroraw-o-matic convert bild.CR2 \
  --output ./FITS \
  --object "M31" --type light

# Verzeichnis mit Session-JSON
astroraw-o-matic convert ./RAW \
  --output ./FITS \
  --metadata session.json

# Dry-Run (nichts schreiben)
astroraw-o-matic convert ./RAW \
  --output ./FITS --dry-run

# Session-JSON validieren
astroraw-o-matic validate session.json
        </pre>
        <div class="note">Kurzname: <code>arom</code> (Alias anlegen mit <code>alias arom='astroraw-o-matic'</code>)</div>
      {/if}

    </div>

    <div class="dialog-footer">
      <span>AstroRAW-o-Matic v0.3.0 — Mostly harmless RAW conversion</span>
      <button class="close-btn-bottom" on:click={close}>Schließen</button>
    </div>

  </div>
</div>
{/if}

<style>
  .overlay {
    position: fixed;
    inset: 0;
    background: rgba(0,0,0,0.6);
    display: flex;
    align-items: center;
    justify-content: center;
    z-index: 100;
  }

  .dialog {
    background: #1a1f2e;
    border: 1px solid #2d3748;
    border-radius: 8px;
    width: 720px;
    max-height: 80vh;
    display: flex;
    flex-direction: column;
    box-shadow: 0 8px 32px rgba(0,0,0,0.5);
  }

  .dialog-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: 12px 16px;
    border-bottom: 1px solid #2d3748;
    flex-shrink: 0;
  }

  .title { font-size: 14px; font-weight: 600; color: #63b3ed; }

  .close-btn {
    background: none;
    border: none;
    color: #718096;
    cursor: pointer;
    font-size: 14px;
    padding: 2px 6px;
  }
  .close-btn:hover { color: #e2e8f0; }

  .tabs {
    display: flex;
    border-bottom: 1px solid #2d3748;
    flex-shrink: 0;
  }

  .tab {
    background: none;
    border: none;
    color: #718096;
    padding: 8px 14px;
    font-size: 12px;
    cursor: pointer;
    border-bottom: 2px solid transparent;
    margin-bottom: -1px;
  }
  .tab:hover { color: #e2e8f0; }
  .tab.active { color: #63b3ed; border-bottom-color: #63b3ed; }

  .content {
    padding: 16px;
    overflow-y: auto;
    flex: 1;
    font-size: 12px;
    line-height: 1.6;
    color: #a0aec0;
  }

  h3 { color: #e2e8f0; margin-bottom: 10px; font-size: 13px; }
  ol { padding-left: 18px; display: flex; flex-direction: column; gap: 5px; }
  li { color: #a0aec0; }
  strong { color: #e2e8f0; }
  code { background: #0f1117; padding: 1px 5px; border-radius: 3px; color: #63b3ed; font-family: monospace; }
  pre { background: #0f1117; padding: 10px; border-radius: 5px; font-size: 11px; color: #68d391; line-height: 1.7; overflow-x: auto; }

  table { width: 100%; border-collapse: collapse; margin-top: 8px; }
  th { text-align: left; color: #63b3ed; padding: 5px 8px; border-bottom: 1px solid #2d3748; font-size: 11px; }
  td { padding: 5px 8px; border-bottom: 1px solid #1a202c; vertical-align: top; }
  td:first-child { color: #e2e8f0; white-space: nowrap; font-family: monospace; width: 140px; }

  .note {
    margin-top: 12px;
    padding: 8px 10px;
    background: #0f1117;
    border-left: 3px solid #63b3ed;
    border-radius: 3px;
    font-size: 11px;
    color: #718096;
  }

  .example {
    margin-top: 12px;
    padding: 8px 10px;
    background: #0f1117;
    border-radius: 3px;
    font-size: 11px;
    line-height: 1.8;
  }
  em { color: #68d391; font-style: normal; }

  .dialog-footer {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: 10px 16px;
    border-top: 1px solid #2d3748;
    flex-shrink: 0;
  }
  .dialog-footer span { font-size: 11px; color: #4a5568; font-style: italic; }

  .close-btn-bottom {
    background: #2d3748;
    color: #a0aec0;
    border: 1px solid #4a5568;
    border-radius: 4px;
    padding: 4px 14px;
    font-size: 12px;
    cursor: pointer;
  }
  .close-btn-bottom:hover { background: #3d4f66; color: #e2e8f0; }
</style>
