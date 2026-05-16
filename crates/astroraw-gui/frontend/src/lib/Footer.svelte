<script>
  import { openUrl } from "@tauri-apps/plugin-opener";
  export let cliVersion = "0.2.0";
  export let guiVersion = "0.2.0";
  const discordUrl = "https://discord.gg/mvgC6aXY";

  function openDiscord() {
    openUrl(discordUrl);
  }
</script>

<footer>
  <div class="left">
    <span class="item">CLI v{cliVersion}</span>
    <span class="sep">·</span>
    <span class="item">GUI v{guiVersion}</span>
  </div>

  <div class="center">
    <button class="community" on:click={openDiscord}>
      🔭 Dark Matters Community
    </button>
  </div>

  <div class="right">
    <span class="item">Author: Kai Linde</span>
  </div>
</footer>

<style>
  footer {
    display: grid;
    grid-template-columns: 1fr 1fr 1fr;
    align-items: center;
    justify-items: stretch;
    padding: 0 16px;
    background: #1a1f2e;
    border-top: 1px solid #2d3748;
    flex-shrink: 0;
    height: 28px;
  }

  .left, .center, .right {
    display: flex;
    align-items: center;
    height: 100%;
  }

  .left {
    display: flex;
    align-items: center;
    gap: 6px;
  }

  .center {
    justify-content: center;
  }

  .right {
    justify-content: flex-end;
  }

  .item {
    font-size: 12px;
    color: #4a5568;
  }

  .sep {
    font-size: 12px;
    color: #2d3748;
  }

  .community {
    font-size: 12px;
    color: #63b3ed;
    background: none;
    border: none;
    cursor: pointer;
    padding: 0;
    opacity: 0.8;
    transition: opacity 0.15s;
  }

  .community:hover {
    opacity: 1;
  }
</style>
