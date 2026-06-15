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
    /** @param {any} event */
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
            <img src="/appiconv2.png" alt="Env Explorer" class="w-8 h-8 object-contain drop-shadow-md" />
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
        class="!bg-[var(--retro-panel)] !dark:bg-[var(--retro-panel)] border-2 border-[var(--retro-neon)] rounded-none shadow-[4px_4px_0px_0px_var(--retro-neon)] backdrop:bg-black/80 text-[var(--retro-neon)]"
        bodyClass="p-6 space-y-6 !bg-[var(--retro-panel)] rounded-none"
        closeBtnClass="text-[var(--retro-neon)] hover:text-white cursor-pointer absolute top-4 end-4"
      >
        {#snippet header()}
          <div class="flex items-center gap-2">
            <div class="p-1.5 bg-[var(--retro-neon)]/10 text-[var(--retro-neon)] rounded-none">
              <Icon name="add" size={20} strokeWidth={2} />
            </div>
            <h3 class="text-xl font-bold text-[var(--retro-neon)] uppercase tracking-wider">Load New Environment Variable</h3>
          </div>
        {/snippet}

        <div class="space-y-4">
          <div>
            <Label for="var-name" class="block mb-2 text-sm font-bold text-[var(--retro-neon)] uppercase">Variable Name</Label>
            <Input 
              id="var-name" 
              type="text" 
              placeholder="e.g. DATABASE_URL" 
              bind:value={varName} 
              required 
              class="w-full bg-[var(--retro-bg)] border-2 border-[var(--retro-neon)] text-[var(--retro-neon)] placeholder-[var(--retro-neon)]/50 rounded-none focus:ring-0 focus:border-[var(--retro-neon)] p-2.5 font-mono"
            />
          </div>

          <div>
            <Label for="var-value" class="block mb-2 text-sm font-bold text-[var(--retro-neon)] uppercase">Variable Value</Label>
            <Input 
              id="var-value" 
              type="text" 
              placeholder="e.g. postgresql://user:pass@localhost:5432/db" 
              bind:value={varValue} 
              required 
              class="w-full bg-[var(--retro-bg)] border-2 border-[var(--retro-neon)] text-[var(--retro-neon)] placeholder-[var(--retro-neon)]/50 rounded-none focus:ring-0 focus:border-[var(--retro-neon)] p-2.5 font-mono"
            />
          </div>
        </div>

        {#snippet footer()}
          <div class="flex justify-end gap-3 w-full border-t-2 border-[var(--retro-neon)] pt-4">
            <Button 
              color="alternative" 
              onclick={() => blur = false}
              class="bg-transparent border-2 border-[var(--retro-neon)] hover:bg-[var(--retro-neon)] text-[var(--retro-neon)] hover:text-black font-bold cursor-pointer rounded-none uppercase transition-none"
            >
              Cancel
            </Button>
            <Button 
              type="submit" 
              disabled={submitting}
              class="bg-[var(--retro-neon)] hover:bg-[var(--retro-bg)] border-2 border-[var(--retro-neon)] text-black hover:text-[var(--retro-neon)] font-bold px-5 cursor-pointer flex items-center justify-center min-w-[80px] rounded-none shadow-[2px_2px_0px_0px_var(--retro-neon)] hover:shadow-none translate-y-0 hover:translate-y-[2px] transition-none uppercase"
            >
              {#if submitting}
                <Spinner size="4" color="green" />
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
  @import "../component/sidebar.css";

  :global(button), :global(input[type="radio"]), :global(label), :global(.cursor-pointer) {
    cursor: pointer !important;
  }

  :global(dialog), :global(dialog form) {
    background: var(--retro-panel) !important;
    background-color: var(--retro-panel) !important;
    border: 2px solid var(--retro-neon) !important;
    box-shadow: 6px 6px 0px var(--retro-neon) !important;
    border-radius: 0 !important;
  }

  :global(dialog *), :global(dialog div) {
    border-color: var(--retro-neon) !important;
  }

  :global(dialog button) {
    cursor: pointer !important;
  }

  .shell {
    display: grid;
    grid-template-columns: 260px 1fr;
    height: 100vh;
    font-family: 'Courier New', Courier, monospace;
    background: var(--retro-bg);
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
