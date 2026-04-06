import type { Goal } from "./Goal";
import type { Group } from "./Group";
import type { MatchResult } from "./MatchResult";
import type { Team } from "./Team";

export interface Match {
  id: number;
  when_utc: string | null;
  is_finished: boolean;
  team1: Team;
  team2: Team;
  results: MatchResult[] | null;
  goals: Goal[] | null;
  location: { location_id: number; location_city: string | null; location_stadium: string | null } | null;
  group: Group;
  number_of_viewers: number | null;
}
