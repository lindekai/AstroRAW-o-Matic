<script>
  export let results;

  function basename(path) {
    return path.split(/[\\/]/).pop();
  }
</script>

<div class="results">
  <div class="summary">
    <span class="stat ok">✓ {results.succeeded}</span>
    {#if results.failed > 0}
      <span class="stat fail">✗ {results.failed}</span>
    {/if}
    <span class="stat total">of {results.total} files</span>
    <span class="tagline">
      {results.failed === 0 ? "Conversion complete. Don't panic." : "Conversion failed, but at least it failed deterministically."}
    </span>
  </div>

  <div class="file-results">
    {#each results.results as r}
      <div class="result-row {r.success ? 'ok' : 'fail'}">
        <span class="status">{r.success ? "✓" : "✗"}</span>
        <span class="name">{basename(r.input)}</span>
        {#if r.error}
          <span class="error">{r.error}</span>
        {/if}
        {#each r.warnings as w}
          <span class="warning">⚠ {w}</span>
        {/each}
      </div>
    {/each}
  </div>
</div>

<style>
  .results {
    border-top: 1px solid #2d3748;
    flex-shrink: 0;
    max-height: 180px;
    overflow-y: auto;
  }

  .summary {
    display: flex;
    align-items: center;
    gap: 10px;
    padding: 7px 12px;
    background: #1a1f2e;
    font-size: 12px;
    border-bottom: 1px solid #2d3748;
  }

  .stat { font-weight: 600; }
  .stat.ok { color: #68d391; }
  .stat.fail { color: #fc8181; }
  .stat.total { color: #718096; }
  .tagline { color: #718096; font-style: italic; font-size: 11px; margin-left: auto; }

  .file-results { padding: 4px 0; }

  .result-row {
    display: flex;
    align-items: flex-start;
    flex-wrap: wrap;
    gap: 4px;
    padding: 4px 12px;
    font-size: 12px;
    border-bottom: 1px solid #1a202c;
  }
  .result-row.ok { color: #e2e8f0; }
  .result-row.fail { color: #fc8181; }

  .status { font-weight: 700; flex-shrink: 0; }
  .name { flex-shrink: 0; }
  .error { color: #fc8181; font-size: 11px; width: 100%; padding-left: 16px; }
  .warning { color: #f6ad55; font-size: 11px; width: 100%; padding-left: 16px; }
</style>
