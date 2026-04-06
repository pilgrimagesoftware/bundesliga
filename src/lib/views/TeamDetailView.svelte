<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import type { CachedResponse } from "../../types/CachedResponse";
  import type { Match } from "../../types/Match";
  import type { Player } from "../../types/Player";
  import type { TableTeam } from "../../types/TableTeam";
  import type { TeamDetail } from "../../types/TeamDetail";
  import { getLeague, getSeason } from "../stores/context.svelte";
  import { navigate } from "../stores/view.svelte";

  let { teamId }: { teamId: number } = $props();

  let detail = $state<TeamDetail | null>(null);
  let tableEntry = $state<TableTeam | null>(null);
  let tablePosition = $state<number | null>(null);
  let recentMatches = $state<Match[]>([]);
  let loading = $state(true);
  let error = $state<string | null>(null);

  $effect(() => {
    load();
  });

  async function load() {
    const league = getLeague();
    const season = getSeason();
    if (!league || !season) return;
    loading = true;
    error = null;
    try {
      // Get team detail (name will be found from table first)
      const tableResp: CachedResponse<TableTeam[]> = await invoke("get_table", {
        league: league.shortcut,
        season,
      });
      const tbl = tableResp.data;
      const idx = tbl.findIndex((t) => t.id === teamId);
      if (idx >= 0) {
        tableEntry = tbl[idx];
        tablePosition = idx + 1;
      }

      const teamName = tableEntry?.name ?? tableEntry?.short_name ?? String(teamId);
      const resp: CachedResponse<TeamDetail> = await invoke("get_team_detail", {
        teamId,
        teamName,
        league: league.shortcut,
        season,
      });
      detail = resp.data;

      // Fetch recent matches for this team from matchday 1 as a sample
      try {
        const matchResp = await invoke<{ data: Match[] }>("get_matches_for_matchday", {
          league: league.shortcut,
          season,
          groupOrderId: 1,
        }).catch(() => ({ data: [] as Match[] }));

        recentMatches = matchResp.data
          .filter((m) => m.team1.id === teamId || m.team2.id === teamId)
          .slice(0, 6);
      } catch (_) {}
    } catch (e) {
      error = String(e);
    } finally {
      loading = false;
    }
  }

  type PositionGroup = "GK" | "DF" | "MF" | "FW" | "Other";

  function positionGroup(pos: string | null): PositionGroup {
    if (!pos) return "Other";
    const p = pos.toLowerCase();
    if (p.includes("goal") || p === "gk") return "GK";
    if (p.includes("defend") || p === "df" || p === "cb" || p === "lb" || p === "rb") return "DF";
    if (p.includes("mid") || p === "mf" || p === "cm" || p === "dm" || p === "am") return "MF";
    if (p.includes("forward") || p.includes("attack") || p === "fw" || p === "cf" || p === "lw" || p === "rw" || p === "st") return "FW";
    return "Other";
  }

  const squadByPosition = $derived(() => {
    const groups: Record<string, Player[]> = { GK: [], DF: [], MF: [], FW: [] };
    for (const p of detail?.squad ?? []) {
      const g = positionGroup(p.position);
      if (g !== "Other") groups[g].push(p);
      else groups["FW"].push(p);
    }
    return groups;
  });

  function calcAge(dob: string | null): string {
    if (!dob) return "";
    const birth = new Date(dob);
    const now = new Date();
    const age = now.getFullYear() - birth.getFullYear();
    return `(${age})`;
  }

  function matchScore(m: Match): string {
    const r = m.results?.find((r) => r.result_type === 2) ?? m.results?.find((r) => r.result_type === 1);
    if (!r) return "–:–";
    return `${r.points_team1}:${r.points_team2}`;
  }
</script>

<div class="flex-1 overflow-auto p-4 max-w-3xl mx-auto w-full">
  <button
    onclick={() => navigate({ screen: "teams" })}
    class="text-sm text-[var(--color-text-muted)] hover:text-[var(--color-text)] mb-4 cursor-pointer"
  >
    ← Back to Teams
  </button>

  {#if loading}
    <div class="flex items-center justify-center h-32 text-[var(--color-text-muted)]">Loading…</div>
  {:else if error || !detail}
    <div class="text-[var(--color-bundesliga-red)] p-4">{error ?? "Team not found"}</div>
  {:else}
    <!-- Identity section -->
    <div class="bg-[var(--color-surface-elevated)] rounded border border-[var(--color-border)] p-4 mb-4 flex items-center gap-4">
      {#if detail.icon_url ?? tableEntry?.icon_url}
        <img src={detail.icon_url ?? tableEntry?.icon_url ?? ""} alt={detail.name ?? ""} class="w-16 h-16 object-contain shrink-0" />
      {/if}
      <div class="flex flex-col gap-1">
        <h2 class="text-lg font-bold">{detail.name ?? "Unknown Team"}</h2>
        {#if detail.founded}
          <span class="text-xs text-[var(--color-text-muted)]">Founded: {detail.founded}</span>
        {/if}
        {#if detail.stadium}
          <span class="text-xs text-[var(--color-text-muted)]">
            🏟️ {detail.stadium}{detail.capacity ? ` (cap. ${detail.capacity})` : ""}
          </span>
        {/if}
      </div>
    </div>

    <!-- Season stats row -->
    {#if tableEntry}
      <div class="bg-[var(--color-surface-elevated)] rounded border border-[var(--color-border)] p-3 mb-4">
        <div class="text-xs uppercase text-[var(--color-text-muted)] mb-2">Season Stats</div>
        <div class="flex gap-4 text-sm text-center">
          {#if tablePosition}<div class="flex flex-col"><span class="font-bold text-[var(--color-bundesliga-red)]">{tablePosition}</span><span class="text-[var(--color-text-muted)] text-xs">Pos</span></div>{/if}
          <div class="flex flex-col"><span class="font-bold">{tableEntry.matches}</span><span class="text-[var(--color-text-muted)] text-xs">P</span></div>
          <div class="flex flex-col"><span class="font-bold">{tableEntry.wins}</span><span class="text-[var(--color-text-muted)] text-xs">W</span></div>
          <div class="flex flex-col"><span class="font-bold">{tableEntry.draws}</span><span class="text-[var(--color-text-muted)] text-xs">D</span></div>
          <div class="flex flex-col"><span class="font-bold">{tableEntry.losses}</span><span class="text-[var(--color-text-muted)] text-xs">L</span></div>
          <div class="flex flex-col"><span class="font-bold">{tableEntry.goals}</span><span class="text-[var(--color-text-muted)] text-xs">GF</span></div>
          <div class="flex flex-col"><span class="font-bold">{tableEntry.opponent_goals}</span><span class="text-[var(--color-text-muted)] text-xs">GA</span></div>
          <div class="flex flex-col"><span class="font-bold {tableEntry.goal_difference >= 0 ? 'text-green-400' : 'text-[var(--color-bundesliga-red)]'}">{tableEntry.goal_difference > 0 ? "+" : ""}{tableEntry.goal_difference}</span><span class="text-[var(--color-text-muted)] text-xs">GD</span></div>
          <div class="flex flex-col"><span class="font-bold text-[var(--color-bundesliga-red)]">{tableEntry.points}</span><span class="text-[var(--color-text-muted)] text-xs">Pts</span></div>
        </div>
      </div>
    {/if}

    <!-- Squad section -->
    {#if detail.squad_source_found && detail.squad.length > 0}
      <div class="mb-4">
        <div class="text-xs uppercase text-[var(--color-text-muted)] mb-2">Squad</div>
        {#each Object.entries(squadByPosition()) as [pos, players]}
          {#if players.length > 0}
            <div class="mb-3">
              <div class="text-xs font-semibold text-[var(--color-text-muted)] mb-1">{pos}</div>
              <div class="flex flex-col gap-0.5">
                {#each players as player}
                  <div class="flex items-center gap-3 py-1 border-b border-[var(--color-border)] text-sm">
                    <span class="flex-1">{player.name ?? "—"}</span>
                    {#if player.nationality}
                      <span class="text-xs text-[var(--color-text-muted)]">{player.nationality}</span>
                    {/if}
                    {#if player.date_of_birth}
                      <span class="text-xs text-[var(--color-text-muted)]">{player.date_of_birth} {calcAge(player.date_of_birth)}</span>
                    {/if}
                  </div>
                {/each}
              </div>
            </div>
          {/if}
        {/each}
      </div>
    {:else}
      <div class="text-[var(--color-text-muted)] text-sm p-3 bg-[var(--color-surface-elevated)] rounded border border-[var(--color-border)] mb-4">
        Squad data unavailable.
      </div>
    {/if}

    <!-- Staff section -->
    {#if detail.staff.length > 0}
      <div class="mb-4">
        <div class="text-xs uppercase text-[var(--color-text-muted)] mb-2">Staff</div>
        <div class="flex flex-col gap-1">
          {#each detail.staff as member}
            <div class="flex items-center gap-3 text-sm py-1 border-b border-[var(--color-border)]">
              <span class="flex-1">{member.name}</span>
              <span class="text-xs text-[var(--color-text-muted)]">{member.role}</span>
            </div>
          {/each}
        </div>
      </div>
    {/if}
  {/if}
</div>
