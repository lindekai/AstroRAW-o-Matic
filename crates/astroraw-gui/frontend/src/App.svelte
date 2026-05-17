<script>
  import { invoke } from "@tauri-apps/api/core";
  import { open, save } from "@tauri-apps/plugin-dialog";
  import { getCurrentWindow } from "@tauri-apps/api/window";
  import { onMount } from "svelte";
  import SessionForm from "./lib/SessionForm.svelte";
  import FileList from "./lib/FileList.svelte";
  import ConvertPanel from "./lib/ConvertPanel.svelte";
  import ResultsPanel from "./lib/ResultsPanel.svelte";
  import Footer from "./lib/Footer.svelte";
  import HelpDialog from "./lib/HelpDialog.svelte";

  let selectedFiles = [];
  let outputDir = "";
  let results = null;
  let converting = false;
  let sessionStatus = "";
  let helpOpen = false;
  let dragOver = false;

  const RAW_EXTS = ["cr2", "nef", "arw", "raf"];

  onMount(async () => {
    const win = getCurrentWindow();
    const unlisten = await win.onDragDropEvent(async (event) => {
      const type = event.payload.type;
      if (type === "drop") {
        const dropped = event.payload.paths || [];
        // Separate files and directories
        const rawFiles = dropped.filter(p =>
          RAW_EXTS.includes(p.split(".").pop()?.toLowerCase() ?? "")
        );
        // Scan dropped folders for RAW files
        const dirs = dropped.filter(p => !p.includes(".") || !RAW_EXTS.includes(p.split(".").pop()?.toLowerCase() ?? ""));
        let folderFiles = [];
        for (const dir of dirs) {
          const found = await invoke("scan_folder", { path: dir });
          folderFiles = [...folderFiles, ...found];
        }
        const allNew = [...rawFiles, ...folderFiles];
        if (allNew.length) {
          selectedFiles = [...selectedFiles, ...allNew];
          if (!session.session_date && allNew[0]) {
            await prefillDateFromFile(allNew[0]);
          }
        }
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
    session_date: "",
    session_time: "",
    date_obs: null,
    frame_type: "light",
    equipment: {
      telescope: "",
      focal_length: null,
      aperture: null,
      filter: "",
      camera: { make: "Canon", model: "", pixel_size_x: 4.3, pixel_size_y: 4.3 }
    },
    location: { latitude: null, longitude: null, elevation: null, site_name: "" },
    output: { raw_mode: "raw_bayer", header_mode: "astro", overwrite: false, write_history: true, json_filename_pattern: "session_{object}_{date}" }
  };

  async function prefillDateFromFile(path) {
    try {
      const meta = await invoke("inspect_file", { path });
      if (meta.success && meta.date_obs) {
        // date_obs format: "2024-01-09T03:42:09"
        const [datePart, timePart] = meta.date_obs.split("T");
        if (datePart && !session.session_date) session.session_date = datePart;
        if (timePart && !session.session_time) session.session_time = timePart;
      }
    } catch (_) {}
  }

  async function pickFiles() {
    const files = await open({
      multiple: true,
      filters: [{ name: "RAW Files", extensions: ["cr2", "CR2", "nef", "NEF", "arw", "ARW"] }]
    });
    if (files) {
      const paths = Array.isArray(files) ? files : [files];
      selectedFiles = [...selectedFiles, ...paths];
      if (!session.session_date && paths[0]) await prefillDateFromFile(paths[0]);
    }
  }

  async function pickFolder() {
    const dir = await open({ directory: true });
    if (!dir) return;
    const paths = await invoke("scan_folder", { path: dir });
    if (paths.length === 0) {
      alert("Keine RAW-Dateien in diesem Ordner gefunden.");
      return;
    }
    selectedFiles = [...selectedFiles, ...paths];
    if (!session.session_date && paths[0]) await prefillDateFromFile(paths[0]);
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

  function resolveFilename(pattern, s) {
    const date = new Date().toISOString().slice(0, 10);
    return (pattern || "session")
      .replace("{object}", (s.object || "session").replace(/\s+/g, "_"))
      .replace("{date}", date)
      .replace("{observer}", (s.observer || "").replace(/\s+/g, "_"))
      + ".json";
  }

  async function saveSession() {
    const pattern = session.output?.json_filename_pattern;
    const defaultName = resolveFilename(pattern, session);
    const file = await save({
      filters: [{ name: "Session JSON", extensions: ["json"] }],
      defaultPath: defaultName
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
    <button class="tb-btn help-btn" on:click={() => helpOpen = true}>? Hilfe</button>
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
        <FileList files={selectedFiles} on:pick={pickFiles} on:pickFolder={pickFolder} />
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
  <Footer cliVersion="0.3.0" guiVersion="0.3.0" />
</main>

<HelpDialog bind:open={helpOpen} />

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
  .help-btn { margin-left: auto; color: #63b3ed; border-color: #2b6cb0; }
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
