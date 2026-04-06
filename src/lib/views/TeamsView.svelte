<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import type { Team } from "../../types/Team";
  import { getLeague, getSeason } from "../stores/context.svelte";
  import TeamCard from "../components/TeamCard.svelte";

  let teams = $state<Team[]>([]);
  let loading = $state(false);
  let error = $state<string | null>(null);

  $effect(() => {
    getLeague();
    getSeason();
    fetchTeams();
  });

  async function fetchTeams() {
    const league = getLeague();
    const season = getSeason();
    if (!league || !season) return;
    loading = true;
    error = null;
    try {
      teams = await invoke<Team[]>("get_teams", {
        league: league.shortcut,
        season,
      });
    } catch (e) {
      error = String(e);
    } finally {
      loading = false;
    }
  }
</script>

<div class="flex-1 overflow-auto p-4">
  {#if loading && teams.length === 0}
    <div class="flex items-center justify-center h-32 text-[var(--color-text-muted)]">Loading…</div>
  {:else if error}
    <div class="text-[var(--color-bundesliga-red)] p-4">{error}</div>
  {:else if teams.length === 0}
    <div class="text-[var(--color-text-muted)] p-4">No teams found.</div>
  {:else}
    <div class="grid grid-cols-4 gap-3">
      {#each teams as team}
        <TeamCard {team} />
      {/each}
    </div>
  {/if}
</div>
