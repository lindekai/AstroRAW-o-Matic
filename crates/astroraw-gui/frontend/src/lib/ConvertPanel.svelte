<script>
  import { createEventDispatcher } from "svelte";
  export let outputDir = "";
  export let converting = false;
  export let hasFiles = false;
  const dispatch = createEventDispatcher();

  function basename(path) {
    return path ? path.split(/[\\/]/).pop() || path : "";
  }
</script>

<div class="convert-panel">
  <div class="output-row">
    <button class="dir-btn" on:click={() => dispatch("pickOutput")}>
      Output Folder
    </button>
    <span class="dir-label" title={outputDir}>
      {outputDir ? basename(outputDir) : "Not selected"}
    </span>
  </div>

  <button
    class="convert-btn"
    disabled={!hasFiles || !outputDir || converting}
    on:click={() => dispatch("convert")}
  >
    {#if converting}
      Converting…
    {:else}
      Convert to FITS
    {/if}
  </button>
</div>

<style>
  .convert-panel {
    padding: 10px 12px;
    border-top: 1px solid #2d3748;
    display: flex;
    flex-direction: column;
    gap: 8px;
    flex-shrink: 0;
  }

  .output-row {
    display: flex;
    align-items: center;
    gap: 8px;
  }

  .dir-btn {
    background: #2d3748;
    color: #a0aec0;
    border: 1px solid #4a5568;
    border-radius: 4px;
    padding: 5px 10px;
    font-size: 12px;
    cursor: pointer;
    white-space: nowrap;
    flex-shrink: 0;
  }
  .dir-btn:hover { background: #3d4f66; color: #e2e8f0; }

  .dir-label {
    font-size: 12px;
    color: #718096;
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }

  .convert-btn {
    background: #2b6cb0;
    color: white;
    border: none;
    border-radius: 5px;
    padding: 9px;
    font-size: 13px;
    font-weight: 600;
    cursor: pointer;
    width: 100%;
    letter-spacing: 0.3px;
  }
  .convert-btn:hover:not(:disabled) { background: #3182ce; }
  .convert-btn:disabled { opacity: 0.4; cursor: not-allowed; }
</style>
