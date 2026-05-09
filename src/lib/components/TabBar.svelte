<script lang="ts">
  import type { ClientId } from "../clients/config";

  type TabId = "overview" | ClientId;

  interface Tab {
    id: TabId;
    label: string;
  }

  interface Props {
    activeTab: TabId;
    tabs: Tab[];
    onTabChange: (tab: TabId) => void;
  }

  let { activeTab, tabs, onTabChange }: Props = $props();

  let containerRef = $state<HTMLDivElement | null>(null);
  let canScrollLeft = $state(false);
  let canScrollRight = $state(false);

  function updateScrollIndicators() {
    const el = containerRef;
    if (!el) return;
    canScrollLeft = el.scrollLeft > 2;
    canScrollRight = el.scrollLeft + el.clientWidth < el.scrollWidth - 2;
  }

  function scrollToActiveTab() {
    const el = containerRef;
    if (!el) return;
    const activeButton = el.querySelector('[data-active="true"]') as HTMLElement | null;
    if (activeButton) {
      const containerWidth = el.clientWidth;
      const buttonLeft = activeButton.offsetLeft;
      const buttonWidth = activeButton.clientWidth;
      const scrollLeft = buttonLeft - containerWidth / 2 + buttonWidth / 2;
      el.scrollTo({ left: scrollLeft, behavior: "smooth" });
    }
  }

  $effect(() => {
    // Scroll to active tab when it changes
    activeTab;
    scrollToActiveTab();
  });
</script>

<div class="relative bg-white border-b border-smarthr-border">
  <!-- Left fade indicator -->
  {#if canScrollLeft}
    <div class="absolute left-0 top-0 bottom-0 w-8 bg-gradient-to-r from-white to-transparent pointer-events-none z-10"></div>
  {/if}
  <!-- Right fade indicator -->
  {#if canScrollRight}
    <div class="absolute right-0 top-0 bottom-0 w-8 bg-gradient-to-l from-white to-transparent pointer-events-none z-10"></div>
  {/if}

  <div
    bind:this={containerRef}
    onscroll={updateScrollIndicators}
    class="flex overflow-x-auto scrollbar-hide"
    role="tablist"
  >
    {#each tabs as tab}
      <button
        role="tab"
        data-active={activeTab === tab.id}
        onclick={() => onTabChange(tab.id)}
        class="px-5 py-3 text-sm font-bold transition-colors relative flex-shrink-0 whitespace-nowrap"
        class:text-smarthr-product-main={activeTab === tab.id}
        class:text-smarthr-text-grey={activeTab !== tab.id}
        class:hover:text-smarthr-text-black={activeTab !== tab.id}
      >
        {tab.label}
        {#if activeTab === tab.id}
          <div class="absolute bottom-0 left-0 right-0 h-0.5 bg-smarthr-product-main"></div>
        {/if}
      </button>
    {/each}
  </div>
</div>

<style>
  .scrollbar-hide {
    -ms-overflow-style: none;
    scrollbar-width: none;
  }
  .scrollbar-hide::-webkit-scrollbar {
    display: none;
  }
</style>
