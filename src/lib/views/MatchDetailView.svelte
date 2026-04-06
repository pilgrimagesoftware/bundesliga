<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import type { Goal } from "../../types/Goal";
  import type { Match } from "../../types/Match";
  import { navigate } from "../stores/view.svelte";

  let {
    matchId,
    fromMatchday,
  }: {
    matchId: number;
    fromMatchday: number;
  } = $props();

  let match = $state<Match | null>(null);
  let loading = $state(true);
  let error = $state<string | null>(null);

  $effect(() => {
    invoke<Match>("get_match_detail", { matchId })
      .then((m) => {
        match = m;
      })
      .catch((e) => {
        error = String(e);
      })
      .finally(() => {
        loading = false;
      });
  });

  function finalScore(m: Match): { t1: number; t2: number } | null {
    const final = m.results?.find((r) => r.result_type === 2);
    if (final?.points_team1 != null && final?.points_team2 != null) {
      return { t1: final.points_team1, t2: final.points_team2 };
    }
    const cur = m.results?.find((r) => r.result_type === 1);
    if (cur?.points_team1 != null && cur?.points_team2 != null) {
      return { t1: cur.points_team1, t2: cur.points_team2 };
    }
    return null;
  }

  const sortedGoals = $derived(
    (match?.goals ?? []).slice().sort((a, b) => (a.match_minute ?? 0) - (b.match_minute ?? 0))
  );

  function formatViewers(n: number): string {
    if (n >= 1_000_000) return (n / 1_000_000).toFixed(1) + "M";
    if (n >= 1000) return (n / 1000).toFixed(0) + "K";
    return String(n);
  }

  function goalBadges(g: Goal): string[] {
    const b: string[] = [];
    if (g.is_penalty) b.push("PEN");
    if (g.is_own_goal) b.push("OG");
    if (g.is_overtime) b.push("ET");
    return b;
  }
</script>

<div class="flex-1 overflow-auto p-4 max-w-2xl mx-auto w-full">
  <button
    onclick={() => navigate({ screen: "matches", matchday: fromMatchday })}
    class="text-sm text-[var(--color-text-muted)] hover:text-[var(--color-text)] mb-4 cursor-pointer"
  >
    ← Back to Matchday {fromMatchday}
  </button>

  {#if loading}
    <div class="flex items-center justify-center h-32 text-[var(--color-text-muted)]">Loading…</div>
  {:else if error || !match}
    <div class="text-[var(--color-bundesliga-red)] p-4">{error ?? "Match not found"}</div>
  {:else}
    <!-- Match header -->
    <div class="bg-[var(--color-surface-elevated)] rounded border border-[var(--color-border)] p-4 mb-4">
      <div class="flex items-center justify-between gap-4">
        <div class="flex flex-col items-center gap-1 flex-1">
          {#if match.team1.icon_url}
            <img src={match.team1.icon_url} alt={match.team1.name ?? ""} class="w-12 h-12 object-contain" />
          {/if}
          <span class="text-sm font-medium text-center">{match.team1.name ?? "?"}</span>
        </div>

        <div class="flex flex-col items-center shrink-0">
          {#if finalScore(match)}
            {@const s = finalScore(match)!}
            <span class="text-3xl font-bold tabular-nums">{s.t1} – {s.t2}</span>
          {:else}
            <span class="text-xl text-[var(--color-text-muted)]">vs</span>
          {/if}
          {#if match.is_finished}
            <span class="text-xs text-[var(--color-text-muted)] mt-1">FT</span>
          {/if}
        </div>

        <div class="flex flex-col items-center gap-1 flex-1">
          {#if match.team2.icon_url}
            <img src={match.team2.icon_url} alt={match.team2.name ?? ""} class="w-12 h-12 object-contain" />
          {/if}
          <span class="text-sm font-medium text-center">{match.team2.name ?? "?"}</span>
        </div>
      </div>
    </div>

    <!-- Goal timeline -->
    {#if sortedGoals.length > 0}
      <div class="mb-4">
        <h3 class="text-xs uppercase text-[var(--color-text-muted)] mb-2">Goals</h3>
        <div class="flex flex-col gap-1">
          {#each sortedGoals as goal}
            <div class="flex items-center gap-3 py-1 border-b border-[var(--color-border)] text-sm">
              <span class="text-[var(--color-text-muted)] tabular-nums w-8 text-right shrink-0">
                {goal.match_minute ?? "?"}′
              </span>
              <span class="flex-1">{goal.goal_getter_name ?? "Unknown"}</span>
              {#each goalBadges(goal) as badge}
                <span class="text-xs bg-[var(--color-surface-elevated)] border border-[var(--color-border)] rounded px-1 py-0.5">{badge}</span>
              {/each}
              <span class="text-[var(--color-text-muted)] tabular-nums text-xs">
                {goal.score_team1 ?? "?"} – {goal.score_team2 ?? "?"}
              </span>
            </div>
          {/each}
        </div>
      </div>
    {/if}

    <!-- Match metadata -->
    <div class="text-xs text-[var(--color-text-muted)] flex flex-col gap-1">
      {#if match.location?.location_stadium}
        <span>🏟️ {match.location.location_stadium}{match.location.location_city ? `, ${match.location.location_city}` : ""}</span>
      {/if}
      {#if match.number_of_viewers != null}
        <span>👥 {formatViewers(match.number_of_viewers)} viewers</span>
      {/if}
      {#if match.when_utc}
        <span>🗓️ {new Date(match.when_utc).toLocaleString()}</span>
      {/if}
    </div>
  {/if}
</div>
