export interface TableTeam {
  id: number;
  name: string | null;
  short_name: string | null;
  icon_url: string | null;
  points: number;
  opponent_goals: number;
  goals: number;
  matches: number;
  wins: number;
  draws: number;
  losses: number;
  goal_difference: number;
}
