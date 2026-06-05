<script>
    // import { projectWindow } from './projectsWindow';
    let state = "loading" ;//loading, empty, ready, 
    
</script>
<div class="window-root">
    {#if state = "loading"}
        Loading
    {/if}
</div>