<script lang="ts">
  interface Props {
    usageRate: number;
    remaining: number;
    isWarning: boolean;
    label: string;
  }

  let { usageRate, remaining, isWarning, label }: Props = $props();

  const radius = 40;
  const circumference = 2 * Math.PI * radius;
  const strokeDashoffset = $derived(circumference * (1 - Math.min(usageRate, 1)));

  const primaryColor = "#0077c7";
  const warningColor = "#ffcc17";
  const trackColor = "#d6d3d0";
  const strokeColor = $derived(isWarning ? warningColor : primaryColor);

  function formatCurrency(n: number): string {
    return `$${n.toFixed(2)}`;
  }

  function formatPercent(n: number): string {
    return `${(n * 100).toFixed(0)}%`;
  }
</script>

<div class="flex flex-col items-center gap-2">
  <h3 class="text-sm font-bold text-smarthr-text-grey">{label}</h3>
  <div class="relative w-40 h-40">
    <svg viewBox="0 0 100 100" class="w-full h-full -rotate-90">
      <circle
        cx="50"
        cy="50"
        r={radius}
        fill="none"
        stroke={trackColor}
        stroke-width="10"
      />
      <circle
        cx="50"
        cy="50"
        r={radius}
        fill="none"
        stroke={strokeColor}
        stroke-width="10"
        stroke-dasharray={circumference}
        stroke-dashoffset={strokeDashoffset}
        stroke-linecap="round"
        class="transition-all duration-500"
      />
    </svg>
    <div class="absolute inset-0 flex flex-col items-center justify-center">
      <span class="text-lg font-bold text-smarthr-text-black">
        {formatCurrency(remaining)} left
      </span>
      <span class="text-xs text-smarthr-text-grey">
        {formatPercent(usageRate)} used
      </span>
    </div>
  </div>
</div>
