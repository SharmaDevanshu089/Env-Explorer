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

  function handleRefresh() {
    console.log("refresh initiate");
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
      [ ENV ]
    </div>
  </div>

  <!-- Right: Actions & Window controls -->
  <div class="titlebar__right">
    <button
      class="titlebar__action-btn"
      on:click={handleRefresh}
      on:mousedown={stopPropagation}
      aria-label="Refresh"
      title="Refresh"
    >
      R
    </button>

    <div class="titlebar__separator"></div>

    <div class="titlebar__controls">
      <button
        class="titlebar__btn titlebar__btn--minimize"
        on:click={handleMinimize}
        on:mousedown={stopPropagation}
        aria-label="Minimize"
        title="Minimize"
      >
        _
      </button>

      <button
        class="titlebar__btn titlebar__btn--maximize"
        on:click={handleMaximizeRestore}
        on:mousedown={stopPropagation}
        aria-label={isMaximized ? 'Restore' : 'Maximize'}
        title={isMaximized ? 'Restore' : 'Maximize'}
      >
        {#if isMaximized}
          [=]
        {:else}
          [ ]
        {/if}
      </button>

      <button
        class="titlebar__btn titlebar__btn--close"
        on:click={handleClose}
        on:mousedown={stopPropagation}
        aria-label="Close"
        title="Close"
      >
        X
      </button>
    </div>
  </div>
</div>

<style>
  @import "./titlebar.css";
</style>