<script lang="ts">
  import DonutChart from "./DonutChart.svelte";
  import type { WindowMetrics } from "../calculator";

  interface Props {
    windows: {
      label: string;
      metrics: WindowMetrics;
    }[];
    nextFreeSlotMinutes: number | null;
    lastUpdated: Date | null;
    onRefresh: () => void;
    error: string | null;
  }

  let {
    windows,
    nextFreeSlotMinutes,
    lastUpdated,
    onRefresh,
    error,
  }: Props = $props();

  function formatNextFreeSlot(minutes: number | null): string {
    if (minutes === null) return "Window is free";
    if (minutes <= 0) return "Window is now free";
    if (minutes < 60) return `Next free slot: in ${minutes}m`;
    const h = Math.floor(minutes / 60);
    const m = minutes % 60;
    return `Next free slot: in ${h}h ${m}m`;
  }
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
          onclick={onRefresh}
          class="px-4 py-2 text-sm font-bold text-white bg-smarthr-product-main rounded-md hover:opacity-90 transition-opacity"
        >
          Refresh
        </button>
      </div>
    </div>

    {#if error}
      <div
        class="mb-6 p-4 rounded-md bg-red-50 border border-smarthr-danger text-smarthr-danger text-sm"
      >
        {error}
      </div>
    {/if}

    <div class="grid grid-cols-1 md:grid-cols-3 gap-6">
      {#each windows as w}
        <div
          class="bg-white rounded-lg p-6 shadow-sm border border-smarthr-border"
        >
          <DonutChart
            usageRate={w.metrics.usageRate}
            remaining={w.metrics.remaining}
            isWarning={w.metrics.isWarning}
            label={w.label}
          />
        </div>
      {/each}
    </div>

    <div class="mt-6 text-center text-sm text-smarthr-text-grey">
      {formatNextFreeSlot(nextFreeSlotMinutes)}
    </div>
  </div>
</div>
