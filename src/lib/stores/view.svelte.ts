import { invoke } from "@tauri-apps/api/core";
import type { AppViewState } from "../../types/AppViewState";
import { getLeague, getSeason } from "./context.svelte";

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
  saveCurrentView();
}

export function saveCurrentView() {
  const league = getLeague();
  const season = getSeason();
  if (!league || !season) return;

  const state: AppViewState = {
    last_opened: Math.floor(Date.now() / 1000),
    league: league.shortcut,
    season,
    view: view.screen,
    matchday: "matchday" in view ? view.matchday : null,
    selected_team_id: "teamId" in view ? view.teamId : null,
  };
  void invoke("save_last_viewed", { viewState: state }).catch(() => {});
}
