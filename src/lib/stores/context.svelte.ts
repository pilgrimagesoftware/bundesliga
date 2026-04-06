import type { League } from "../../types/League";

let league = $state<League | null>(null);
let season = $state<number | null>(null);

export function getLeague() {
  return league;
}

export function getSeason() {
  return season;
}

export function setLeague(l: League) {
  league = l;
}

export function setSeason(s: number) {
  season = s;
}
