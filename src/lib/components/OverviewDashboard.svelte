<script lang="ts">
  import ProgressBar from "./ProgressBar.svelte";
  import type { ModelAggregate, OverviewMode } from "../calculator";

  type WindowLabel = "5h" | "Weekly" | "Monthly";

  interface Props {
    models: ModelAggregate[];
    error: string | null;
    activeWindow: WindowLabel;
    onWindowChange: (window: WindowLabel) => void;
    overviewMode: OverviewMode;
    onModeChange: (mode: OverviewMode) => void;
  }

  let { models, error, activeWindow, onWindowChange, overviewMode, onModeChange }: Props = $props();

  const windows: { label: WindowLabel; text: string }[] = [
    { label: "5h", text: "5h" },
    { label: "Weekly", text: "Weekly" },
    { label: "Monthly", text: "Monthly" },
  ];
</script>

<div class="bg-white rounded-lg shadow-sm border border-smarthr-border overflow-hidden">
  <div class="p-4 border-b border-smarthr-border flex items-center justify-between flex-wrap gap-3">
    <h2 class="text-lg font-bold text-smarthr-text-black">Overview</h2>
    <div class="flex items-center gap-2">
      <!-- Mode toggle -->
      <div class="flex bg-smarthr-stone02 rounded-md p-0.5">
        <button
          onclick={() => onModeChange("provider")}
          class="px-3 py-1 text-xs font-bold rounded transition-colors"
          class:bg-white={overviewMode === "provider"}
          class:text-smarthr-text-black={overviewMode === "provider"}
          class:shadow-sm={overviewMode === "provider"}
          class:text-smarthr-text-grey={overviewMode !== "provider"}
        >
          Provider
        </button>
        <button
          onclick={() => onModeChange("model")}
          class="px-3 py-1 text-xs font-bold rounded transition-colors"
          class:bg-white={overviewMode === "model"}
          class:text-smarthr-text-black={overviewMode === "model"}
          class:shadow-sm={overviewMode === "model"}
          class:text-smarthr-text-grey={overviewMode !== "model"}
        >
          Model
        </button>
      </div>

      <!-- Window selector -->
      <div class="flex bg-smarthr-stone02 rounded-md p-0.5">
        {#each windows as w}
          <button
            onclick={() => onWindowChange(w.label)}
            class="px-3 py-1 text-xs font-bold rounded transition-colors"
            class:bg-white={activeWindow === w.label}
            class:text-smarthr-text-black={activeWindow === w.label}
            class:shadow-sm={activeWindow === w.label}
            class:text-smarthr-text-grey={activeWindow !== w.label}
          >
            {w.text}
          </button>
        {/each}
      </div>
    </div>
  </div>

  {#if error}
    <div class="p-4 bg-red-50 border-b border-smarthr-danger text-smarthr-danger text-sm">
      {error}
    </div>
  {/if}

  <div class="overflow-y-auto p-4 max-h-[60vh]">
    {#if models.length === 0}
      <div class="text-center py-8 text-smarthr-text-grey text-sm">
        No usage data available
      </div>
    {:else}
      <div class="space-y-1">
        {#each models as model}
          <ProgressBar
            modelName={model.modelId}
            clientLabel={overviewMode === "provider" ? "" : model.clientLabel}
            cost={model.cost}
            limit={model.limit}
            usageRate={model.usageRate}
            isWarning={model.isWarning}
          />
        {/each}
      </div>
    {/if}
  </div>
</div>
