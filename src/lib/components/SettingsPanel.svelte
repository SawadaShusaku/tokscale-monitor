<script lang="ts">
  import { settingsStore, type ProviderSettings } from "../stores/settingsStore";
  import { CLIENT_CONFIGS, ALL_CLIENTS } from "../clients/config";
  import type { ClientId } from "../clients/config";

  interface Props {
    isOpen: boolean;
    onClose: () => void;
  }

  let { isOpen, onClose }: Props = $props();

  let settings = $state<ProviderSettings>({ enabled: {} });

  $effect(() => {
    const unsub = settingsStore.subscribe((s) => {
      settings = s;
    });
    return unsub;
  });

  function handleToggle(client: ClientId) {
    settingsStore.toggleProvider(client);
  }

  function handleBackdropClick(e: MouseEvent) {
    if (e.target === e.currentTarget) {
      onClose();
    }
  }

  function handleKeydown(e: KeyboardEvent) {
    if (e.key === "Escape") {
      onClose();
    }
  }
</script>

<svelte:window onkeydown={handleKeydown} />

{#if isOpen}
  <div
    class="fixed inset-0 z-40 bg-black/30 backdrop-blur-sm transition-opacity"
    onclick={handleBackdropClick}
    aria-hidden="true"
  ></div>
{/if}

<aside
  class="fixed top-0 right-0 h-full w-80 max-w-[90vw] bg-white shadow-xl z-50 transform transition-transform duration-300 ease-out"
  class:translate-x-0={isOpen}
  class:translate-x-full={!isOpen}
>
  <div class="flex flex-col h-full">
    <div class="flex items-center justify-between px-5 py-4 border-b border-smarthr-border">
      <h2 class="text-lg font-bold text-smarthr-text-black">Settings</h2>
      <button
        onclick={onClose}
        class="p-2 rounded-md text-smarthr-text-grey hover:text-smarthr-text-black hover:bg-smarthr-stone02 transition-colors"
        aria-label="Close settings"
      >
        <svg
          xmlns="http://www.w3.org/2000/svg"
          width="20"
          height="20"
          viewBox="0 0 24 24"
          fill="none"
          stroke="currentColor"
          stroke-width="2"
          stroke-linecap="round"
          stroke-linejoin="round"
        >
          <path d="M18 6 6 18" />
          <path d="m6 6 12 12" />
        </svg>
      </button>
    </div>

    <div class="flex-1 overflow-y-auto px-5 py-4">
      <h3 class="text-sm font-bold text-smarthr-text-grey uppercase tracking-wider mb-4">
        Providers
      </h3>

      <div class="space-y-3">
        {#each ALL_CLIENTS.filter((c) => c !== "gemini") as client}
          <div class="flex items-center justify-between py-2">
            <div class="flex items-center gap-3">
              <span class="text-sm font-medium text-smarthr-text-black">
                {CLIENT_CONFIGS[client].label}
              </span>
            </div>
            <button
              type="button"
              role="switch"
              aria-checked={settings.enabled[client] ?? true}
              aria-label="Toggle {CLIENT_CONFIGS[client].label}"
              onclick={() => handleToggle(client)}
              class="relative inline-flex h-6 w-11 items-center rounded-full transition-colors focus:outline-none focus:ring-2 focus:ring-smarthr-product-main focus:ring-offset-2"
              class:bg-smarthr-product-main={settings.enabled[client] ?? true}
              class:bg-smarthr-border={!(settings.enabled[client] ?? true)}
            >
              <span
                class="inline-block h-4 w-4 transform rounded-full bg-white transition-transform"
                class:translate-x-6={settings.enabled[client] ?? true}
                class:translate-x-1={!(settings.enabled[client] ?? true)}
              ></span>
            </button>
          </div>
        {/each}
      </div>
    </div>

    <div class="px-5 py-4 border-t border-smarthr-border">
      <button
        onclick={() => settingsStore.reset()}
        class="w-full px-4 py-2 text-sm font-bold text-smarthr-text-grey border border-smarthr-border rounded-md hover:bg-smarthr-stone02 transition-colors"
      >
        Reset to Defaults
      </button>
    </div>
  </div>
</aside>
