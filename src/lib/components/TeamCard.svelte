<script lang="ts">
  import type { Team } from "../../types/Team";
  import { navigate } from "../stores/view.svelte";

  let { team }: { team: Team } = $props();
</script>

<button
  onclick={() => navigate({ screen: "team_detail", teamId: team.id })}
  class="flex flex-col items-center gap-2 p-4 bg-[var(--color-surface-elevated)] border border-[var(--color-border)] rounded hover:border-[var(--color-bundesliga-red)] transition-colors cursor-pointer"
>
  {#if team.icon_url}
    <img src={team.icon_url} alt={team.name ?? ""} class="w-12 h-12 object-contain" />
  {:else}
    <div class="w-12 h-12 rounded-full bg-[var(--color-surface)] flex items-center justify-center text-[var(--color-text-muted)] text-lg">🛡️</div>
  {/if}
  <span class="text-xs text-center leading-tight">{team.short_name ?? team.name ?? "?"}</span>
</button>
