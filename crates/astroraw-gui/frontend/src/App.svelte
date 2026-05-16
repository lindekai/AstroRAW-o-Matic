<script>
  import { invoke } from "@tauri-apps/api/core";
  import { open, save } from "@tauri-apps/plugin-dialog";
  import { getCurrentWindow } from "@tauri-apps/api/window";
  import { onMount } from "svelte";
  import SessionForm from "./lib/SessionForm.svelte";
  import FileList from "./lib/FileList.svelte";
  import ConvertPanel from "./lib/ConvertPanel.svelte";
  import ResultsPanel from "./lib/ResultsPanel.svelte";

  let selectedFiles = [];
  let outputDir = "";
  let results = null;
  let converting = false;
  let sessionStatus = "";
  let dragOver = false;

  const RAW_EXTS = ["cr2", "nef", "arw", "raf"];

  onMount(async () => {
    const win = getCurrentWindow();
    const unlisten = await win.onDragDropEvent((event) => {
      const type = event.payload.type;
      if (type === "drop") {
        const paths = (event.payload.paths || []).filter(p =>
          RAW_EXTS.includes(p.split(".").pop()?.toLowerCase() ?? "")
        );
        if (paths.length) selectedFiles = [...selectedFiles, ...paths];
        dragOver = false;
      } else if (type === "over" || type === "enter") {
        dragOver = true;
      } else {
        dragOver = false;
      }
    });
    return unlisten;
  });

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

  async function loadSession() {
    const file = await open({
      filters: [{ name: "Session JSON", extensions: ["json"] }]
    });
    if (!file) return;
    try {
      const result = await invoke("load_session_json", { path: file });
      session = result;
      sessionStatus = "Loaded";
      setTimeout(() => sessionStatus = "", 2000);
    } catch (e) {
      sessionStatus = "Error: " + e;
    }
  }

  async function saveSession() {
    const file = await save({
      filters: [{ name: "Session JSON", extensions: ["json"] }],
      defaultPath: "session.json"
    });
    if (!file) return;
    try {
      await invoke("save_session_json", { path: file, session });
      sessionStatus = "Saved";
      setTimeout(() => sessionStatus = "", 2000);
    } catch (e) {
      sessionStatus = "Error: " + e;
    }
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

<main class:drag-over={dragOver}>

  <!-- Toolbar -->
  <div class="toolbar">
    <span class="toolbar-label">Session</span>
    <button class="tb-btn" on:click={loadSession}>📂 Load JSON</button>
    <button class="tb-btn" on:click={saveSession}>💾 Save JSON</button>
    {#if sessionStatus}
      <span class="status-msg">{sessionStatus}</span>
    {/if}
  </div>

  <!-- Main layout -->
  <div class="layout">

    <!-- Left: Session form -->
    <aside class="left">
      <SessionForm bind:session />
    </aside>

    <!-- Right: Files + Convert + Results -->
    <div class="right">
      <div class="files-area">
        <FileList files={selectedFiles} on:pick={pickFiles} />
      </div>
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
    padding: 2px 5px;
    font-size: 11px;
    width: 100%;
    height: 22px;
  }
  :global(input:focus, select:focus) { outline: none; border-color: #63b3ed; }
  :global(label) { color: #718096; font-size: 10px; display: block; margin-bottom: 1px; white-space: nowrap; }

  main {
    display: flex;
    flex-direction: column;
    height: 100vh;
    overflow: hidden;
  }
  main.drag-over { outline: 2px solid #63b3ed; outline-offset: -2px; }

  /* Toolbar */
  .toolbar {
    display: flex;
    align-items: center;
    gap: 6px;
    padding: 5px 10px;
    background: #1a1f2e;
    border-bottom: 1px solid #2d3748;
    flex-shrink: 0;
  }
  .toolbar-label { font-size: 11px; color: #4a5568; margin-right: 2px; }
  .tb-btn {
    background: #2d3748;
    color: #a0aec0;
    border: 1px solid #4a5568;
    border-radius: 3px;
    padding: 3px 9px;
    font-size: 11px;
    cursor: pointer;
  }
  .tb-btn:hover { background: #3d4f66; color: #e2e8f0; }
  .status-msg { font-size: 11px; color: #68d391; margin-left: 4px; }

  /* Layout */
  .layout {
    display: grid;
    grid-template-columns: 370px 1fr;
    flex: 1;
    overflow: hidden;
    min-height: 0;
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
    min-height: 0;
  }

  .files-area {
    flex: 1;
    overflow: hidden;
    min-height: 0;
    display: flex;
    flex-direction: column;
  }
</style>
