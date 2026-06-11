<script>
    import { invoke } from "@tauri-apps/api/core";
    import { onMount } from "svelte";
    // Funtion to get total number of Variables
    let variable_count = 0;
    let env_files = [];
    let count = 0;
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
            console.log(file.name);
            let no_of_env_variables = await invoke("count_env_vars", { path: file.path });
            console.log(no_of_env_variables);
            count = count + no_of_env_variables;
        }
        console.log(count);
    }


    onMount(async () => {get_env_list();});    
</script>

<div>
    <h1>Analytic window</h1>
</div>