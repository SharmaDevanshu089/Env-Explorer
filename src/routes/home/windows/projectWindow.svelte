<script>
    // import { projectWindow } from './projectsWindow';
    import { Spinner } from "flowbite-svelte";
    import { invoke } from "@tauri-apps/api/core";
    import { onMount } from "svelte";
  import { Button } from "flowbite-svelte";
 
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
            for (let env of env_data) {
                env.count = await invoke("count_env_vars", { path: env.path });
            }
            env_data = [...env_data];
            console.log("Successfully fetched env_data:", env_data);
        } catch (e) {
            console.error("Error reading env config:", e);
        }
        state = "loaded";
    }
 
    onMount(() => {
        first_update();
    });
    // TODO: Change the theme to retro this is definatly ai color scheme
</script>
<div class="window-root flex justify-center items-center">
    {#if state === "loading"}
    <!-- Keep this div full seze of avilable space and make it center i need spinner in center -->
    <div class="flex flex-col justify-center items-center w-full h-[calc(100vh-88px)]">
        <Spinner type="bars" color="#72ddc3" size="16"/>
        <span class="text-white mt-2">{loadingtext}</span>
    </div>
    {/if}
    {#if state === "loaded"}
        <div class="flex flex-col gap-3 w-full max-w-7xl mx-auto p-2 pl-15 pr-15">
            <div class="flex justify-between items-center mb-2 border-b border-white/5 pb-3">
                <div class="p-2">
                    <h2 class="text-4xl font-semibold text-white pt-8">Projects</h2>
                    <p class="text-lg text-gray-400 ">Environment configurations found on your system</p>
                </div>
                <span class="text-xs px-2.5 py-1 bg-[#72ddc3]/10 text-[#72ddc3] rounded-full border border-[#72ddc3]/20 font-medium">
                    {env_data.length} Found
                </span>
            </div>
            
            <div class="no-scroll flex flex-col gap-2 overflow-y-auto max-h-[calc(100vh-200px)]">
                {#each env_data as env}
                    <div class="flex justify-between items-center p-4 bg-[#111317] rounded-lg border border-white/5 hover:border-[#72ddc3]/30 transition-all duration-200">
                        <div class="flex flex-col gap-1">
                            <h5 class="text-sm font-semibold text-white">{env.name}</h5>
                            <p class="text-xs text-gray-400 font-mono select-all">{env.path}</p>
                        </div>
                        <Button size="xs" class="bg-[#72ddc3]/10 text-[#72ddc3] border border-[#72ddc3]/20 hover:bg-[#72ddc3] hover:text-black transition-all">
                            Load {#if env.count !== undefined}({env.count}){/if}
                        </Button>
                    </div>
                {/each}
            </div>
        </div>
    {/if}
</div>