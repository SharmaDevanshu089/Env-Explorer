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
</script>
<div class="window-root flex justify-center items-start overflow-y-auto h-[calc(100vh-88px)] w-full no-scroll">
    {#if state === "loading"}
    <!-- Keep this div full seze of avilable space and make it center i need spinner in center -->
    <div class="flex flex-col justify-center items-center w-full h-[calc(100vh-88px)]">
        <Spinner type="bars" color="purple" size="16"/>
        <span class="text-[var(--cat-subtext)] mt-4 font-medium tracking-wide">{loadingtext}</span>
    </div>
    {/if}
    {#if state === "loaded"}
        <div class="flex flex-col gap-3 w-full max-w-7xl mx-auto p-2 pl-15 pr-15">
            <div class="flex justify-between items-center mb-2 border-b border-[var(--cat-surface0)] pb-3">
                <div class="p-2">
                    <h2 class="text-4xl font-bold text-[var(--cat-text)] tracking-wider pt-8">Projects</h2>
                    <p class="text-lg text-[var(--cat-subtext)] font-medium mt-2">Environment configurations found on your system</p>
                </div>
                <span class="text-xs px-2.5 py-1 bg-[var(--cat-surface0)] text-[var(--cat-mauve)] rounded-lg border border-[var(--cat-surface1)] font-medium">
                    {env_data.length} Found
                </span>
            </div>
            
            <div class="no-scroll flex flex-col gap-4 overflow-y-auto max-h-[calc(100vh-200px)] p-2">
                {#each env_data as env}
                    <div class="flex justify-between items-center p-5 bg-[var(--cat-mantle)] rounded-xl border border-[var(--cat-surface0)] shadow-sm hover:shadow-md transition-all duration-200">
                        <div class="flex flex-col">
                            <span class="text-lg font-bold text-[var(--cat-text)]">{env.name}</span>
                            <span class="text-sm text-[var(--cat-subtext)]">{env.path}</span>
                        </div>
                        <button 
                            on:click={() => handleLaunchTerminal(env)}
                            class="bg-[var(--cat-surface0)] text-[var(--cat-text)] hover:bg-[var(--cat-surface1)] hover:text-[var(--cat-mauve)] border border-[var(--cat-surface1)] rounded-lg shadow-sm font-medium transition-all duration-200 w-32 py-2 flex items-center justify-center gap-2"
                        >
                            {#if env.loading}
                                <Spinner size="4" color="purple" />
                            {:else}
                                Terminal {#if env.count !== undefined}({env.count}){/if}
                            {/if}
                        </button>
                    </div>
                {/each}
            </div>
        </div>
    {/if}
</div>