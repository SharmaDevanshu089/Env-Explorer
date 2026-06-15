<!--TODO: Add a notification for error being caused -->
<script>
    // import { projectWindow } from './projectsWindow';
    import { Spinner } from "flowbite-svelte";
    import { invoke } from "@tauri-apps/api/core";
    import { onMount } from "svelte";
  import { Button } from "flowbite-svelte";
 
    let state = "loading" ;
    let loadingtext = "Scanning...";
    /** @type {any[]} */
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


    /** @param {any} env */
    async function handleLaunchTerminal(env) {
        console.log(`[Frontend] User clicked 'Terminal' button for project: ${env.name} (Path: ${env.path})`);

        const targetIndex = env_data.findIndex(item => item.path === env.path);
        if (targetIndex === -1) {
            console.warn(`[Frontend] Could not find the project with path ${env.path} in env_data list.`);
            return;
        }

        console.log(`[Frontend] Setting loading state to true for project: ${env.name}`);
        env_data[targetIndex].loading = true;
        env_data = [...env_data]; // Trigger reactivity

        try {
            console.log(`[Frontend] Starting simulated delay of 400ms to visualize spinner...`);
            await new Promise(resolve => setTimeout(resolve, 400));

            console.log(`[Frontend] Invoking backend 'launch_terminal_with_env' command with path: ${env.path}`);
            const success = await invoke("launch_terminal_with_env", { path: env.path });
            console.log(`[Frontend] Backend command returned result: ${success}`);
        } catch (error) {
            console.error(`[Frontend] Exception occurred during invoke('launch_terminal_with_env'):`, error);
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
        <Spinner type="bars" color="green" size="16"/>
        <span class="text-[var(--retro-neon)] mt-4 font-bold uppercase tracking-widest">{loadingtext}</span>
    </div>
    {/if}
    {#if state === "loaded"}
        <div class="flex flex-col gap-3 w-full max-w-7xl mx-auto p-2 pl-15 pr-15">
            <div class="flex justify-between items-center mb-2 border-b-2 border-[var(--retro-neon)] pb-3">
                <div class="p-2">
                    <h2 class="text-4xl font-bold text-[var(--retro-neon)] uppercase tracking-wider drop-shadow-[2px_2px_0px_rgba(57,255,20,0.5)] pt-8">Projects</h2>
                    <p class="text-lg text-[var(--retro-neon)]/70 uppercase font-bold mt-2">Environment configurations found on your system</p>
                </div>
                <span class="text-xs px-2.5 py-1 bg-[var(--retro-neon)]/10 text-[var(--retro-neon)] rounded-none border-2 border-[var(--retro-neon)] font-bold uppercase shadow-[2px_2px_0px_0px_var(--retro-neon)]">
                    {env_data.length} Found
                </span>
            </div>
            
            <div class="no-scroll flex flex-col gap-4 overflow-y-auto max-h-[calc(100vh-200px)] p-2">
                {#each env_data as env}
                    <div class="flex justify-between items-center p-4 bg-[var(--retro-panel)] rounded-none border-2 border-[var(--retro-neon)] shadow-[4px_4px_0px_0px_var(--retro-neon)] transition-all duration-100">
                        <div class="flex flex-col gap-1">
                            <h5 class="text-sm font-bold text-[var(--retro-neon)] uppercase tracking-wide">{env.name}</h5>
                            <p class="text-xs text-[var(--retro-neon)]/70 font-mono select-all">{env.path}</p>
                        </div>
                        <Button 
                            size="xs" 
                            disabled={env.loading}
                            onclick={() => handleLaunchTerminal(env)}
                            class="bg-transparent text-[var(--retro-neon)] border-2 border-[var(--retro-neon)] hover:bg-[var(--retro-neon)] hover:text-black rounded-none shadow-[2px_2px_0px_0px_var(--retro-neon)] hover:shadow-none hover:translate-x-[2px] hover:translate-y-[2px] uppercase font-bold transition-none w-32 flex items-center justify-center"
                        >
                            {#if env.loading}
                                <Spinner size="4" color="green" />
                            {:else}
                                Terminal {#if env.count !== undefined}({env.count}){/if}
                            {/if}
                        </Button>
                    </div>
                {/each}
            </div>
        </div>
    {/if}
</div>