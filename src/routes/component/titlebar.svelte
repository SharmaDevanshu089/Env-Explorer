<script>
  import { onMount } from 'svelte';
  import { getCurrentWindow } from '@tauri-apps/api/window';

  let isMaximized = false;
  /** @type {any} */
  let appWindow;

  onMount(() => {
    appWindow = getCurrentWindow();
  });

  async function handleMinimize() {
    await appWindow.minimize();
  }

  async function handleMaximizeRestore() {
    const maximized = await appWindow.isMaximized();
    if (maximized) {
      await appWindow.unmaximize();
      isMaximized = false;
    } else {
      await appWindow.maximize();
      isMaximized = true;
    }
  }

  async function handleClose() {
    await appWindow.close();
  }

  async function handleDoubleClick() {
    await handleMaximizeRestore();
  }

  /** @param {Event} e */
  function stopPropagation(e) {
    e.stopPropagation();
  }
</script>

<!-- svelte-ignore a11y-no-static-element-interactions -->
<div
  class="titlebar"
  data-tauri-drag-region
  on:dblclick={handleDoubleClick}
>
  <!-- Left: App identity -->
  <div class="titlebar__left" data-tauri-drag-region>
    <div class="titlebar__app-icon">
      <!-- Icon placeholder -->
    </div>
    <span class="titlebar__app-name">Environment Explorer</span>
  </div>

  <!-- Right: Window controls -->
  <div class="titlebar__controls">
    <button
      class="titlebar__btn titlebar__btn--minimize"
      on:click={handleMinimize}
      on:mousedown={stopPropagation}
      aria-label="Minimize"
      title="Minimize"
    >
      <!-- Minimize Icon -->
      <svg width="10" height="10" viewBox="0 0 10 1" xmlns="http://www.w3.org/2000/svg">
        <rect width="10" height="1" fill="currentColor" />
      </svg>
    </button>

    <button
      class="titlebar__btn titlebar__btn--maximize"
      on:click={handleMaximizeRestore}
      on:mousedown={stopPropagation}
      aria-label={isMaximized ? 'Restore' : 'Maximize'}
      title={isMaximized ? 'Restore' : 'Maximize'}
    >
      {#if isMaximized}
        <!-- Restore Icon -->
        <svg width="10" height="10" viewBox="0 0 10 10" xmlns="http://www.w3.org/2000/svg">
          <path d="M3 0v3H0v7h7V7h3V0H3zm6 6H7V3H4V1h5v5zM6 9H1V4h5v5z" fill="currentColor" />
        </svg>
      {:else}
        <!-- Maximize Icon -->
        <svg width="10" height="10" viewBox="0 0 10 10" xmlns="http://www.w3.org/2000/svg">
          <path d="M0 0v10h10V0H0zm9 9H1V1h8v8z" fill="currentColor" />
        </svg>
      {/if}
    </button>

    <button
      class="titlebar__btn titlebar__btn--close"
      on:click={handleClose}
      on:mousedown={stopPropagation}
      aria-label="Close"
      title="Close"
    >
      <!-- Close Icon -->
      <svg width="10" height="10" viewBox="0 0 10 10" xmlns="http://www.w3.org/2000/svg">
        <path
          d="M1.354.646a.5.5 0 0 0-.708.708L4.293 5 .646 8.646a.5.5 0 0 0 .708.708L5 5.707l3.646 3.647a.5.5 0 0 0 .708-.708L5.707 5l3.647-3.646a.5.5 0 0 0-.708-.708L5 4.293 1.354.646z"
          fill="currentColor"
        />
      </svg>
    </button>
  </div>
</div>

<style>
  @import url('https://fonts.googleapis.com/css2?family=Inter:wght@300;400;500;600&display=swap');

  .titlebar {
    display: flex;
    align-items: center;
    justify-content: space-between;

    height: 40px;
    width: 100%;
    padding: 0px 5px;
    margin: 0;
    box-sizing: border-box;

    background: rgb(23, 23, 26);
    backdrop-filter: blur(32px) saturate(1.6);
    -webkit-backdrop-filter: blur(32px) saturate(1.6);

    border-bottom: 1px solid rgba(255, 255, 255, 0.06);
    font-family: 'Inter', 'Segoe UI Variable', 'Segoe UI', sans-serif;

    user-select: none;
    -webkit-user-select: none;

    position: relative;
    z-index: 9999;
  }

  /* ── Left — App identity ── */

  .titlebar__left {
    display: flex;
    align-items: center;
    gap: 8px;
    padding-left: 12px;
    justify-self: start;
  }

  .titlebar__app-icon {
    display: flex;
    align-items: center;
    justify-content: center;
    width: 20px;
    height: 20px;
    color: rgba(255, 255, 255, 0.85);
    flex-shrink: 0;
    opacity: 0.9;
  }

  .titlebar__app-name {
    font-size: 16px;
    font-weight: 500;
    color: rgba(255, 255, 255, 0.72);
    letter-spacing: 0.01em;
    white-space: nowrap;
  }

  /* ── Right — Window controls ── */

  .titlebar__controls {
    display: flex;
    align-items: stretch;
    justify-self: end;
    height: 100%;
  }

  .titlebar__btn {
    display: flex;
    align-items: center;
    justify-content: center;

    width: 46px;
    height: 100%;
    padding: 0;
    margin: 0;

    background: transparent;
    border: none;
    outline: none;
    cursor: pointer;

    color: rgba(255, 255, 255, 0.78);
    transition: background 0.1s ease, color 0.1s ease;
  }

  .titlebar__btn--minimize:hover,
  .titlebar__btn--maximize:hover {
    background: rgba(255, 255, 255, 0.08);
    color: rgba(255, 255, 255, 1);
  }

  .titlebar__btn--minimize:active,
  .titlebar__btn--maximize:active {
    background: rgba(255, 255, 255, 0.05);
  }

  .titlebar__btn--close:hover {
    background: #c42b1c;
    color: #ffffff;
  }

  .titlebar__btn--close:active {
    background: #b22318;
    color: rgba(255, 255, 255, 0.88);
  }

  .titlebar__btn :global(svg) {
    display: block;
  }
</style>