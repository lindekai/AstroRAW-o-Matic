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
  <div class="layout">
    <aside class="left">
      <SessionForm bind:session />
    </aside>

    <div class="right">
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
    </div>
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
    border-radius: 3px;
    padding: 3px 6px;
    font-size: 12px;
    width: 100%;
    height: 24px;
  }
  :global(input:focus, select:focus) {
    outline: none;
    border-color: #63b3ed;
  }
  :global(label) {
    color: #718096;
    font-size: 11px;
    display: block;
    margin-bottom: 2px;
    white-space: nowrap;
  }

  main { display: flex; flex-direction: column; height: 100vh; overflow: hidden; }

  .layout {
    display: grid;
    grid-template-columns: 380px 1fr;
    flex: 1;
    overflow: hidden;
  }

  .left {
    border-right: 1px solid #2d3748;
    overflow-y: auto;
    background: #0f1117;
  }

  .right {
    display: flex;
    flex-direction: column;
    overflow: hidden;
  }
</style>
