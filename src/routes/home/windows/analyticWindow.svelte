<script>
    import { invoke } from "@tauri-apps/api/core";
    import { onMount } from "svelte";
    

    let variable_count = 0;
    let env_files = [];
    let total_variable_count = 0;
    async function get_env_list() {
        try{
            let reading_from_os = await invoke("read_env_config");
            console.log(reading_from_os);   
            env_files = reading_from_os;
        }
        catch (error){
            console.log(error);
        }
        for (const file of env_files) {
            // console.log(file.name);
            let no_of_env_variables = await invoke("count_env_vars", { path: file.path });
            // console.log(no_of_env_variables);
            total_variable_count = total_variable_count + no_of_env_variables;
        }
        // console.log(total_variable_count);
    }

    async function scan_all_files_and_return_total_count() {
        console.clear();
        for (const file of env_files) {
            // need to get variable list
            console.log(total_variable_count);
            let variables_in_current_file = await invoke("get_current_env_vars", {path: file.path});
            console.log(variables_in_current_file);
        }
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
    */
    onMount(async () => {
        get_env_list();
        scan_all_files_and_return_total_count();
    });    
</script>

<div>
    <h1>Analytic window</h1>
</div>