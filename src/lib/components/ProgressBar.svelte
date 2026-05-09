<script lang="ts">
  interface Props {
    modelName: string;
    clientLabel: string;
    cost: number;
    limit: number;
    usageRate: number;
    isWarning: boolean;
  }

  let { modelName, clientLabel, cost, limit, usageRate, isWarning }: Props = $props();

  const percentage = $derived(Math.min(usageRate * 100, 100));

  function formatCurrency(n: number): string {
    return `$${n.toFixed(2)}`;
  }
</script>

<div class="flex items-center gap-3 py-2 min-w-0">
  <div class="w-28 sm:w-32 shrink-0 min-w-0">
    <div class="text-sm font-bold text-smarthr-text-black truncate">{modelName}</div>
    {#if clientLabel}
      <div class="text-xs text-smarthr-text-grey truncate">{clientLabel}</div>
    {/if}
  </div>
  <div class="flex-1 min-w-0">
    <div class="h-2 bg-smarthr-stone02 rounded-full overflow-hidden">
      <div
        class="h-full rounded-full transition-all duration-500"
        class:bg-smarthr-product-main={!isWarning}
        class:bg-smarthr-warning={isWarning}
        style="width: {percentage}%"
      ></div>
    </div>
  </div>
  <div class="w-24 sm:w-28 shrink-0 text-right min-w-0">
    <span class="text-sm font-bold" class:text-smarthr-text-black={!isWarning} class:text-smarthr-warning={isWarning}>
      {percentage.toFixed(0)}%
    </span>
    <span class="text-xs text-smarthr-text-grey ml-1">
      {formatCurrency(cost)} / {formatCurrency(limit)}
    </span>
  </div>
</div>
