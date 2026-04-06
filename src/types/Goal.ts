export interface Goal {
  id: number;
  score_team1: number | null;
  score_team2: number | null;
  match_minute: number | null;
  goal_getter_name: string | null;
  is_penalty: boolean | null;
  is_own_goal: boolean | null;
  is_overtime: boolean | null;
}
