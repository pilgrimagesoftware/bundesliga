<script lang="ts">
  import { onMount } from "svelte";
  import { invoke } from "@tauri-apps/api/core";
  import { writable, derived, get } from "svelte/store";
  import type { League } from "../types/League";
  import type { TableTeam } from "../types/TableTeam";

  // Stores for leagues, seasons, and results
  const leagues = writable<League[]>([]);
  const selectedLeague = writable<number | null>(null);
  const seasons = writable<number[]>([]);
  const selectedSeason = writable<number | null>(null);
  const results = writable<TableTeam[]>([]);

  // Fetch leagues from Rust backend
  async function fetchLeagues() {
    const data: League[] = await invoke("get_leagues");
    leagues.set(data);
    if (data.length > 0) selectedLeague.set(0);
  }

  // Fetch seasons when a league is selected
  async function fetchSeasons(league: League) {
    const data: number[] = await invoke("get_seasons", { league: league.shortcut });
    seasons.set(data);
    if (data.length > 0) selectedSeason.set(data[0]);
  }

  // Fetch results when league and season are selected
  async function fetchResults(league: League, season: number) {
    const data: TableTeam[] = await invoke("get_results", { league: league.shortcut, season });
    results.set(data);
  }

  // Auto-fetch results every 30 seconds
  let interval: number; // TODO: NodeJS.Timeout;
  function startAutoUpdate() {
    if (interval) clearInterval(interval);
    interval = setInterval(async () => {
      const league = get(leagues)[$selectedLeague];
      const season = get(selectedSeason);
      if (league && season) await fetchResults(league, season);
    }, 30000);
  }

  // Watch for changes and fetch data
  derived(selectedLeague, ($selectedLeague) => {
    if ($selectedLeague) fetchSeasons(get(leagues)[$selectedLeague]);
  });

  derived([selectedLeague, selectedSeason], ([$selectedLeague, $selectedSeason]) => {
    if ($selectedLeague && $selectedSeason) {
      fetchResults(get(leagues)[$selectedLeague], $selectedSeason);
      startAutoUpdate();
    }
  });

  onMount(fetchLeagues);
</script>

<main class="p-4">
  <h1 class="text-2xl font-bold">Soccer League Results</h1>

  <!-- League Selector -->
  <label for="league" class="block mt-4">League:</label>
  <select id="league" bind:value={$selectedLeague} class="border p-2 rounded">
    {#each $leagues as league}
      <option value={league.shortcut}>{league.name}</option>
    {/each}
  </select>

  <!-- Season Selector -->
    <label for="season" class="block mt-4">Season:</label>
    <select id="season" bind:value={$selectedSeason} class="border p-2 rounded">
      {#if $selectedLeague}
      {#each $seasons as season}
        <option value={season}>{season}</option>
      {/each}
      {/if}
    </select>

  <!-- Results Table -->
    <h2 class="mt-6 text-xl font-semibold">Table</h2>
    <table class="w-full mt-2 border">
      <thead>
        <tr class="bg-gray-200">
          <th class="p-2">Position</th>
          <th class="p-2">Logo</th>
          <th class="p-2">Name</th>
          <th class="p-2">Points</th>
          <th class="p-2">Goal Difference</th>
          <th class="p-2">Wins</th>
          <th class="p-2">Losses</th>
          <th class="p-2">Draws</th>
        </tr>
      </thead>
      <tbody>
        {#if $selectedLeague && $selectedSeason}
        {#each $results as result, index}
          <tr class="border-b">
            <td class="p-2">{index + 1}</td>
            <td class="p-2"><img src="{result.teamIconUrl}" alt="{result.teamName} Icon" /></td>
            <td class="p-2">{result.teamName}</td>
            <td class="p-2">{result.points}</td>
            <td class="p-2">{result.goalDifference}</td>
            <td class="p-2">{result.wins}</td>
            <td class="p-2">{result.losses}</td>
            <td class="p-2">{result.draws}</td>
          </tr>
        {/each}
        {/if}
      </tbody>
    </table>
</main>

<style>
  select {
    display: block;
    width: 100%;
    padding: 8px;
    margin-top: 4px;
  }
  table {
    width: 100%;
    border-collapse: collapse;
  }
  th, td {
    border: 1px solid #ddd;
    padding: 8px;
  }
  th {
    background-color: #f4f4f4;
  }
</style>
