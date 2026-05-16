<script>
  export let session;
  const frameTypes = ["light", "dark", "flat", "bias"];
  const headerModes = ["astro", "minimal"];
</script>

<form class="session-form">

  <div class="section">SESSION</div>
  <div class="grid2">
    <div class="field">
      <label for="object">Object</label>
      <input id="object" bind:value={session.object} placeholder="M31, NGC 7000 …" />
    </div>
    <div class="field">
      <label for="observer">Observer</label>
      <input id="observer" bind:value={session.observer} placeholder="Your name" />
    </div>
    <div class="field">
      <label for="frame_type">Frame Type</label>
      <select id="frame_type" bind:value={session.frame_type}>
        {#each frameTypes as t}
          <option value={t}>{t.charAt(0).toUpperCase() + t.slice(1)}</option>
        {/each}
      </select>
    </div>
    <div class="field">
      <label for="filter">Filter</label>
      <input id="filter" bind:value={session.equipment.filter} placeholder="L, Ha, OIII …" />
    </div>
  </div>

  <div class="section">CAMERA</div>
  <div class="grid2">
    <div class="field">
      <label for="cam_model">Model</label>
      <input id="cam_model" bind:value={session.equipment.camera.model} placeholder="EOS 600Da" />
    </div>
    <div class="field">
      <label for="cam_make">Make</label>
      <input id="cam_make" bind:value={session.equipment.camera.make} placeholder="Canon" />
    </div>
    <div class="field">
      <label for="px">Pixel X (µm)</label>
      <input id="px" type="number" step="0.01" bind:value={session.equipment.camera.pixel_size_x} />
    </div>
    <div class="field">
      <label for="py">Pixel Y (µm)</label>
      <input id="py" type="number" step="0.01" bind:value={session.equipment.camera.pixel_size_y} />
    </div>
  </div>

  <div class="section">TELESCOPE</div>
  <div class="grid2">
    <div class="field span2">
      <label for="telescope">Telescope / Lens</label>
      <input id="telescope" bind:value={session.equipment.telescope} placeholder="Skywatcher 80ED" />
    </div>
    <div class="field">
      <label for="focal">Focal Length (mm)</label>
      <input id="focal" type="number" bind:value={session.equipment.focal_length} placeholder="600" />
    </div>
    <div class="field">
      <label for="aperture">Aperture (mm)</label>
      <input id="aperture" type="number" bind:value={session.equipment.aperture} placeholder="80" />
    </div>
  </div>

  <div class="section">LOCATION</div>
  <div class="grid2">
    <div class="field span2">
      <label for="site">Site Name</label>
      <input id="site" bind:value={session.location.site_name} placeholder="Backyard, Berlin" />
    </div>
    <div class="field">
      <label for="lat">Latitude (°)</label>
      <input id="lat" type="number" step="0.0001" bind:value={session.location.latitude} />
    </div>
    <div class="field">
      <label for="lon">Longitude (°)</label>
      <input id="lon" type="number" step="0.0001" bind:value={session.location.longitude} />
    </div>
    <div class="field">
      <label for="elev">Elevation (m)</label>
      <input id="elev" type="number" bind:value={session.location.elevation} />
    </div>
  </div>

  <div class="section">OUTPUT</div>
  <div class="grid2">
    <div class="field">
      <label for="header_mode">Header Mode</label>
      <select id="header_mode" bind:value={session.output.header_mode}>
        {#each headerModes as m}
          <option value={m}>{m}</option>
        {/each}
      </select>
    </div>
    <div class="field checks">
      <label class="check-label">
        <input type="checkbox" bind:checked={session.output.overwrite} />
        Overwrite FITS
      </label>
      <label class="check-label">
        <input type="checkbox" bind:checked={session.output.write_history} />
        Write HISTORY
      </label>
    </div>
  </div>

</form>

<style>
  .session-form {
    padding: 10px 12px;
    display: flex;
    flex-direction: column;
    gap: 4px;
  }

  .section {
    font-size: 10px;
    font-weight: 700;
    color: #63b3ed;
    letter-spacing: 1px;
    margin-top: 8px;
    padding-bottom: 3px;
    border-bottom: 1px solid #2d3748;
  }

  .grid2 {
    display: grid;
    grid-template-columns: 1fr 1fr;
    gap: 6px 10px;
    padding-top: 4px;
  }

  .field { display: flex; flex-direction: column; gap: 2px; min-width: 0; }
  .span2 { grid-column: 1 / -1; }

  .checks {
    display: flex;
    flex-direction: column;
    justify-content: center;
    gap: 4px;
    padding-top: 2px;
  }

  .check-label {
    display: flex;
    align-items: center;
    gap: 5px;
    font-size: 11px;
    color: #a0aec0;
    cursor: pointer;
    white-space: nowrap;
  }
  .check-label input[type="checkbox"] {
    width: auto;
    height: auto;
    accent-color: #63b3ed;
  }
</style>
