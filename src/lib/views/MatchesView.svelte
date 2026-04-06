<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import type { CachedResponse } from "../../types/CachedResponse";
  import type { Group } from "../../types/Group";
  import type { Match } from "../../types/Match";
  import { getLeague, getSeason } from "../stores/context.svelte";
  import { navigate } from "../stores/view.svelte";
  import MatchCard from "../components/MatchCard.svelte";

  let {
    matchday: initialMatchday,
    onCooldownChange,
  }: {
    matchday: number;
    onCooldownChange?: (cached: boolean, nextRefreshAt: number | null) => void;
  } = $props();

  let matchdays = $state<Group[]>([]);
  let matches = $state<Match[]>([]);
  let currentMatchday = $state(0);

  $effect(() => {
    if (!currentMatchday) currentMatchday = initialMatchday;
  });
  let loading = $state(false);
  let error = $state<string | null>(null);
  let refreshTimer: ReturnType<typeof setInterval> | null = null;

  function isLive(m: Match): boolean {
    if (m.is_finished || !m.when_utc) return false;
    const kickoff = new Date(m.when_utc).getTime();
    const now = Date.now();
    return now >= kickoff && now <= kickoff + 2 * 60 * 60 * 1000;
  }

  async function fetchMatchdays() {
    const league = getLeague();
    const season = getSeason();
    if (!league || !season) return;
    try {
      const resp: CachedResponse<Group[]> = await invoke("get_matchdays", {
        league: league.shortcut,
        season,
      });
      matchdays = resp.data;
      if (matchdays.length > 0 && currentMatchday === 1) {
        // default to current matchday
        try {
          const cur: Group = await invoke("get_current_matchday", { league: league.shortcut });
          currentMatchday = cur.order_id;
        } catch (_) {}
      }
    } catch (e) {
      error = String(e);
    }
  }

  async function fetchMatches() {
    const league = getLeague();
    const season = getSeason();
    if (!league || !season) return;
    const md = matchdays.find((m) => m.order_id === currentMatchday);
    if (!md) return;
    loading = true;
    error = null;
    try {
      const resp: CachedResponse<Match[]> = await invoke("get_matches_for_matchday", {
        league: league.shortcut,
        season,
        groupOrderId: currentMatchday,
      });
      matches = resp.data;
      onCooldownChange?.(resp.cached, resp.next_refresh_at);
    } catch (e) {
      error = String(e);
    } finally {
      loading = false;
    }
  }

  $effect(() => {
    fetchMatchdays().then(() => fetchMatches());
  });

  $effect(() => {
    currentMatchday;
    fetchMatches();
  });

  $effect(() => {
    refreshTimer = setInterval(() => {
      if (matches.some(isLive)) fetchMatches();
    }, 30000);

    return () => {
      if (refreshTimer) clearInterval(refreshTimer);
    };
  });

  export function refresh() {
    fetchMatches();
  }

  const currentGroup = $derived(matchdays.find((m) => m.order_id === currentMatchday));
  const minMatchday = $derived(matchdays.length > 0 ? Math.min(...matchdays.map((m) => m.order_id)) : 1);
  const maxMatchday = $derived(matchdays.length > 0 ? Math.max(...matchdays.map((m) => m.order_id)) : 34);
</script>

<div class="flex-1 overflow-auto p-4">
  <!-- Matchday navigation -->
  <div class="flex items-center justify-between mb-4">
    <button
      onclick={() => currentMatchday--}
      disabled={currentMatchday <= minMatchday}
      class="px-3 py-1 rounded border border-[var(--color-border)] text-sm disabled:opacity-40 disabled:cursor-not-allowed hover:border-[var(--color-bundesliga-red)] cursor-pointer"
    >
      ← Prev
    </button>
    <h2 class="font-semibold text-sm text-[var(--color-text)]">
      {currentGroup?.name ?? `Matchday ${currentMatchday}`}
    </h2>
    <button
      onclick={() => currentMatchday++}
      disabled={currentMatchday >= maxMatchday}
      class="px-3 py-1 rounded border border-[var(--color-border)] text-sm disabled:opacity-40 disabled:cursor-not-allowed hover:border-[var(--color-bundesliga-red)] cursor-pointer"
    >
      Next →
    </button>
  </div>

  {#if loading && matches.length === 0}
    <div class="flex items-center justify-center h-32 text-[var(--color-text-muted)]">Loading…</div>
  {:else if error}
    <div class="text-[var(--color-bundesliga-red)] p-4">{error}</div>
  {:else if matches.length === 0}
    <div class="text-[var(--color-text-muted)] p-4">No matches found.</div>
  {:else}
    <div class="flex flex-col gap-2">
      {#each matches as match}
        <MatchCard
          {match}
          onclick={() => navigate({ screen: "match_detail", matchId: match.id, fromMatchday: currentMatchday })}
        />
      {/each}
    </div>
  {/if}
</div>
