<script>
  import { invoke } from "@tauri-apps/api/core";
  import { open } from "@tauri-apps/plugin-dialog";
  import SessionForm from "./lib/SessionForm.svelte";
  import FileList from "./lib/FileList.svelte";
  import ConvertPanel from "./lib/ConvertPanel.svelte";
  import ResultsPanel from "./lib/ResultsPanel.svelte";

  let selectedFiles = [];
  let outputDir = "";
  let session = {
    schema_version: "1.0",
    object: "",
    observer: "",
    frame_type: "light",
    equipment: {
      telescope: "",
      focal_length: null,
      aperture: null,
      filter: "",
      camera: { make: "Canon", model: "", pixel_size_x: 4.3, pixel_size_y: 4.3 }
    },
    location: { latitude: null, longitude: null, elevation: null, site_name: "" },
    output: { raw_mode: "raw_bayer", header_mode: "astro", overwrite: false, write_history: true }
  };
  let results = null;
  let converting = false;

  async function pickFiles() {
    const files = await open({
      multiple: true,
      filters: [{ name: "RAW Files", extensions: ["cr2", "CR2", "nef", "NEF", "arw", "ARW"] }]
    });
    if (files) selectedFiles = Array.isArray(files) ? files : [files];
  }

  async function pickOutputDir() {
    const dir = await open({ directory: true });
    if (dir) outputDir = dir;
  }

  async function runConvert() {
    if (!selectedFiles.length || !outputDir) return;
    converting = true;
    results = null;
    try {
      results = await invoke("convert_files", {
        request: {
          input_paths: selectedFiles,
          output_dir: outputDir,
          session,
          overwrite: session.output.overwrite,
          dry_run: false
        }
      });
    } finally {
      converting = false;
    }
  }
</script>

<main>
  <header>
    <h1>AstroRAW-o-Matic</h1>
    <span class="subtitle">Mostly harmless RAW conversion</span>
  </header>

  <div class="layout">
    <section class="left">
      <FileList files={selectedFiles} on:pick={pickFiles} />
      <ConvertPanel
        {outputDir}
        {converting}
        hasFiles={selectedFiles.length > 0}
        on:pickOutput={pickOutputDir}
        on:convert={runConvert}
      />
      {#if results}
        <ResultsPanel {results} />
      {/if}
    </section>

    <section class="right">
      <SessionForm bind:session />
    </section>
  </div>
</main>

<style>
  :global(*, *::before, *::after) { box-sizing: border-box; margin: 0; padding: 0; }
  :global(body) {
    font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", sans-serif;
    background: #0f1117;
    color: #e2e8f0;
    font-size: 13px;
    height: 100vh;
    overflow: hidden;
  }
  :global(input, select) {
    background: #1a202c;
    border: 1px solid #2d3748;
    color: #e2e8f0;
    border-radius: 4px;
    padding: 4px 8px;
    font-size: 13px;
    width: 100%;
  }
  :global(input:focus, select:focus) {
    outline: none;
    border-color: #63b3ed;
  }
  :global(label) { color: #a0aec0; font-size: 12px; display: block; margin-bottom: 3px; }

  main { display: flex; flex-direction: column; height: 100vh; }

  header {
    padding: 10px 16px;
    background: #1a1f2e;
    border-bottom: 1px solid #2d3748;
    display: flex;
    align-items: baseline;
    gap: 10px;
    flex-shrink: 0;
  }
  h1 { font-size: 16px; font-weight: 700; color: #63b3ed; }
  .subtitle { font-size: 11px; color: #718096; font-style: italic; }

  .layout {
    display: grid;
    grid-template-columns: 1fr 300px;
    flex: 1;
    overflow: hidden;
  }

  .left {
    display: flex;
    flex-direction: column;
    overflow: hidden;
    border-right: 1px solid #2d3748;
  }

  .right {
    overflow-y: auto;
    background: #13171f;
  }
</style>
