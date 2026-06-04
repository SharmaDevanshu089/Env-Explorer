<script>
    import ApplicationTitlebar from '../component/titlebar.svelte';
    import Icon from '../component/Icon.svelte';

    let active = "projects";
</script>
<div class="shell">
  <aside class="sidebar">
    <header class="sidebar-header">
        <div class="logo">
            🚀
        </div>

        <div class="title-container">
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
      <!-- Content goes here -->
    </div>
  </main>
</div>

<style>
  @import url('https://fonts.googleapis.com/css2?family=Inter:wght@300;400;500;600&display=swap');

  .shell {
    display: grid;
    grid-template-columns: 260px 1fr;
    height: 100vh;
    font-family: 'Inter', 'Segoe UI Variable', 'Segoe UI', sans-serif;
    background: #090a0c;
  }

  .sidebar {
    display: flex;
    flex-direction: column;
    
    background: #111317;
    backdrop-filter: blur(32px) saturate(1.6);
    -webkit-backdrop-filter: blur(32px) saturate(1.6);
    border-right: 1px solid rgba(255, 255, 255, 0.06);
    
    padding: 24px 16px;
  }

  .sidebar-header {
    display: flex;
    align-items: center;
    gap: 12px;
    margin-bottom: 32px;
    padding: 0 4px;
  }

  .logo {
    width: 36px;
    height: 36px;
    display: flex;
    align-items: center;
    justify-content: center;
    border-radius: 8px;
    background: rgba(114, 221, 195, 0.15);
    color: #72ddc3;
    font-size: 18px;
  }

  .title-container {
    display: flex;
    flex-direction: column;
    gap: 2px;
  }

  .title {
    margin: 0;
    color: #ffffff;
    font-size: 14px;
    font-weight: 600;
    letter-spacing: 0.3px;
  }

  .subtitle {
    margin: 0;
    font-size: 11px;
    color: rgba(255, 255, 255, 0.5);
    font-weight: 400;
  }

  .create-btn {
    display: flex;
    align-items: center;
    justify-content: center;
    gap: 8px;
    
    width: 100%;
    padding: 10px;
    margin-bottom: 24px;
    
    border: 1px solid rgba(114, 221, 195, 0.2);
    border-radius: 6px;
    
    background: rgba(114, 221, 195, 0.1);
    color: #72ddc3;
    
    font-size: 13px;
    font-weight: 500;
    cursor: pointer;
    transition: all 0.2s ease;
  }

  .create-btn:hover {
    background: rgba(114, 221, 195, 0.15);
    border-color: rgba(114, 221, 195, 0.3);
  }

  .create-btn :global(svg) {
    flex-shrink: 0;
  }

  .navigation {
    display: flex;
    flex-direction: column;
    gap: 4px;
    flex: 1;
  }

  .nav-item {
    display: flex;
    align-items: center;
    gap: 12px;
    
    padding: 10px 12px;
    
    border: none;
    border-radius: 6px;
    
    background: transparent;
    color: rgba(255, 255, 255, 0.65);
    
    font-size: 13px;
    font-weight: 500;
    text-align: left;
    cursor: pointer;
    transition: all 0.15s ease;
  }

  .nav-item:hover {
    background: rgba(255, 255, 255, 0.05);
    color: rgba(255, 255, 255, 0.9);
  }

  .nav-item.active {
    background: rgba(255, 255, 255, 0.08);
    color: #ffffff;
  }

  .nav-item :global(svg) {
    opacity: 0.8;
    transition: color 0.15s ease, opacity 0.15s ease;
    flex-shrink: 0;
  }

  .nav-item.active :global(svg) {
    color: #72ddc3;
    opacity: 1;
  }

  .footer {
    display: flex;
    flex-direction: column;
    gap: 4px;
    margin-top: auto;
    padding-top: 16px;
    border-top: 1px solid rgba(255, 255, 255, 0.06);
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
    padding: 24px;
    flex: 1;
    overflow-y: auto;
    color: #fff;
  }
</style>
