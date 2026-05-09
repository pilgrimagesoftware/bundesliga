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
    <div class="w-12 h-12 rounded-full bg-[var(--color-surface)] flex items-center justify-center text-[var(--color-text-muted)]">
      <svg class="h-7 w-7" viewBox="0 0 20 20" fill="none" aria-hidden="true">
        <path d="M10 3.5 15.5 6v4.2c0 3.1-2.2 5.2-5.5 6.3-3.3-1.1-5.5-3.2-5.5-6.3V6L10 3.5Z" stroke="currentColor" stroke-width="1.6" stroke-linejoin="round" />
        <path d="M7.5 10h5" stroke="currentColor" stroke-width="1.6" stroke-linecap="round" />
      </svg>
    </div>
  {/if}
  <span class="text-xs text-center leading-tight">{team.short_name ?? team.name ?? "?"}</span>
</button>
