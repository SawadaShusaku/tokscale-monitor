<script lang="ts">
  import { onMount, onDestroy } from "svelte";
  import { getUnifiedMessages } from "$lib/tokscale";
  import type { UnifiedData, UnifiedMessage } from "$lib/types";
  import { CLIENT_CONFIGS, ALL_CLIENTS } from "$lib/clients/config";
  import type { ClientId } from "$lib/clients/config";
  import {
    filterTargetMessages,
    calculateWindowCost,
    calculateWindowMetrics,
    calculateNextFreeSlot,
    aggregateAllModels,
    aggregateAllProviders,
  } from "$lib/calculator";
  import type { OverviewMode } from "$lib/calculator";
  import { settingsStore } from "$lib/stores/settingsStore";
  import type { ProviderSettings } from "$lib/stores/settingsStore";
  import TabBar from "$lib/components/TabBar.svelte";
  import OverviewDashboard from "$lib/components/OverviewDashboard.svelte";
  import ClientDashboard from "$lib/components/ClientDashboard.svelte";
  import GearIcon from "$lib/components/GearIcon.svelte";
  import SettingsPanel from "$lib/components/SettingsPanel.svelte";

  type TabId = "overview" | ClientId;
  type OverviewWindow = "5h" | "Weekly" | "Monthly";

  const OVERVIEW_WINDOW_HOURS: Record<OverviewWindow, number> = {
    "5h": 5,
    "Weekly": 7 * 24,
    "Monthly": 30 * 24,
  };

  let activeTab = $state<TabId>("overview");
  let overviewWindow = $state<OverviewWindow>("5h");
  let overviewMode = $state<OverviewMode>("provider");
  let clientMessages = $state<Record<string, UnifiedMessage[]>>({});
  let clientErrors = $state<Record<string, string | null>>({});
  let overviewError = $state<string | null>(null);
  let lastUpdated = $state<Date | null>(null);
  let intervalId = $state<ReturnType<typeof setInterval> | null>(null);
  let isLoading = $state(false);
  let now = $state(Date.now());
  let showSettings = $state(false);
  let settings = $state<ProviderSettings>({ enabled: {} });

  // Sync settings store to local reactive state
  $effect(() => {
    const unsub = settingsStore.subscribe((s) => {
      settings = s;
    });
    return unsub;
  });

  // If active provider tab is disabled, switch to overview
  $effect(() => {
    const enabled = enabledClients;
    if (activeTab !== "overview" && !enabled.includes(activeTab)) {
      activeTab = "overview";
    }
  });

  const enabledClients = $derived.by(() => {
    return ALL_CLIENTS.filter((c) => settings.enabled[c] ?? true);
  });

  const tabs = $derived.by(() => {
    const result: { id: TabId; label: string }[] = [
      { id: "overview", label: "Overview" },
    ];
    for (const client of enabledClients) {
      result.push({ id: client, label: CLIENT_CONFIGS[client].label });
    }
    return result;
  });

  async function fetchClientData(client: ClientId): Promise<UnifiedMessage[]> {
    const raw = await getUnifiedMessages(client);
    const data: UnifiedData = JSON.parse(raw);
    return data.messages ?? [];
  }

  async function fetchAllData() {
    isLoading = true;
    overviewError = null;
    const newErrors: Record<string, string | null> = {};
    const newMessages: Record<string, UnifiedMessage[]> = {};

    try {
      const results = await Promise.allSettled(
        ALL_CLIENTS.map(async (client) => {
          const messages = await fetchClientData(client);
          return { client, messages };
        })
      );

      for (const result of results) {
        if (result.status === "fulfilled") {
          newMessages[result.value.client] = result.value.messages;
        } else {
          const client = ALL_CLIENTS[results.indexOf(result)];
          newErrors[client] = result.reason instanceof Error ? result.reason.message : String(result.reason);
        }
      }

      clientMessages = newMessages;
      clientErrors = newErrors;
      now = Date.now();
      lastUpdated = new Date();
    } catch (e) {
      overviewError = e instanceof Error ? e.message : String(e);
    } finally {
      isLoading = false;
    }
  }

  async function fetchActiveTabData() {
    if (activeTab === "overview") {
      await fetchAllData();
    } else {
      isLoading = true;
      clientErrors = { ...clientErrors, [activeTab]: null };
      try {
        const messages = await fetchClientData(activeTab);
        clientMessages = { ...clientMessages, [activeTab]: messages };
        now = Date.now();
        lastUpdated = new Date();
      } catch (e) {
        clientErrors = {
          ...clientErrors,
          [activeTab]: e instanceof Error ? e.message : String(e),
        };
      } finally {
        isLoading = false;
      }
    }
  }

  function startPolling() {
    stopPolling();
    fetchActiveTabData();
    intervalId = setInterval(fetchActiveTabData, 60_000);
  }

  function stopPolling() {
    if (intervalId) {
      clearInterval(intervalId);
      intervalId = null;
    }
  }

  function handleRefresh() {
    startPolling();
  }

  function handleTabChange(tab: TabId) {
    activeTab = tab;
    startPolling();
  }

  function handleOverviewWindowChange(window: OverviewWindow) {
    overviewWindow = window;
  }

  function handleOverviewModeChange(mode: OverviewMode) {
    overviewMode = mode;
  }

  function handleOpenSettings() {
    showSettings = true;
  }

  function handleCloseSettings() {
    showSettings = false;
  }

  onMount(() => {
    startPolling();
  });

  onDestroy(() => {
    stopPolling();
  });

  // Derived state for Overview - filtered by enabled providers only
  const overviewModels = $derived.by(() => {
    if (activeTab !== "overview") return [];
    const enabledSet = new Set(enabledClients);
    const filteredMessages: Record<string, UnifiedMessage[]> = {};
    const clientLabels: Record<string, string> = {};
    const planConfigs: Record<string, typeof CLIENT_CONFIGS[ClientId]["planConfig"]> = {};

    for (const client of ALL_CLIENTS) {
      if (!enabledSet.has(client)) continue;
      filteredMessages[client] = clientMessages[client] ?? [];
      clientLabels[client] = CLIENT_CONFIGS[client].label;
      planConfigs[client] = CLIENT_CONFIGS[client].planConfig;
    }

    const hours = OVERVIEW_WINDOW_HOURS[overviewWindow];
    if (overviewMode === "provider") {
      return aggregateAllProviders(filteredMessages, clientLabels, planConfigs, now, hours);
    }
    return aggregateAllModels(filteredMessages, clientLabels, planConfigs, now, hours);
  });

  // Derived state for client tabs
  const clientWindows = $derived.by(() => {
    if (activeTab === "overview") return [];
    const config = CLIENT_CONFIGS[activeTab];
    const messages = clientMessages[activeTab] ?? [];
    const targetMessages = filterTargetMessages(messages, config.planConfig);

    return config.planConfig.windows.map((w) => {
      const cost = calculateWindowCost(targetMessages, now, w.hours);
      const metrics = calculateWindowMetrics(cost, w.limit);
      return { label: w.label, metrics };
    });
  });

  const clientNextFreeSlot = $derived.by(() => {
    if (activeTab === "overview") return { minutesUntil: null as number | null };
    const config = CLIENT_CONFIGS[activeTab];
    const messages = clientMessages[activeTab] ?? [];
    const targetMessages = filterTargetMessages(messages, config.planConfig);
    return calculateNextFreeSlot(
      targetMessages,
      now,
      config.planConfig.windows[0].hours
    );
  });

  const activeError = $derived(
    activeTab === "overview" ? overviewError : clientErrors[activeTab] ?? null
  );
</script>

<div class="min-h-screen p-6 bg-smarthr-stone01">
  <div class="max-w-4xl mx-auto">
    <div class="flex items-center justify-between mb-6">
      <h1 class="text-2xl font-bold text-smarthr-text-black">
        Tokscale Monitor
      </h1>
      <div class="flex items-center gap-4">
        {#if lastUpdated}
          <span class="text-xs text-smarthr-text-grey">
            Last updated: {lastUpdated.toLocaleTimeString()}
          </span>
        {/if}
        <button
          onclick={handleRefresh}
          disabled={isLoading}
          class="px-4 py-2 text-sm font-bold text-white bg-smarthr-product-main rounded-md hover:opacity-90 transition-opacity disabled:opacity-50"
        >
          {isLoading ? "Loading..." : "Refresh"}
        </button>
        <GearIcon onClick={handleOpenSettings} />
      </div>
    </div>

    <TabBar {activeTab} {tabs} onTabChange={handleTabChange} />

    <div class="mt-6">
      {#if activeTab === "overview"}
        <OverviewDashboard
          models={overviewModels}
          error={activeError}
          activeWindow={overviewWindow}
          onWindowChange={handleOverviewWindowChange}
          overviewMode={overviewMode}
          onModeChange={handleOverviewModeChange}
        />
      {:else}
        <ClientDashboard
          windows={clientWindows}
          nextFreeSlotMinutes={clientNextFreeSlot.minutesUntil}
          clientLabel={CLIENT_CONFIGS[activeTab].label}
          error={activeError}
        />
      {/if}
    </div>
  </div>
</div>

<SettingsPanel isOpen={showSettings} onClose={handleCloseSettings} />
