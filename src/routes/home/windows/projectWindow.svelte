<script>
    // import { projectWindow } from './projectsWindow';
    import { Spinner } from "flowbite-svelte";
    import { invoke } from "@tauri-apps/api/core";
    import { onMount } from "svelte";
    let state = "loading" ;
    let loadingtext = "Scanning...";
    let env_data = [];
    
    async function first_update() {
        // Scanning shuru ki hai 
        await invoke("intiate_crawl");
        // Wait one second
        await new Promise(resolve => setTimeout(resolve, 1000));
        loadingtext = "Updating Index...";
        
        try {
            env_data = await invoke("read_env_config");
            console.log("Successfully fetched env_data:", env_data);
        } catch (e) {
            console.error("Error reading env config:", e);
        }
        
        state = "loaded";
    }

    onMount(() => {
        first_update();
    });
    
</script>
<div class="window-root flex justify-center items-center">
    {#if state === "loading"}
    <!-- Keep this div full seze of avilable space and make it center i need spinner in center -->
    <div class="flex flex-col justify-center items-center w-full h-[calc(100vh-88px)]">
        <Spinner type="bars" color="#72ddc3" size="16"/>
        <span class="text-white mt-2">{loadingtext}</span>
    </div>
    {/if}
</div>