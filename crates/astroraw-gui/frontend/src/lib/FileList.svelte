<script>
  import { createEventDispatcher } from "svelte";
  export let files = [];
  const dispatch = createEventDispatcher();

  function basename(path) {
    return path.split(/[\\/]/).pop();
  }

  function removeFile(index) {
    files = files.filter((_, i) => i !== index);
  }

  function clearAll() {
    files = [];
  }
</script>

<div class="file-list">
  <div class="header">
    <span>RAW Files ({files.length})</span>
    <div class="actions">
      {#if files.length > 0}
        <button class="clear-btn" on:click={clearAll} title="Alle Dateien entfernen">✕ Clear</button>
      {/if}
      <button class="add-btn" on:click={() => dispatch("pick")}>+ Add Files</button>
    </div>
  </div>

  <div class="list">
    {#if files.length === 0}
      <div class="empty">
        <p>No files selected.</p>
        <p class="hint">Click "Add Files" or drag & drop CR2 files here.</p>
      </div>
    {:else}
      {#each files as file, i}
        <div class="file-row">
          <span class="icon">📷</span>
          <span class="name">{basename(file)}</span>
          <button class="remove-btn" on:click={() => removeFile(i)} title="Entfernen">✕</button>
        </div>
      {/each}
    {/if}
  </div>
</div>

<style>
  .file-list { display: flex; flex-direction: column; flex: 1; overflow: hidden; }

  .header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: 6px 12px;
    background: #1a1f2e;
    border-bottom: 1px solid #2d3748;
    font-size: 12px;
    color: #a0aec0;
    flex-shrink: 0;
  }

  .actions { display: flex; gap: 6px; align-items: center; }

  .add-btn {
    background: #2b6cb0;
    color: white;
    border: none;
    border-radius: 4px;
    padding: 4px 10px;
    font-size: 12px;
    cursor: pointer;
  }
  .add-btn:hover { background: #3182ce; }

  .clear-btn {
    background: #742a2a;
    color: #feb2b2;
    border: none;
    border-radius: 4px;
    padding: 4px 10px;
    font-size: 12px;
    cursor: pointer;
  }
  .clear-btn:hover { background: #9b2c2c; color: white; }

  .list { flex: 1; overflow-y: auto; padding: 4px 0; }

  .empty {
    padding: 24px 16px;
    text-align: center;
    color: #4a5568;
  }
  .hint { font-size: 11px; margin-top: 4px; }

  .file-row {
    display: flex;
    align-items: center;
    gap: 8px;
    padding: 5px 12px;
    border-bottom: 1px solid #1a202c;
    font-size: 12px;
  }
  .file-row:hover { background: #1a2035; }
  .icon { font-size: 14px; flex-shrink: 0; }
  .name { color: #e2e8f0; overflow: hidden; text-overflow: ellipsis; white-space: nowrap; flex: 1; }

  .remove-btn {
    background: none;
    border: none;
    color: #4a5568;
    cursor: pointer;
    font-size: 11px;
    padding: 2px 4px;
    border-radius: 3px;
    flex-shrink: 0;
    opacity: 0;
  }
  .file-row:hover .remove-btn { opacity: 1; }
  .remove-btn:hover { color: #fc8181; background: #2d1f1f; }
</style>
