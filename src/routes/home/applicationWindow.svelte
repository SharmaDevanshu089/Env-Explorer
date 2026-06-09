<script>
    import ApplicationTitlebar from '../component/titlebar.svelte';
    import Icon from '../component/Icon.svelte';
    import ProjectWindow from "./windows/projectWindow.svelte"
    import AnalyticWindow from "./windows/analyticWindow.svelte"
    import BulkWindow from "./windows/bulkWindow.svelte"
    import { Modal, Button, Label, Input, Radio } from 'flowbite-svelte';

    let active = "projects";
    let blur = false;
    let varName = "";
    let varValue = "";
    let varType = "application";

    function initialte_adding_variables() {
      varName = "";
      varValue = "";
      varType = "application";
      blur = true;
      console.log("Enabling Blur");
    }
    function handleSubmit(event) {
      event.preventDefault();
      console.log("Form submitted:", { varName, varValue, varType });
      blur = false;
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
            class:active={active === 'recent'}
            on:click={() => active = 'recent'}
        >
            <Icon name="history" />
            Recent
        </button>

        <button
            class="nav-item"
            class:active={active === 'keys'}
            on:click={() => active = 'keys'}
        >
            <Icon name="key" />
            Global Keys
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

          <div class="space-y-2">
            <Label class="block text-sm font-medium text-gray-300">Variable Type</Label>
            <div class="flex gap-6 p-3 bg-[#111317] border border-white/10 rounded-md">
              <Radio 
                name="var-type" 
                value="application" 
                bind:group={varType} 
                class="text-[#72ddc3] focus:ring-[#72ddc3] cursor-pointer"
              >
                <span class="ml-2 text-sm text-gray-300 font-medium cursor-pointer">Application</span>
              </Radio>
              <Radio 
                name="var-type" 
                value="user" 
                bind:group={varType} 
                class="text-[#72ddc3] focus:ring-[#72ddc3] cursor-pointer"
              >
                <span class="ml-2 text-sm text-gray-300 font-medium cursor-pointer">User</span>
              </Radio>
            </div>
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
            {#if varType === 'application'}
              <Button 
                type="submit" 
                class="bg-[#72ddc3] hover:bg-[#5ec4ad] text-black font-semibold px-5 cursor-pointer"
              >
                Open Terminal
              </Button>
            {:else}
              <Button 
                type="submit" 
                class="bg-[#72ddc3] hover:bg-[#5ec4ad] text-black font-semibold px-5 cursor-pointer"
              >
                Load
              </Button>
            {/if}
          </div>
        {/snippet}
      </Modal>
      <div class="blur-overlay">
        {#if active === "projects"}
          <ProjectWindow />
        {:else if active === "recent"}
          <AnalyticWindow />
        {:else if active === "keys"}
          <BulkWindow />
        {/if}
      </div>
    {:else}

      <div class="no-blur-overlay">
        {#if active === "projects"}
          <ProjectWindow />
        {:else if active === "recent"}
          <AnalyticWindow />
        {:else if active === "keys"}
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
