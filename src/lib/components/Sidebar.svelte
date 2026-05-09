<script lang="ts">
  import { getView, navigate } from "../stores/view.svelte";

  type NavItem = { label: string; screen: "table" | "matches" | "teams"; icon: "table" | "matches" | "teams" };

  const items: NavItem[] = [
    { label: "Table", screen: "table", icon: "table" },
    { label: "Matches", screen: "matches", icon: "matches" },
    { label: "Teams", screen: "teams", icon: "teams" },
  ];

  function go(item: NavItem) {
    if (item.screen === "matches") {
      navigate({ screen: "matches", matchday: 1 });
    } else {
      navigate(item.screen === "table" ? { screen: "table" } : { screen: "teams" });
    }
  }

  function isActive(screen: string) {
    const v = getView();
    if (screen === "matches") {
      return v.screen === "matches" || v.screen === "match_detail";
    }
    if (screen === "teams") {
      return v.screen === "teams" || v.screen === "team_detail";
    }
    return v.screen === screen;
  }
</script>

<nav class="flex flex-col w-16 h-full bg-[var(--color-surface-elevated)] border-r border-[var(--color-border)]">
  <div class="flex-1 flex flex-col items-center pt-4 gap-1">
    {#each items as item}
      <button
        onclick={() => go(item)}
        class="w-full flex flex-col items-center py-3 px-1 text-xs gap-1 transition-colors cursor-pointer
          {isActive(item.screen)
            ? 'text-[var(--color-bundesliga-red)] border-l-2 border-[var(--color-bundesliga-red)] bg-[var(--color-surface-hover)]'
            : 'text-[var(--color-text-muted)] hover:text-[var(--color-text)] hover:bg-[var(--color-surface-hover)]'}"
      >
        <span class="grid h-5 w-5 place-items-center">
          {#if item.icon === "table"}
            <svg viewBox="0 0 20 20" fill="none" aria-hidden="true">
              <path d="M4 5.5h12M4 10h12M4 14.5h12M7 3.5v13M13 3.5v13" stroke="currentColor" stroke-width="1.6" stroke-linecap="round" />
            </svg>
          {:else if item.icon === "matches"}
            <svg viewBox="0 0 20 20" fill="none" aria-hidden="true">
              <circle cx="10" cy="10" r="6.5" stroke="currentColor" stroke-width="1.6" />
              <path d="m10 6.5 3 2.2-1.1 3.5H8.1L7 8.7l3-2.2Z" stroke="currentColor" stroke-width="1.4" stroke-linejoin="round" />
              <path d="m4.7 8.2 2.3.5M13 8.7l2.3-.5M8.1 12.2l-1.4 2M11.9 12.2l1.4 2" stroke="currentColor" stroke-width="1.2" stroke-linecap="round" />
            </svg>
          {:else}
            <svg viewBox="0 0 20 20" fill="none" aria-hidden="true">
              <path d="M10 3.5 15.5 6v4.2c0 3.1-2.2 5.2-5.5 6.3-3.3-1.1-5.5-3.2-5.5-6.3V6L10 3.5Z" stroke="currentColor" stroke-width="1.6" stroke-linejoin="round" />
              <path d="M7.5 10h5" stroke="currentColor" stroke-width="1.6" stroke-linecap="round" />
            </svg>
          {/if}
        </span>
        <span>{item.label}</span>
      </button>
    {/each}
  </div>
</nav>
