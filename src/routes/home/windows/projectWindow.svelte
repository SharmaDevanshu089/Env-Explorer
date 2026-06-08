<!--TODO: Add a notification for error being caused -->
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
        console.log("[Frontend] first_update() initiated. UI state: loading.");
        
        try {
            console.log("[Frontend] Invoking backend initiate_crawl...");
            await invoke("intiate_crawl");
            console.log("[Frontend] backend initiate_crawl invocation resolved successfully.");
        } catch (e) {
            console.error("[Frontend] backend initiate_crawl invocation failed:", e);
        }
        
        console.log("[Frontend] Waiting 1000ms delay to simulate progress/allow disk sync...");
        await new Promise(resolve => setTimeout(resolve, 1000));
        loadingtext = "Updating Index...";
        console.log("[Frontend] loadingtext updated to: " + loadingtext);
        
        try {
            console.log("[Frontend] Invoking backend read_env_config...");
            env_data = await invoke("read_env_config");
            console.log(`[Frontend] read_env_config returned ${env_data.length} project items:`, env_data);
            
            for (let i = 0; i < env_data.length; i++) {
                let env = env_data[i];
                console.log(`[Frontend] [Item ${i + 1}/${env_data.length}] Requesting variable count for path: ${env.path}`);
                env.count = await invoke("count_env_vars", { path: env.path });
                env.loading = false;
                env.loaded = false;
                console.log(`[Frontend] [Item ${i + 1}/${env_data.length}] Count received: ${env.count}`);
            }
            
            env_data = [...env_data];
            console.log("[Frontend] Successfully built reactive env_data array with counts and tracking states.");
        } catch (e) {
            console.error("[Frontend] Error reading env config or fetching counts:", e);
        }
        
        state = "loaded";
        console.log("[Frontend] state transitioned to loaded. Ready to render workspace.");
    }


    async function handleLoadEnv(env) {
        console.log(`[Frontend] User clicked 'Load' button for project: ${env.name} (Path: ${env.path})`);

        const targetIndex = env_data.findIndex(item => item.path === env.path);
        if (targetIndex === -1) {
            console.warn(`[Frontend] Could not find the project with path ${env.path} in env_data list.`);
            return;
        }

        console.log(`[Frontend] Setting loading state to true for project: ${env.name}`);
        env_data[targetIndex].loading = true;
        env_data = [...env_data]; // Trigger reactivity

        try {
            console.log(`[Frontend] Starting simulated delay of 600ms to visualize spinner...`);
            await new Promise(resolve => setTimeout(resolve, 600));

            console.log(`[Frontend] Invoking backend 'load_env_to_system_process' command with path: ${env.path}`);
            const success = await invoke("load_env_to_system_process", { path: env.path });
            console.log(`[Frontend] Backend command returned result: ${success}`);

            if (success) {
                console.log(`[Frontend] Loading succeeded! Setting loaded = true and disabling button for: ${env.name}`);
                env_data[targetIndex].loaded = true;
            } else {
                console.warn(`[Frontend] Loading returned false (non-success status) for: ${env.name}`);
            }
        } catch (error) {
            console.error(`[Frontend] Exception occurred during invoke('load_env_to_system_process'):`, error);
        } finally {
            console.log(`[Frontend] Resetting loading state to false for project: ${env.name}`);
            env_data[targetIndex].loading = false;
            env_data = [...env_data]; // Trigger reactivity
        }
    }
 
    onMount(() => {
        console.log("[Frontend] projectWindow.svelte component has mounted successfully in the DOM.");
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
                        <Button 
                            size="xs" 
                            disabled={env.loading || env.loaded}
                            onclick={() => handleLoadEnv(env)}
                            class="bg-[#72ddc3]/10 text-[#72ddc3] border border-[#72ddc3]/20 hover:bg-[#72ddc3] hover:text-black transition-all w-24 flex items-center justify-center"
                        >
                            {#if env.loading}
                                <Spinner size="4" color="white" />
                            {:else if env.loaded}
                                Loaded
                            {:else}
                                Load {#if env.count !== undefined}({env.count}){/if}
                            {/if}
                        </Button>
                    </div>
                {/each}
            </div>
        </div>
    {/if}
</div>