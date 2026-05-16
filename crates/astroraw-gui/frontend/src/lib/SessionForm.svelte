<script>
  export let session;

  const frameTypes = ["light", "dark", "flat", "bias"];
  const headerModes = ["astro", "minimal"];
</script>

<div class="session-form">
  <div class="section-title">Session</div>

  <div class="field">
    <label>Object</label>
    <input bind:value={session.object} placeholder="M31, NGC 7000 …" />
  </div>
  <div class="field">
    <label>Observer</label>
    <input bind:value={session.observer} placeholder="Your name" />
  </div>
  <div class="field">
    <label>Frame Type</label>
    <select bind:value={session.frame_type}>
      {#each frameTypes as t}
        <option value={t}>{t.charAt(0).toUpperCase() + t.slice(1)}</option>
      {/each}
    </select>
  </div>

  <div class="section-title">Equipment</div>

  <div class="field">
    <label>Camera Model</label>
    <input bind:value={session.equipment.camera.model} placeholder="EOS 600Da" />
  </div>
  <div class="field-row">
    <div class="field">
      <label>Pixel X (µm)</label>
      <input type="number" step="0.01" bind:value={session.equipment.camera.pixel_size_x} />
    </div>
    <div class="field">
      <label>Pixel Y (µm)</label>
      <input type="number" step="0.01" bind:value={session.equipment.camera.pixel_size_y} />
    </div>
  </div>
  <div class="field">
    <label>Telescope / Lens</label>
    <input bind:value={session.equipment.telescope} placeholder="Skywatcher 80ED" />
  </div>
  <div class="field-row">
    <div class="field">
      <label>Focal Length (mm)</label>
      <input type="number" bind:value={session.equipment.focal_length} placeholder="600" />
    </div>
    <div class="field">
      <label>Aperture (mm)</label>
      <input type="number" bind:value={session.equipment.aperture} placeholder="80" />
    </div>
  </div>
  <div class="field">
    <label>Filter</label>
    <input bind:value={session.equipment.filter} placeholder="L, Ha, OIII …" />
  </div>

  <div class="section-title">Location</div>

  <div class="field">
    <label>Site Name</label>
    <input bind:value={session.location.site_name} placeholder="Backyard, Berlin" />
  </div>
  <div class="field-row">
    <div class="field">
      <label>Latitude (°)</label>
      <input type="number" step="0.0001" bind:value={session.location.latitude} />
    </div>
    <div class="field">
      <label>Longitude (°)</label>
      <input type="number" step="0.0001" bind:value={session.location.longitude} />
    </div>
  </div>
  <div class="field">
    <label>Elevation (m)</label>
    <input type="number" bind:value={session.location.elevation} />
  </div>

  <div class="section-title">Output</div>

  <div class="field">
    <label>Header Mode</label>
    <select bind:value={session.output.header_mode}>
      {#each headerModes as m}
        <option value={m}>{m}</option>
      {/each}
    </select>
  </div>
  <div class="checkbox-row">
    <input type="checkbox" id="overwrite" bind:checked={session.output.overwrite} />
    <label for="overwrite" style="display:inline; margin:0">Overwrite existing FITS</label>
  </div>
  <div class="checkbox-row">
    <input type="checkbox" id="history" bind:checked={session.output.write_history} />
    <label for="history" style="display:inline; margin:0">Write HISTORY record</label>
  </div>
</div>

<style>
  .session-form { padding: 10px 12px; display: flex; flex-direction: column; gap: 6px; }

  .section-title {
    font-size: 11px;
    font-weight: 600;
    color: #63b3ed;
    text-transform: uppercase;
    letter-spacing: 0.8px;
    margin-top: 10px;
    padding-bottom: 4px;
    border-bottom: 1px solid #2d3748;
  }

  .field { display: flex; flex-direction: column; gap: 2px; }
  .field-row { display: grid; grid-template-columns: 1fr 1fr; gap: 6px; }

  .checkbox-row {
    display: flex;
    align-items: center;
    gap: 6px;
    font-size: 12px;
    color: #a0aec0;
  }
  .checkbox-row input[type="checkbox"] {
    width: auto;
    accent-color: #63b3ed;
  }
</style>
