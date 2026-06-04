<script>
    import ApplicationTitlebar from '../component/titlebar.svelte';
</script>
<div class="shell">
  <aside class="left">
    
  </aside>
  <main>
    <ApplicationTitlebar />
    <div class="content">
      <!-- Content goes here -->
    </div>
  </main>
</div>

<style>
  .shell {
    display: grid;
    grid-template-columns: 260px 1fr;
    height: 100vh;
  }

  aside {
    padding: 12px;
  }

  main {
    display: flex;
    flex-direction: column;
    background: #fff;
    padding: 0;
  }

  .content {
    padding: 20px;
    flex: 1;
    overflow-y: auto;
  }
  .left{
    background: #111317ea;
    backdrop-filter: blur(32px) saturate(1.6);
    -webkit-backdrop-filter: blur(32px) saturate(1.6);

    border-bottom: 1px solid rgba(255, 255, 255, 0.06);
    font-family: 'Inter', 'Segoe UI Variable', 'Segoe UI', sans-serif;
  }
</style>
