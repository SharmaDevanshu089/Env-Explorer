<script>
    // import { projectWindow } from './projectsWindow';
    import { Spinner } from "flowbite-svelte";
    let state = "loading" ;
    
</script>
<div class="window-root">
    {#if state = "loading"}
        <Spinner />
    {/if}
</div>