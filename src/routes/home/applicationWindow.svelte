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
        class="!bg-[var(--cat-mantle)] !dark:bg-[var(--cat-mantle)] border border-[var(--cat-surface0)] rounded-xl shadow-lg shadow-[var(--cat-crust)] backdrop:bg-black/60 text-[var(--cat-text)]"
        bodyClass="p-6 space-y-6 !bg-[var(--cat-mantle)] rounded-xl"
        closeBtnClass="text-[var(--cat-subtext)] hover:text-[var(--cat-text)] cursor-pointer absolute top-4 end-4 transition-colors"
      >
        {#snippet header()}
          <div class="flex items-center gap-2">
            <div class="p-1.5 bg-[var(--cat-mauve)]/10 text-[var(--cat-mauve)] rounded-lg">
              <Icon name="add" size={20} strokeWidth={2} />
            </div>
            <h3 class="text-xl font-bold text-[var(--cat-text)] tracking-wide">Load New Environment Variable</h3>
          </div>
        {/snippet}

        <div class="space-y-4">
          <div>
            <Label for="var-name" class="block mb-2 text-sm font-medium text-[var(--cat-text)]">Variable Name</Label>
            <Input 
              id="var-name" 
              type="text" 
              placeholder="e.g. DATABASE_URL" 
              bind:value={varName} 
              required 
              class="w-full bg-[var(--cat-base)] border border-[var(--cat-surface0)] text-[var(--cat-text)] placeholder-[var(--cat-subtext)] rounded-lg focus:ring-0 focus:border-[var(--cat-mauve)] p-2.5 font-mono"
            />
          </div>

          <div>
            <Label for="var-value" class="block mb-2 text-sm font-medium text-[var(--cat-text)]">Variable Value</Label>
            <Input 
              id="var-value" 
              type="text" 
              placeholder="e.g. postgresql://user:pass@localhost:5432/db" 
              bind:value={varValue} 
              required 
              class="w-full bg-[var(--cat-base)] border border-[var(--cat-surface0)] text-[var(--cat-text)] placeholder-[var(--cat-subtext)] rounded-lg focus:ring-0 focus:border-[var(--cat-mauve)] p-2.5 font-mono"
            />
          </div>
        </div>

        {#snippet footer()}
          <div class="flex justify-end gap-3 w-full border-t border-[var(--cat-surface0)] pt-4">
            <Button 
              color="alternative" 
              onclick={() => blur = false}
              class="bg-transparent border border-[var(--cat-surface0)] hover:bg-[var(--cat-surface0)] text-[var(--cat-text)] font-medium cursor-pointer rounded-lg transition-colors"
            >
              Cancel
            </Button>
            <Button 
              type="submit" 
              disabled={submitting}
              class="bg-[var(--cat-mauve)] hover:opacity-90 border-none text-[var(--cat-base)] font-bold px-5 cursor-pointer flex items-center justify-center min-w-[80px] rounded-lg shadow-md transition-all"
            >
              {#if submitting}
                <Spinner size="4" color="purple" />
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
    background: var(--cat-mantle) !important;
    background-color: var(--cat-mantle) !important;
    border: 1px solid var(--cat-surface0) !important;
    box-shadow: 0 8px 32px var(--cat-crust) !important;
    border-radius: 12px !important;
  }

  :global(dialog *), :global(dialog div) {
    border-color: var(--cat-surface0) !important;
  }

  :global(dialog button) {
    cursor: pointer !important;
  }

  .shell {
    display: grid;
    grid-template-columns: 260px 1fr;
    height: 100vh;
    font-family: 'Inter', sans-serif;
    background: var(--cat-base);
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
