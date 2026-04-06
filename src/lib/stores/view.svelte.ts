import { invoke } from "@tauri-apps/api/core";
import type { AppViewState } from "../../types/AppViewState";

export type AppView =
  | { screen: "table" }
  | { screen: "matches"; matchday: number }
  | { screen: "match_detail"; matchId: number; fromMatchday: number }
  | { screen: "teams" }
  | { screen: "team_detail"; teamId: number };

let view = $state<AppView>({ screen: "table" });

export function getView() {
  return view;
}

export function navigate(next: AppView) {
  view = next;
}

export async function navigateAndSave(
  next: AppView,
  league: string,
  season: number
) {
  view = next;
  const state: AppViewState = {
    last_opened: Math.floor(Date.now() / 1000),
    league,
    season,
    view: next.screen,
    matchday: "matchday" in next ? next.matchday : null,
    selected_team_id: "teamId" in next ? next.teamId : null,
  };
  await invoke("save_last_viewed", { viewState: state }).catch(() => {});
}
