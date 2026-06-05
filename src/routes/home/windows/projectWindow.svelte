<script>
    // import { projectWindow } from './projectsWindow';
    import { Spinner } from "flowbite-svelte";
    import { invoke } from "@tauri-apps/api/core";
    let state = "loading" ;
    let loadingtext = "Scanning...";
    
    // Scanning shuru ki hai 
    await 
    
</script>
<div class="window-root flex justify-center items-center">
    {#if state = "loading"}
    <!-- Keep this div full seze of avilable space and make it center i need spinner in center -->
    <div class="flex flex-col justify-center items-center w-full h-[calc(100vh-88px)]">
        <Spinner type="bars" color="#72ddc3" size="16"/>
        <span class="text-white mt-2">Scanning...</span>
    </div>
    {/if}
</div>