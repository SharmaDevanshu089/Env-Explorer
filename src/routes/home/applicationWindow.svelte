<!-- Sidebar with Project, Analytics, and Bulk Change windows -->
<script>
    import ApplicationTitlebar from '../component/titlebar.svelte';
    import Icon from '../component/Icon.svelte';
    import ProjectWindow from "./windows/projectWindow.svelte"
    import AnalyticWindow from "./windows/analyticWindow.svelte"
    import BulkWindow from "./windows/bulkWindow.svelte"
    import { Modal, Button, Label, Input, Radio, Spinner } from 'flowbite-svelte';
    import { invoke } from "@tauri-apps/api/core";

    let active = "projects";
    let blur = false;
    let varName = "";
    let varValue = "";
    let submitting = false;

    function initialte_adding_variables() {
      varName = "";
      varValue = "";
      submitting = false;
      blur = true;
      console.log("Enabling Blur");
    }
    async function handleSubmit(event) {
      event.preventDefault();
      console.log("Form submitted:", { varName, varValue });
      
      submitting = true;
      try {
        console.log("[Frontend] Invoking backend add_user_env_var...");
        const success = await invoke("add_user_env_var", { key: varName, value: varValue });
        console.log("[Frontend] add_user_env_var response:", success);
        if (success) {
          blur = false;
        }
      } catch (error) {
        console.log("[Frontend] Error adding user environment variable:", error);
      } finally {
        submitting = false;
      }
    }
</script>
<div class="shell">
  <aside class="sidebar" data-tauri-drag-region>
    <header class="sidebar-header">
        <div class="logo">
            🚀
        </div>

        <div class="title-container" data-tauri-drag-region>
            <h1 class="title">Env Explorer</h1>
            <p class="subtitle">Local Environment Manager</p>
        </div>
    </header>

    <button class="create-btn" on:click={() => initialte_adding_variables()}>
        <Icon name="add" size={16} strokeWidth={2.5} />
        New Variable
    </button>

    <nav class="navigation">
        <button
            class="nav-item"
            class:active={active === 'projects'}
            on:click={() => active = 'projects'}
        >
            <Icon name="folder" />
            Projects
        </button>

        <button
            class="nav-item"
            class:active={active === 'analytics'}
            on:click={() => active = 'analytics'}
        >
            <Icon name="analytics" />
            Analytics
        </button>

        <button
            class="nav-item"
            class:active={active === 'bulk'}
            on:click={() => active = 'bulk'}
        >
            <Icon name="bulk" />
            Bulk Change
        </button>
    </nav>

    <footer class="footer">
        <button class="nav-item">
            <Icon name="settings" />
            Settings
        </button>

        <button class="nav-item">
            <Icon name="help" />
            Help
        </button>

        <button class="nav-item">
            <Icon name="feedback" />
            Feedback
        </button>
    </footer>
  </aside>

  <main>
    <ApplicationTitlebar />
    <div class="content">
    <!-- TODO: Fix colors -->
    {#if blur}
      <Modal 
        form 
        bind:open={blur} 
        outsideclose={false} 
        onsubmit={handleSubmit}
        class="!bg-[#111317] !dark:bg-[#111317] border border-white/10 rounded-lg shadow-xl backdrop:bg-black/60 text-white"
        bodyClass="p-6 space-y-6 !bg-[#111317] rounded-lg"
        dialogClass="fixed top-0 start-0 end-0 h-modal md:h-full w-full inset-0 z-50 flex items-center justify-center p-4"
        closeBtnClass="text-gray-400 hover:text-white cursor-pointer absolute top-4 end-4"
      >
        {#snippet header()}
          <div class="flex items-center gap-2">
            <div class="p-1.5 bg-[#111317]/10 text-[#72ddc3] rounded">
              <Icon name="add" size={20} strokeWidth={2} />
            </div>
            <h3 class="text-xl font-semibold text-white">Load New Environment Variable</h3>
          </div>
        {/snippet}

        <div class="space-y-4">
          <div>
            <Label for="var-name" class="block mb-2 text-sm font-medium text-gray-300">Variable Name</Label>
            <Input 
              id="var-name" 
              type="text" 
              placeholder="e.g. DATABASE_URL" 
              bind:value={varName} 
              required 
              class="w-full bg-[#111317] border border-white/10 text-white placeholder-gray-500 rounded-md focus:ring-[#72ddc3] focus:border-[#72ddc3] p-2.5"
            />
          </div>

          <div>
            <Label for="var-value" class="block mb-2 text-sm font-medium text-gray-300">Variable Value</Label>
            <Input 
              id="var-value" 
              type="text" 
              placeholder="e.g. postgresql://user:pass@localhost:5432/db" 
              bind:value={varValue} 
              required 
              class="w-full bg-[#111317] border border-white/10 text-white placeholder-gray-500 rounded-md focus:ring-[#72ddc3] focus:border-[#72ddc3] p-2.5"
            />
          </div>
        </div>

        {#snippet footer()}
          <div class="flex justify-end gap-3 w-full border-t border-white/5 pt-4">
            <Button 
              color="alternative" 
              onclick={() => blur = false}
              class="bg-transparent border border-white/10 hover:bg-white/5 text-gray-300 hover:text-white cursor-pointer"
            >
              Cancel
            </Button>
            <Button 
              type="submit" 
              disabled={submitting}
              class="bg-[#72ddc3] hover:bg-[#5ec4ad] text-black font-semibold px-5 cursor-pointer flex items-center justify-center min-w-[80px]"
            >
              {#if submitting}
                <Spinner size="4" color="current" />
              {:else}
                Load
              {/if}
            </Button>
          </div>
        {/snippet}
      </Modal>
      <div class="blur-overlay">
        {#if active === "projects"}
          <ProjectWindow />
        {:else if active === "analytics"}
          <AnalyticWindow />
        {:else if active === "bulk"}
          <BulkWindow />
        {/if}
      </div>
    {:else}

      <div class="no-blur-overlay">
        {#if active === "projects"}
          <ProjectWindow />
        {:else if active === "analytics"}
          <AnalyticWindow />
        {:else if active === "bulk"}
          <BulkWindow />
        {/if}
      </div>
    {/if}
    </div>
  </main>
  
</div>

<style>
  @import url('https://fonts.googleapis.com/css2?family=Inter:wght@300;400;500;600&display=swap');
  @import "../component/sidebar.css";

  :global(button), :global(input[type="radio"]), :global(label), :global(.cursor-pointer) {
    cursor: pointer !important;
  }

  :global(dialog), :global(dialog form) {
    background: #111317 !important;
    background-color: #111317 !important;
    border: 1px solid rgba(255, 255, 255, 0.1) !important;
  }

  :global(dialog *), :global(dialog div) {
    border-color: rgba(255, 255, 255, 0.1) !important;
  }

  :global(dialog button) {
    cursor: pointer !important;
  }

  .shell {
    display: grid;
    grid-template-columns: 260px 1fr;
    height: 100vh;
    font-family: 'Inter', 'Segoe UI Variable', 'Segoe UI', sans-serif;
    background: #191b1f;
  }

  main {
    display: flex;
    flex-direction: column;
    background: transparent;
    padding: 0;
    height: 100%;
    overflow: hidden;
  }

  .content {
    /* padding: 12px; */
    flex: 1;
    /* overflow-y: auto; */
    color: #fff;
    background-color: transparent;
  }
  .blur-overlay{
    filter: blur(6px);
    pointer-events: none;
  }
</style>
