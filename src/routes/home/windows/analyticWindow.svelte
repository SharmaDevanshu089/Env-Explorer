<script>
    import { invoke } from "@tauri-apps/api/core";
    import { onMount } from "svelte";

    // Tells the maximum number of Items to be Added In the bar chart and the cutoff value
    const MAX_CHART_VALUE = 10;
    const CLAMP_VALUE = 0;

    let variable_count = 0;
    let env_files = [];
    let total_variable_count = 0;
    let master_envirment_list = [];
    let final_master_envirment_count_list = {};
    let temp_array = {};
    let cleanlist = {};

    async function get_env_list() {
        try {
            let reading_from_os = await invoke("read_env_config");
            console.log(reading_from_os);
            env_files = reading_from_os;
        } catch (error) {
            console.log(error);
        }
        for (const file of env_files) {
            // console.log(file.name);
            let no_of_env_variables = await invoke("count_env_vars", {
                path: file.path,
            });
            // console.log(no_of_env_variables);
            total_variable_count = total_variable_count + no_of_env_variables;
        }
        // this is being printed that means function is probably ending

        // why is this being called every envirment
        // Update: it Wasnt , i was just logging it : )
        console.log(total_variable_count);
    }

    async function initiate_giant_pile() {
        for (const file of env_files) {
            let variables_in_current_file = await invoke(
                "get_current_env_vars",
                { path: file.path },
            );

            for (let envirment_variable in variables_in_current_file) {
                // How is only key being printited here

                // console.log(envirment_variable);
                // Answer: Javascript only iterates over keys, this just got 100,000 times easier
                master_envirment_list.push(envirment_variable);
            }
            // console.log(variables_in_current_file);
        }
    }

    async function implement_count() {
        for (let item of master_envirment_list) {
            // console.log(item);
            if (final_master_envirment_count_list[item] == undefined) {
                final_master_envirment_count_list[item] = 1;
            } else {
                final_master_envirment_count_list[item] =
                    final_master_envirment_count_list[item] + 1;
            }
        }
    }

    async function scan_all_files_and_return_total_count() {
        // mybe this function is not being called
        // Update: this funtion is being called befor ethe get_env_list, ok maybe i should use await
        console.log("scan_all_files_and_return_total_count got called");
        // console.clear();
        for (const file of env_files) {
            // need to get variable list
            // console.log(total_variable_count);
            let variables_in_current_file = await invoke(
                "get_current_env_vars",
                { path: file.path },
            );
            // why is this friking not working?
            // above funtion is not called
            // Udpate: Fixed the issue
            console.log(variables_in_current_file);
        }
    }

    async function sort() {
        temp_array = Object.entries(final_master_envirment_count_list);

        temp_array.sort(function (cardA, cardB) {
            // Remember, on our card: position [0] is the Name, position [1] is the Number
            let numberA = cardA[1];
            let numberB = cardB[1];

            // By subtracting A from B, we tell the machine: "Biggest numbers go first!"
            return numberB - numberA;
        });
        final_master_envirment_count_list = Object.fromEntries(temp_array);
        console.log(final_master_envirment_count_list);
    }

    async function build_clean_list() {
        // There is something seriously wrong here, the length value is overly inflated idk
        // But it is bit smaller then the garbage_pile value
        // I think it is including both the copunt and name of variable in there
        console.log("Length = " + temp_array.length);

        // We make sure cleanlist starts totally empty
        cleanlist = {};

        // Use .entries() to get the index and the card at the same time
        for (let [index, card] of temp_array.entries()) {
            // 1. Enforce the limit: If index hits 10, destroy the loop!
            if (index >= MAX_CHART_VALUE) {
                break;
            }

            let variableName = card[0];
            let variableCount = card[1];

            // 2. Enforce the clamp: Only add it if it's greater than CLAMP_VALUE
            if (variableCount > CLAMP_VALUE) {
                // Write it onto our clean, final scoreboard
                cleanlist[variableName] = variableCount;
            }
        }

        console.log("Final Clean Dashboard Data:", cleanlist);
    }

    /**
     * Load ->
     * Look for a file
     * loop start, file 1, var 1
     * select first variable in file
     * new loop
     * look file 1, search variable
     * then file 2 ..
     * update count
     * end loop
     * select 2nd variable in file
     * do as above
     * do all variable
     * file 2 variable one
     * do all files
     * end and return count
     *
     * Update , now i am thinking of the vomit method
     * Load all file names to a single Object
     * Use Object[item] = undifined to check if it exists
     * if it doesnt, update the count to 1
     * if it does exist , set it to count + 1
     * then move to next variable
     * and do this till eternity
     *
     */
    onMount(async () => {
        console.clear();
        await get_env_list();
        await initiate_giant_pile();
        await implement_count();
        await sort();
        // console.log(final_master_envirment_count_list);
        await build_clean_list();
        // scan_all_files_and_return_total_count();
    });
</script>

<div>
    <h1>Analytic window</h1>
</div>
