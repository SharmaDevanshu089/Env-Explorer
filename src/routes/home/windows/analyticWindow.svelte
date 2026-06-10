<script>
    import { invoke } from "@tauri-apps/api/core";
    import { onMount } from "svelte";
    // Funtion to get total number of Variables
    let variable_count = 0;
    let env_files = [];
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
            let
        }
    }


    onMount(async () => {get_env_list();});    
</script>

<div>
    <h1>Analytic window</h1>
</div>