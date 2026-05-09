<script lang="ts">
  import type { League } from "../../types/League";
  import { getLeague, getSeason, setLeague, setSeason } from "../stores/context.svelte";

  let {
    leagues,
    seasons,
    isLive = false,
    isOnCooldown = false,
    lastUpdatedLabel = "",
    onRefresh,
  }: {
    leagues: League[];
    seasons: number[];
    isLive?: boolean;
    isOnCooldown?: boolean;
    lastUpdatedLabel?: string;
    onRefresh: () => void;
  } = $props();

  let selectedLeagueShortcut = $state("");
  let selectedSeason = $state(0);

  $effect(() => {
    const activeLeague = getLeague();
    if (activeLeague?.shortcut && selectedLeagueShortcut !== activeLeague.shortcut) {
      selectedLeagueShortcut = activeLeague.shortcut;
    } else if (!selectedLeagueShortcut && leagues[0]?.shortcut) {
      selectedLeagueShortcut = leagues[0].shortcut;
    }
  });

  $effect(() => {
    const activeSeason = getSeason();
    if (activeSeason && selectedSeason !== activeSeason) {
      selectedSeason = activeSeason;
    } else if (!selectedSeason && seasons[0]) {
      selectedSeason = seasons[0];
    }
  });

  $effect(() => {
    const l = leagues.find((lg) => lg.shortcut === selectedLeagueShortcut);
    if (l) setLeague(l);
  });

  $effect(() => {
    if (selectedSeason) setSeason(selectedSeason);
  });
</script>

<header class="flex items-center h-12 px-4 gap-4 bg-[var(--color-surface-elevated)] border-b border-[var(--color-border)] shrink-0">
  <span class="font-bold text-[var(--color-bundesliga-red)] text-sm tracking-wide mr-2">FOOTBALL SCORES</span>

  <select
    bind:value={selectedLeagueShortcut}
    class="bg-[var(--color-surface)] border border-[var(--color-border)] text-[var(--color-text)] text-sm rounded px-2 py-1"
  >
    {#each leagues as league}
      <option value={league.shortcut}>{league.name ?? league.shortcut}</option>
    {/each}
  </select>

  <select
    bind:value={selectedSeason}
    class="bg-[var(--color-surface)] border border-[var(--color-border)] text-[var(--color-text)] text-sm rounded px-2 py-1"
  >
    {#each seasons as s}
      <option value={s}>{s}/{s + 1}</option>
    {/each}
  </select>

  {#if isLive}
    <span class="text-xs font-bold text-white bg-[var(--color-bundesliga-red)] px-2 py-0.5 rounded animate-pulse">LIVE</span>
  {/if}

  <div class="ml-auto flex items-center gap-3">
    {#if isOnCooldown && lastUpdatedLabel}
      <span class="text-xs text-[var(--color-text-muted)]">{lastUpdatedLabel}</span>
    {/if}
    <button
      onclick={onRefresh}
      disabled={isOnCooldown}
      class="text-sm px-3 py-1 rounded border border-[var(--color-border)] transition-colors
        {isOnCooldown
          ? 'text-[var(--color-text-muted)] cursor-not-allowed opacity-50'
          : 'text-[var(--color-text)] hover:border-[var(--color-bundesliga-red)] hover:text-[var(--color-bundesliga-red)] cursor-pointer'}"
    >
      <svg class="inline-block h-3.5 w-3.5 align-[-2px]" viewBox="0 0 20 20" fill="none" aria-hidden="true">
        <path d="M16.25 9.25A6.25 6.25 0 1 0 14.4 13.7" stroke="currentColor" stroke-width="1.8" stroke-linecap="round" stroke-linejoin="round" />
        <path d="M16.25 4.75v4.5h-4.5" stroke="currentColor" stroke-width="1.8" stroke-linecap="round" stroke-linejoin="round" />
      </svg>
      <span class="ml-1.5">Refresh</span>
    </button>
  </div>
</header>
