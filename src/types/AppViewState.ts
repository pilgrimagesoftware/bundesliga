export interface AppViewState {
  last_opened: number;
  league: string;
  season: number;
  view: string;
  matchday: number | null;
  selected_team_id: number | null;
}
