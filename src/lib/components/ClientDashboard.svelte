<script lang="ts">
  import DonutChart from "./DonutChart.svelte";
  import type { WindowMetrics } from "../calculator";

  interface Props {
    windows: {
      label: string;
      metrics: WindowMetrics;
    }[];
    nextFreeSlotMinutes: number | null;
    clientLabel: string;
    error: string | null;
  }

  let {
    windows,
    nextFreeSlotMinutes,
    clientLabel,
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

<div class="bg-white rounded-lg shadow-sm border border-smarthr-border overflow-hidden">
  <div class="p-4 border-b border-smarthr-border">
    <h2 class="text-lg font-bold text-smarthr-text-black">{clientLabel}</h2>
    <p class="text-sm text-smarthr-text-grey mt-1">Rolling window metrics</p>
  </div>

  {#if error}
    <div class="p-4 bg-red-50 border-b border-smarthr-danger text-smarthr-danger text-sm">
      {error}
    </div>
  {/if}

  <div class="p-6">
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
