import type { Player } from "./Player";

export interface Staff {
  name: string;
  role: string;
}

export interface TeamDetail {
  id: number;
  name: string | null;
  short_name: string | null;
  icon_url: string | null;
  founded: string | null;
  stadium: string | null;
  capacity: string | null;
  description: string | null;
  squad: Player[];
  staff: Staff[];
  squad_source_found: boolean;
}
