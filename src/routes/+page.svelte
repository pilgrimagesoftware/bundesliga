<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import type { AppViewState } from "../types/AppViewState";
  import type { League } from "../types/League";
  import Header from "../lib/components/Header.svelte";
  import Sidebar from "../lib/components/Sidebar.svelte";
  import MatchDetailView from "../lib/views/MatchDetailView.svelte";
  import MatchesView from "../lib/views/MatchesView.svelte";
  import TableView from "../lib/views/TableView.svelte";
  import TeamDetailView from "../lib/views/TeamDetailView.svelte";
  import TeamsView from "../lib/views/TeamsView.svelte";
  import { getLeague, getSeason, setLeague, setSeason } from "../lib/stores/context.svelte";
  import { getView, navigate } from "../lib/stores/view.svelte";

  let leagues = $state<League[]>([]);
  let seasons = $state<number[]>([]);

  // Cooldown state for Header
  let isOnCooldown = $state(false);
  let nextRefreshAt = $state<number | null>(null);
  let lastUpdatedLabel = $state("");
  let cooldownTimer: ReturnType<typeof setInterval> | null = null;

  // View component refs for manual refresh
  let tableViewRef: { refresh: () => void } | null = $state(null);
  let matchesViewRef: { refresh: () => void } | null = $state(null);

  function handleCooldownChange(cached: boolean, nra: number | null) {
    isOnCooldown = cached && nra != null;
    nextRefreshAt = nra;
    if (cooldownTimer) clearInterval(cooldownTimer);
    if (isOnCooldown && nextRefreshAt) {
      cooldownTimer = setInterval(() => {
        if (!nextRefreshAt) return;
        const remaining = Math.max(0, nextRefreshAt - Date.now());
        if (remaining === 0) {
          isOnCooldown = false;
          lastUpdatedLabel = "";
          if (cooldownTimer) clearInterval(cooldownTimer);
        } else {
          const secs = Math.ceil(remaining / 1000);
          lastUpdatedLabel = `Next refresh in ${secs}s`;
        }
      }, 1000);
    } else {
      lastUpdatedLabel = "";
    }
  }

  let isLive = $state(false);

  function handleRefresh() {
    const v = getView();
    if (v.screen === "table") tableViewRef?.refresh();
    else if (v.screen === "matches") matchesViewRef?.refresh();
  }

  async function startup() {
    try {
      leagues = await invoke<League[]>("get_leagues");
      if (leagues.length > 0) setLeague(leagues[0]);
    } catch (_) {}

    try {
      seasons = await invoke<number[]>("get_seasons");
      if (seasons.length > 0) setSeason(seasons[0]);
    } catch (_) {}

    try {
      const saved: AppViewState | null = await invoke("get_last_viewed");
      if (saved) {
        const TWO_DAYS = 2 * 24 * 3600;
        const now = Math.floor(Date.now() / 1000);
        const elapsed = now - saved.last_opened;

        if (elapsed < TWO_DAYS) {
          const savedLeague = leagues.find((l) => l.shortcut === saved.league);
          if (savedLeague) setLeague(savedLeague);
          if (seasons.includes(saved.season)) setSeason(saved.season);

          if (saved.view === "table") {
            navigate({ screen: "table" });
          } else if (saved.view === "matches" && saved.matchday != null) {
            navigate({ screen: "matches", matchday: saved.matchday });
          } else if (saved.view === "teams") {
            navigate({ screen: "teams" });
          } else if (saved.view === "team_detail" && saved.selected_team_id != null) {
            navigate({ screen: "team_detail", teamId: saved.selected_team_id });
          } else {
            navigate({ screen: "table" });
          }
          return;
        }
      }
    } catch (_) {}

    // Default: navigate to current matchday
    try {
      const league = getLeague();
      if (league) {
        const cur = await invoke<{ order_id: number }>("get_current_matchday", {
          league: league.shortcut,
        });
        navigate({ screen: "matches", matchday: cur.order_id });
        return;
      }
    } catch (_) {}

    navigate({ screen: "table" });
  }

  startup();

  const view = $derived(getView());
</script>

<div class="flex h-screen w-screen overflow-hidden bg-[var(--color-surface)] text-[var(--color-text)]">
  <Sidebar />

  <div class="flex flex-col flex-1 min-w-0">
    <Header
      {leagues}
      {seasons}
      {isLive}
      {isOnCooldown}
      {lastUpdatedLabel}
      onRefresh={handleRefresh}
    />

    <main class="flex flex-1 overflow-hidden">
      {#if view.screen === "table"}
        <TableView
          bind:this={tableViewRef}
          onCooldownChange={handleCooldownChange}
        />
      {:else if view.screen === "matches"}
        <MatchesView
          bind:this={matchesViewRef}
          matchday={view.matchday}
          onCooldownChange={handleCooldownChange}
        />
      {:else if view.screen === "match_detail"}
        <MatchDetailView matchId={view.matchId} fromMatchday={view.fromMatchday} />
      {:else if view.screen === "teams"}
        <TeamsView />
      {:else if view.screen === "team_detail"}
        <TeamDetailView teamId={view.teamId} />
      {/if}
    </main>
  </div>
</div>
