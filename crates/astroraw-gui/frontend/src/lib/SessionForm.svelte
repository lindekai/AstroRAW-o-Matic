<script>
  export let session;
  const frameTypes = ["light", "dark", "flat", "bias"];
  const headerModes = ["astro", "minimal"];

  $: focalRatio = (session.equipment.focal_length && session.equipment.aperture && session.equipment.aperture > 0)
    ? (session.equipment.focal_length / session.equipment.aperture).toFixed(1)
    : null;
</script>

<form class="sf">

  <div class="sec">SESSION</div>
  <div class="g2">
    <div class="f"><label for="obj">Object</label>
      <input id="obj" bind:value={session.object} placeholder="M31 …" /></div>
    <div class="f"><label for="obs">Observer</label>
      <input id="obs" bind:value={session.observer} placeholder="Name" /></div>
    <div class="f"><label for="ft">Frame Type</label>
      <select id="ft" bind:value={session.frame_type}>
        {#each frameTypes as t}<option value={t}>{t.charAt(0).toUpperCase()+t.slice(1)}</option>{/each}
      </select></div>
    <div class="f"><label for="flt">Filter</label>
      <input id="flt" list="filter-list" bind:value={session.equipment.filter} placeholder="UV/IR, Ha …" />
      <datalist id="filter-list">
        <option value="UV/IR" />
        <option value="DNB" />
        <option value="Halpha" />
        <option value="OIII" />
        <option value="SII" />
        <option value="L" />
        <option value="R" />
        <option value="G" />
        <option value="B" />
      </datalist>
    </div>
  </div>

  <div class="sec">CAMERA</div>
  <div class="g2">
    <div class="f"><label for="cmk">Make</label>
      <input id="cmk" bind:value={session.equipment.camera.make} placeholder="Canon" /></div>
    <div class="f"><label for="cmd">Model</label>
      <input id="cmd" bind:value={session.equipment.camera.model} placeholder="EOS 600Da" /></div>
    <div class="f"><label for="px">Pixel X (µm)</label>
      <input id="px" type="number" step="0.01" bind:value={session.equipment.camera.pixel_size_x} /></div>
    <div class="f"><label for="py">Pixel Y (µm)</label>
      <input id="py" type="number" step="0.01" bind:value={session.equipment.camera.pixel_size_y} /></div>
  </div>

  <div class="sec">TELESCOPE</div>
  <div class="g2">
    <div class="f g2-span"><label for="tel">Telescope / Lens</label>
      <input id="tel" bind:value={session.equipment.telescope} placeholder="Skywatcher 80ED" /></div>
    <div class="f"><label for="fl">Focal Length (mm)</label>
      <input id="fl" type="number" bind:value={session.equipment.focal_length} placeholder="600" /></div>
    <div class="f"><label for="ap">Aperture (mm)</label>
      <input id="ap" type="number" bind:value={session.equipment.aperture} placeholder="80" /></div>
    <div class="f"><label>Focal Ratio</label>
      <div class="computed">{focalRatio ? 'f/' + focalRatio : '—'}</div>
    </div>
  </div>

  <div class="sec">LOCATION</div>
  <div class="g2">
    <div class="f g2-span"><label for="sn">Site Name</label>
      <input id="sn" bind:value={session.location.site_name} placeholder="Backyard, Berlin" /></div>
    <div class="f"><label for="lat">Latitude (°)</label>
      <input id="lat" type="number" step="0.0001" bind:value={session.location.latitude} /></div>
    <div class="f"><label for="lon">Longitude (°)</label>
      <input id="lon" type="number" step="0.0001" bind:value={session.location.longitude} /></div>
    <div class="f"><label for="elv">Elevation (m)</label>
      <input id="elv" type="number" bind:value={session.location.elevation} /></div>
  </div>

  <div class="sec">OUTPUT</div>
  <div class="g2">
    <div class="f"><label for="hm">Header Mode</label>
      <select id="hm" bind:value={session.output.header_mode}>
        {#each headerModes as m}<option value={m}>{m}</option>{/each}
      </select></div>
    <div class="f checks">
      <label class="ck"><input type="checkbox" bind:checked={session.output.overwrite} /> Overwrite FITS</label>
      <label class="ck"><input type="checkbox" bind:checked={session.output.write_history} /> Write HISTORY</label>
    </div>
    <div class="f g2-span">
      <label for="jsfn">JSON Filename Pattern</label>
      <input id="jsfn" bind:value={session.output.json_filename_pattern}
             placeholder="session_{object}_{date}" />
      <span class="hint">Placeholders: {"{object}"} {"{date}"} {"{observer}"}</span>
    </div>
  </div>

</form>

<style>
  .sf { padding: 6px 10px; display: flex; flex-direction: column; gap: 3px; }

  .sec {
    font-size: 9px; font-weight: 700; color: #63b3ed;
    letter-spacing: 1px; margin-top: 6px;
    padding-bottom: 2px; border-bottom: 1px solid #2d3748;
  }

  .g2 { display: grid; grid-template-columns: 1fr 1fr; gap: 4px 8px; padding-top: 3px; }
  .g2-span { grid-column: 1 / -1; }

  .f { display: flex; flex-direction: column; gap: 1px; min-width: 0; }

  :global(.sf input, .sf select) { height: 22px; font-size: 11px; padding: 2px 5px; }

  .checks { display: flex; flex-direction: column; justify-content: center; gap: 3px; padding-top: 2px; }
  .ck {
    display: flex; align-items: center; gap: 4px;
    font-size: 11px; color: #a0aec0; cursor: pointer;
  }
  .ck input[type="checkbox"] { width: auto; height: auto; accent-color: #63b3ed; }

  .hint { font-size: 9px; color: #4a5568; margin-top: 1px; }

  .computed {
    height: 22px;
    line-height: 22px;
    padding: 0 5px;
    background: #0f1117;
    border: 1px solid #2d3748;
    border-radius: 3px;
    font-size: 11px;
    color: #63b3ed;
    font-weight: 600;
  }
</style>
