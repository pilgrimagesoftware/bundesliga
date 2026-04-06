<script lang="ts">
  import { getView, navigate } from "../stores/view.svelte";

  type NavItem = { label: string; screen: "table" | "matches" | "teams"; icon: string };

  const items: NavItem[] = [
    { label: "Table", screen: "table", icon: "🏆" },
    { label: "Matches", screen: "matches", icon: "⚽" },
    { label: "Teams", screen: "teams", icon: "🛡️" },
  ];

  function go(item: NavItem) {
    if (item.screen === "matches") {
      navigate({ screen: "matches", matchday: 1 });
    } else {
      navigate({ screen: item.screen } as any);
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
        <span class="text-lg">{item.icon}</span>
        <span>{item.label}</span>
      </button>
    {/each}
  </div>
</nav>
