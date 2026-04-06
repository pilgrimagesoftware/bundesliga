<script lang="ts">
  import type { Match } from "../../types/Match";

  let { match, onclick }: { match: Match; onclick: () => void } = $props();

  function isLive(m: Match): boolean {
    if (m.is_finished || !m.when_utc) return false;
    const kickoff = new Date(m.when_utc).getTime();
    const now = Date.now();
    return now >= kickoff && now <= kickoff + 2 * 60 * 60 * 1000;
  }

  function score(m: Match): string {
    const final = m.results?.find((r) => r.result_type === 2);
    if (final) return `${final.points_team1} – ${final.points_team2}`;
    const current = m.results?.find((r) => r.result_type === 1);
    if (current) return `${current.points_team1} – ${current.points_team2}`;
    return "– : –";
  }

  function statusLabel(m: Match): string {
    if (m.is_finished) return "FT";
    if (isLive(m)) return "LIVE";
    if (m.when_utc) {
      return new Date(m.when_utc).toLocaleTimeString([], { hour: "2-digit", minute: "2-digit" });
    }
    return "";
  }
</script>

<button
  {onclick}
  class="w-full text-left bg-[var(--color-surface-elevated)] border border-[var(--color-border)] rounded p-3 hover:border-[var(--color-bundesliga-red)] transition-colors cursor-pointer"
>
  <div class="flex items-center justify-between gap-2">
    <!-- Team 1 -->
    <div class="flex items-center gap-2 flex-1 min-w-0">
      {#if match.team1.icon_url}
        <img src={match.team1.icon_url} alt={match.team1.name ?? ""} class="w-6 h-6 object-contain shrink-0" />
      {/if}
      <span class="text-sm truncate">{match.team1.short_name ?? match.team1.name ?? "?"}</span>
    </div>

    <!-- Score / Status -->
    <div class="flex flex-col items-center shrink-0 min-w-[80px]">
      <span class="text-base font-bold tabular-nums">{score(match)}</span>
      <span
        class="text-xs mt-0.5
          {match.is_finished
            ? 'text-[var(--color-text-muted)]'
            : isLive(match)
            ? 'text-green-400 font-bold'
            : 'text-[var(--color-text-muted)]'}"
      >
        {statusLabel(match)}
      </span>
    </div>

    <!-- Team 2 -->
    <div class="flex items-center gap-2 flex-1 min-w-0 justify-end">
      <span class="text-sm truncate text-right">{match.team2.short_name ?? match.team2.name ?? "?"}</span>
      {#if match.team2.icon_url}
        <img src={match.team2.icon_url} alt={match.team2.name ?? ""} class="w-6 h-6 object-contain shrink-0" />
      {/if}
    </div>
  </div>
</button>
