<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import type { CachedResponse } from "../../types/CachedResponse";
  import type { TableTeam } from "../../types/TableTeam";
  import { getLeague, getSeason } from "../stores/context.svelte";
  import { navigate } from "../stores/view.svelte";

  let {
    onCooldownChange,
  }: {
    onCooldownChange?: (cached: boolean, nextRefreshAt: number | null) => void;
  } = $props();

  let table = $state<TableTeam[]>([]);
  let loading = $state(false);
  let error = $state<string | null>(null);

  async function fetchTable() {
    const league = getLeague();
    const season = getSeason();
    if (!league || !season) return;
    loading = true;
    error = null;
    try {
      const resp: CachedResponse<TableTeam[]> = await invoke("get_table", {
        league: league.shortcut,
        season,
      });
      table = resp.data;
      onCooldownChange?.(resp.cached, resp.next_refresh_at);
    } catch (e) {
      error = String(e);
    } finally {
      loading = false;
    }
  }

  $effect(() => {
    getLeague();
    getSeason();
    fetchTable();
  });

  export function refresh() {
    fetchTable();
  }

  function zoneColor(pos: number, total: number): string {
    if (pos <= 2) return "border-l-4 border-blue-500";
    if (pos <= 4) return "border-l-4 border-orange-400";
    if (pos > total - 2) return "border-l-4 border-[var(--color-bundesliga-red)]";
    return "border-l-4 border-transparent";
  }
</script>

<div class="flex-1 overflow-auto p-4">
  {#if loading && table.length === 0}
    <div class="flex items-center justify-center h-32 text-[var(--color-text-muted)]">Loading…</div>
  {:else if error}
    <div class="text-[var(--color-bundesliga-red)] p-4">{error}</div>
  {:else if table.length === 0}
    <div class="text-[var(--color-text-muted)] p-4">No data for this season.</div>
  {:else}
    <table class="w-full text-sm">
      <thead>
        <tr class="text-[var(--color-text-muted)] text-xs uppercase border-b border-[var(--color-border)]">
          <th class="py-2 px-1 text-left w-8">#</th>
          <th class="py-2 px-2 text-left w-10"></th>
          <th class="py-2 px-2 text-left">Team</th>
          <th class="py-2 px-2 text-center">P</th>
          <th class="py-2 px-2 text-center">W</th>
          <th class="py-2 px-2 text-center">D</th>
          <th class="py-2 px-2 text-center">L</th>
          <th class="py-2 px-2 text-center">GF</th>
          <th class="py-2 px-2 text-center">GA</th>
          <th class="py-2 px-2 text-center">GD</th>
          <th class="py-2 px-2 text-center font-bold">Pts</th>
        </tr>
      </thead>
      <tbody>
        {#each table as team, i}
          <tr
            class="border-b border-[var(--color-border)] hover:bg-[var(--color-surface-hover)] cursor-pointer {zoneColor(i + 1, table.length)}"
            onclick={() => navigate({ screen: "team_detail", teamId: team.id })}
          >
            <td class="py-2 px-1 text-[var(--color-text-muted)] text-center">{i + 1}</td>
            <td class="py-2 px-2">
              {#if team.icon_url}
                <img src={team.icon_url} alt={team.name ?? ""} class="w-6 h-6 object-contain" />
              {/if}
            </td>
            <td class="py-2 px-2">{team.name ?? team.short_name ?? "—"}</td>
            <td class="py-2 px-2 text-center text-[var(--color-text-muted)]">{team.matches}</td>
            <td class="py-2 px-2 text-center">{team.wins}</td>
            <td class="py-2 px-2 text-center">{team.draws}</td>
            <td class="py-2 px-2 text-center">{team.losses}</td>
            <td class="py-2 px-2 text-center">{team.goals}</td>
            <td class="py-2 px-2 text-center">{team.opponent_goals}</td>
            <td class="py-2 px-2 text-center {team.goal_difference >= 0 ? 'text-green-400' : 'text-[var(--color-bundesliga-red)]'}">
              {team.goal_difference > 0 ? "+" : ""}{team.goal_difference}
            </td>
            <td class="py-2 px-2 text-center font-bold">{team.points}</td>
          </tr>
        {/each}
      </tbody>
    </table>
  {/if}
</div>
