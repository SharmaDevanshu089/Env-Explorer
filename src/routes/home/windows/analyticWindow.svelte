<script>
    import { invoke } from "@tauri-apps/api/core";
    import { onMount } from "svelte";
    import { Spinner } from "flowbite-svelte";

    // Tells the maximum number of Items to be Added In the bar chart and the cutoff value
    const MAX_CHART_VALUE = 10;
    const CLAMP_VALUE = 0;

    let state = "loading";
    let loadingtext = "Scanning & Analyzing environment variables...";
    let variable_count = 0;
    /** @type {any[]} */
    let env_files = [];
    let total_variable_count = 0;
    /** @type {string[]} */
    let master_envirment_list = [];
    /** @type {Record<string, number>} */
    let final_master_envirment_count_list = {};
    /** @type {[string, number][]} */
    let temp_array = [];
    /** @type {[string, number][]} */
    let cleanlist = [];

    async function get_env_list() {
        try {
            let reading_from_os = await invoke("read_env_config");
            console.log(reading_from_os);
            env_files = reading_from_os;
        } catch (error) {
            console.log(error);
        }
        for (const file of env_files) {
            let no_of_env_variables = await invoke("count_env_vars", {
                path: file.path,
            });
            total_variable_count = total_variable_count + no_of_env_variables;
        }
        console.log(total_variable_count);
    }

    async function initiate_giant_pile() {
        for (const file of env_files) {
            let variables_in_current_file = await invoke(
                "get_current_env_vars",
                { path: file.path },
            );

            for (let envirment_variable in variables_in_current_file) {
                master_envirment_list.push(envirment_variable);
            }
        }
    }

    async function implement_count() {
        for (let item of master_envirment_list) {
            if (final_master_envirment_count_list[item] == undefined) {
                final_master_envirment_count_list[item] = 1;
            } else {
                final_master_envirment_count_list[item] =
                    final_master_envirment_count_list[item] + 1;
            }
        }
    }

    async function sort() {
        temp_array = Object.entries(final_master_envirment_count_list);

        temp_array.sort(function (cardA, cardB) {
            let numberA = cardA[1];
            let numberB = cardB[1];
            return numberB - numberA;
        });
        final_master_envirment_count_list = Object.fromEntries(temp_array);
        console.log(final_master_envirment_count_list);
    }

    async function build_clean_list() {
        console.log("Length = " + temp_array.length);
        cleanlist = [];

        for (let [index, card] of temp_array.entries()) {
            if (index >= MAX_CHART_VALUE) {
                break;
            }

            let variableName = card[0];
            let variableCount = card[1];

            if (variableCount > CLAMP_VALUE) {
                cleanlist.push([variableName, variableCount]);
            }
        }
        cleanlist = cleanlist; // Trigger reactivity
        console.log("Final Clean Dashboard Data:", cleanlist);
    }

    $: maxCount = cleanlist.length > 0 ? cleanlist[0][1] : 1;

    onMount(async () => {
        console.clear();
        state = "loading";
        await get_env_list();
        await initiate_giant_pile();
        await implement_count();
        await sort();
        await build_clean_list();
        state = "loaded";
    });
</script>

<div class="window-root flex justify-center items-start overflow-y-auto h-[calc(100vh-88px)] w-full no-scroll">
    {#if state === "loading"}
        <div class="flex flex-col justify-center items-center w-full h-[calc(100vh-88px)]">
            <Spinner type="bars" color="teal" size="16"/>
            <span class="text-white mt-2 font-medium">{loadingtext}</span>
        </div>
    {/if}
    {#if state === "loaded"}
        <div class="flex flex-col gap-6 w-full max-w-7xl mx-auto p-2 pl-15 pr-15 pb-12">
            <!-- Header -->
            <div class="flex justify-between items-end mb-2 border-b border-white/5 pb-3">
                <div class="p-2">
                    <h2 class="text-4xl font-semibold text-white pt-8">Analytics</h2>
                    <p class="text-lg text-gray-400">Environment variable distribution and insights</p>
                </div>
                <div class="p-2 flex gap-2">
                    <span class="text-xs px-2.5 py-1 bg-[#72ddc3]/10 text-[#72ddc3] rounded-full border border-[#72ddc3]/20 font-medium">
                        {env_files.length} Files Scanned
                    </span>
                </div>
            </div>

            <!-- Stats Grid -->
            <div class="grid grid-cols-1 md:grid-cols-3 gap-4 mb-2">
                <div class="bg-[#111317] p-5 rounded-lg border border-white/5 flex flex-col gap-1 hover:border-[#72ddc3]/20 transition-all duration-200">
                    <span class="text-xs text-gray-400 uppercase tracking-wider font-semibold">Total Scanned Files</span>
                    <span class="text-3xl font-bold text-white mt-1">{env_files.length}</span>
                </div>
                <div class="bg-[#111317] p-5 rounded-lg border border-white/5 flex flex-col gap-1 hover:border-[#72ddc3]/20 transition-all duration-200">
                    <span class="text-xs text-gray-400 uppercase tracking-wider font-semibold">Total Defined Variables</span>
                    <span class="text-3xl font-bold text-white mt-1">{total_variable_count}</span>
                </div>
                <div class="bg-[#111317] p-5 rounded-lg border border-white/5 flex flex-col gap-1 hover:border-[#72ddc3]/20 transition-all duration-200">
                    <span class="text-xs text-gray-400 uppercase tracking-wider font-semibold">Unique Variable Names</span>
                    <span class="text-3xl font-bold text-white mt-1">{temp_array.length}</span>
                </div>
            </div>

            <!-- Top Occurrences (Bar Chart) -->
            <div class="flex flex-col gap-3">
                <h3 class="text-xl font-semibold text-white">Top Variable Occurrences</h3>
                <div class="bg-[#111317] p-6 rounded-lg border border-white/5 flex flex-col gap-4">
                    {#if cleanlist.length === 0}
                        <p class="text-sm text-gray-400">No variable occurrence data available.</p>
                    {:else}
                        {#each cleanlist as [variableName, count]}
                            <div class="flex items-center gap-4">
                                <div class="w-48 text-right text-xs font-mono text-gray-300 truncate select-all" title={variableName}>
                                    {variableName}
                                </div>
                                <div class="flex-1 h-3 bg-white/5 rounded-full overflow-hidden">
                                    <div
                                        class="h-full bg-[#72ddc3] rounded-full transition-all duration-500"
                                        style="width: {(count / maxCount) * 100}%"
                                    ></div>
                                </div>
                                <div class="w-20 text-xs font-bold text-[#72ddc3] font-mono">
                                    {count} {count === 1 ? 'file' : 'files'}
                                </div>
                            </div>
                        {/each}
                    {/if}
                </div>
            </div>

            <!-- Complete Variable Inventory List -->
            <div class="flex flex-col gap-3">
                <div class="flex justify-between items-center">
                    <h3 class="text-xl font-semibold text-white">All Environment Variables</h3>
                    <span class="text-xs text-gray-400 font-mono font-semibold">{temp_array.length} total entries</span>
                </div>
                <div class="no-scroll flex flex-col gap-2 overflow-y-auto max-h-[350px] pr-1">
                    {#if temp_array.length === 0}
                        <p class="text-sm text-gray-400 bg-[#111317] p-4 rounded-lg border border-white/5">No environment variables found.</p>
                    {:else}
                        {#each temp_array as [variableName, count]}
                            <div class="flex justify-between items-center p-3 bg-[#111317] rounded-lg border border-white/5 hover:border-[#72ddc3]/30 transition-all duration-200">
                                <span class="text-sm font-semibold text-white font-mono select-all truncate max-w-2xl" title={variableName}>
                                    {variableName}
                                </span>
                                <span class="text-xs px-2.5 py-1 bg-[#72ddc3]/10 text-[#72ddc3] rounded-full border border-[#72ddc3]/20 font-medium font-mono">
                                    {count} {count === 1 ? 'occurrence' : 'occurrences'}
                                </span>
                            </div>
                        {/each}
                    {/if}
                </div>
            </div>
        </div>
    {/if}
</div>
