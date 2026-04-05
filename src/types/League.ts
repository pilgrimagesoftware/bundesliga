import type { Sport } from './Sport';

export interface League {
    id: number;
    name: string;
    shortcut: string;
    season: number;
    sport: Sport;
}
