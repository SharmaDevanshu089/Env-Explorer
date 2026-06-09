<script>
    import ApplicationTitlebar from '../component/titlebar.svelte';
    import Icon from '../component/Icon.svelte';
    import ProjectWindow from "./windows/projectWindow.svelte"
    import AnalyticWindow from "./windows/analyticWindow.svelte"
    import BulkWindow from "./windows/bulkWindow.svelte"

    let active = "projects";
    let blur = false;
</script>
<div class="shell">
  <aside class="sidebar" data-tauri-drag-region>
    <header class="sidebar-header">
        <div class="logo">
            🚀
        </div>

        <div class="title-container" data-tauri-drag-region>
            <h1 class="title">Env Explorer</h1>
            <p class="subtitle">Local Environment Manager</p>
        </div>
    </header>

    <button class="create-btn">
        <Icon name="add" size={16} strokeWidth={2.5} />
        New Variable
    </button>

    <nav class="navigation">
        <button
            class="nav-item"
            class:active={active === 'projects'}
            on:click={() => active = 'projects'}
        >
            <Icon name="folder" />
            Projects
        </button>

        <button
            class="nav-item"
            class:active={active === 'recent'}
            on:click={() => active = 'recent'}
        >
            <Icon name="history" />
            Recent
        </button>

        <button
            class="nav-item"
            class:active={active === 'keys'}
            on:click={() => active = 'keys'}
        >
            <Icon name="key" />
            Global Keys
        </button>
    </nav>

    <footer class="footer">
        <button class="nav-item">
            <Icon name="settings" />
            Settings
        </button>

        <button class="nav-item">
            <Icon name="help" />
            Help
        </button>

        <button class="nav-item">
            <Icon name="feedback" />
            Feedback
        </button>
    </footer>
  </aside>

  <main>
    <ApplicationTitlebar />
    <div class="content">
    {#if blur}
      <div class="blur-overlay">
        {#if active === "projects"}
          <ProjectWindow />
        {:else if active === "recent"}
          <AnalyticWindow />
        {:else if active === "keys"}
          <BulkWindow />
        {/if}
      </div>
    {:else}

      <div class="no-blur-overlay">
        {#if active === "projects"}
          <ProjectWindow />
        {:else if active === "recent"}
          <AnalyticWindow />
        {:else if active === "keys"}
          <BulkWindow />
        {/if}
      </div>
    {/if}
    </div>
  </main>
  
</div>

<style>
  @import url('https://fonts.googleapis.com/css2?family=Inter:wght@300;400;500;600&display=swap');
  @import "../component/sidebar.css";

  .shell {
    display: grid;
    grid-template-columns: 260px 1fr;
    height: 100vh;
    font-family: 'Inter', 'Segoe UI Variable', 'Segoe UI', sans-serif;
    background: #191b1f;
  }

  main {
    display: flex;
    flex-direction: column;
    background: transparent;
    padding: 0;
    height: 100%;
    overflow: hidden;
  }

  .content {
    /* padding: 12px; */
    flex: 1;
    /* overflow-y: auto; */
    color: #fff;
    background-color: transparent;
  }
</style>
