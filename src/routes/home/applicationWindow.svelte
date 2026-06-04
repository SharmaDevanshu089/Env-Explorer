<script>
    import ApplicationTitlebar from '../component/titlebar.svelte';
</script>
<div class="shell">
  <aside class="left">
    Hello 
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
</style>
