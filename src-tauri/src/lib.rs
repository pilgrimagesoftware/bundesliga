#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use chrono::Datelike;
use chrono::Local;
use openligadb::models::goal::GoalGetter;
use openligadb::models::group::Group;
use openligadb::models::league::League;
use openligadb::models::r#match::Match;
use openligadb::models::table::TableTeam;
use openligadb::models::team::Team;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::Mutex;
use std::time::{Instant, SystemTime, UNIX_EPOCH};
use tauri::{Manager, State};

// ─── AppState ────────────────────────────────────────────────────────────────

pub struct AppState {
    cooldown_tracker: HashMap<&'static str, Instant>,
    last_responses: HashMap<&'static str, serde_json::Value>,
    pub app_data_dir: PathBuf,
}

impl AppState {
    fn new(app_data_dir: PathBuf) -> Self {
        std::fs::create_dir_all(&app_data_dir).ok();
        AppState {
            cooldown_tracker: HashMap::new(),
            last_responses: HashMap::new(),
            app_data_dir,
        }
    }

    /// Returns (is_on_cooldown, next_refresh_at_epoch_ms)
    fn check_cooldown(&self, category: &str, min_secs: u64) -> (bool, Option<u64>) {
        if let Some(last) = self.cooldown_tracker.get(category) {
            let elapsed = last.elapsed().as_secs();
            if elapsed < min_secs {
                let remaining_ms = (min_secs - elapsed) * 1000;
                let next_refresh_at = epoch_ms() + remaining_ms;
                return (true, Some(next_refresh_at));
            }
        }
        (false, None)
    }

    fn update_cooldown(&mut self, category: &'static str) {
        self.cooldown_tracker.insert(category, Instant::now());
    }

    fn store_response(&mut self, category: &'static str, value: serde_json::Value) {
        self.last_responses.insert(category, value);
    }

    fn get_cached_response(&self, category: &str) -> Option<&serde_json::Value> {
        self.last_responses.get(category)
    }
}

fn epoch_ms() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_millis() as u64
}

// ─── CachedResponse ──────────────────────────────────────────────────────────

#[derive(Debug, Serialize, Deserialize)]
pub struct CachedResponse<T> {
    pub data: T,
    pub cached: bool,
    pub next_refresh_at: Option<u64>,
}

// ─── AppViewState ─────────────────────────────────────────────────────────────

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AppViewState {
    pub last_opened: i64,
    pub league: String,
    pub season: i32,
    pub view: String,
    pub matchday: Option<i32>,
    pub selected_team_id: Option<i32>,
}

// ─── TheSportsDB structs ──────────────────────────────────────────────────────

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TheSportsDbTeam {
    #[serde(rename = "idTeam")]
    pub id: Option<String>,
    #[serde(rename = "strTeam")]
    pub name: Option<String>,
    #[serde(rename = "intFormedYear")]
    pub founded: Option<String>,
    #[serde(rename = "strStadium")]
    pub stadium: Option<String>,
    #[serde(rename = "intStadiumCapacity")]
    pub capacity: Option<String>,
    #[serde(rename = "strDescriptionEN")]
    pub description: Option<String>,
    #[serde(rename = "strCountry")]
    pub country: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TheSportsDbPlayer {
    #[serde(rename = "strPlayer")]
    pub name: Option<String>,
    #[serde(rename = "strPosition")]
    pub position: Option<String>,
    #[serde(rename = "strNationality")]
    pub nationality: Option<String>,
    #[serde(rename = "dateBorn")]
    pub date_of_birth: Option<String>,
    #[serde(rename = "strCutout")]
    pub photo_url: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TheSportsDbStaff {
    pub name: String,
    pub role: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TeamDetail {
    pub id: i32,
    pub name: Option<String>,
    pub short_name: Option<String>,
    pub icon_url: Option<String>,
    pub founded: Option<String>,
    pub stadium: Option<String>,
    pub capacity: Option<String>,
    pub description: Option<String>,
    pub squad: Vec<TheSportsDbPlayer>,
    pub staff: Vec<TheSportsDbStaff>,
    pub squad_source_found: bool,
}

#[derive(Debug, Serialize, Deserialize)]
struct TeamCacheFile {
    cached_at: i64,
    data: TeamDetail,
}

#[derive(Debug, Deserialize)]
struct TsdbSearchResponse {
    teams: Option<Vec<TheSportsDbTeam>>,
}

#[derive(Debug, Deserialize)]
struct TsdbPlayersResponse {
    player: Option<Vec<TheSportsDbPlayer>>,
}

// ─── TheSportsDB helpers ──────────────────────────────────────────────────────

async fn search_thesportsdb_team(name: &str) -> Option<TheSportsDbTeam> {
    let encoded = urlencoding::encode(name);
    let url = format!(
        "https://www.thesportsdb.com/api/v1/json/3/searchteams.php?t={}",
        encoded
    );
    let resp = reqwest::get(&url).await.ok()?;
    let body: TsdbSearchResponse = resp.json().await.ok()?;
    let teams = body.teams?;
    if teams.is_empty() {
        return None;
    }

    let name_lower = name.to_lowercase();
    let mut best: Option<(f64, TheSportsDbTeam)> = None;
    for team in teams {
        if let Some(team_name) = &team.name {
            let score = strsim::jaro_winkler(&name_lower, &team_name.to_lowercase());
            if best.as_ref().map_or(true, |(s, _)| score > *s) {
                best = Some((score, team));
            }
        }
    }

    best.and_then(|(score, team)| if score > 0.75 { Some(team) } else { None })
}

async fn fetch_thesportsdb_players(tsdb_team_id: &str) -> Vec<TheSportsDbPlayer> {
    let url = format!(
        "https://www.thesportsdb.com/api/v1/json/3/lookup_all_players.php?id={}",
        tsdb_team_id
    );
    let resp = match reqwest::get(&url).await {
        Ok(r) => r,
        Err(_) => return vec![],
    };
    let body: TsdbPlayersResponse = match resp.json().await {
        Ok(b) => b,
        Err(_) => return vec![],
    };
    body.player.unwrap_or_default()
}

fn read_team_cache(team_id: i32, app_data_dir: &PathBuf) -> Option<TeamDetail> {
    let path = app_data_dir
        .join("team_cache")
        .join(format!("{}.json", team_id));
    let content = std::fs::read_to_string(&path).ok()?;
    let cache: TeamCacheFile = serde_json::from_str(&content).ok()?;
    let now = Local::now().timestamp();
    let thirty_days: i64 = 30 * 24 * 3600;
    if now - cache.cached_at > thirty_days {
        return None;
    }
    Some(cache.data)
}

fn write_team_cache(team_id: i32, data: &TeamDetail, app_data_dir: &PathBuf) {
    let cache_dir = app_data_dir.join("team_cache");
    std::fs::create_dir_all(&cache_dir).ok();
    let path = cache_dir.join(format!("{}.json", team_id));
    let cache = TeamCacheFile {
        cached_at: Local::now().timestamp(),
        data: data.clone(),
    };
    if let Ok(json) = serde_json::to_string(&cache) {
        std::fs::write(path, json).ok();
    }
}

// ─── Tauri Commands ───────────────────────────────────────────────────────────

#[tauri::command]
async fn get_leagues() -> Result<Vec<League>, String> {
    League::list().await.map_err(|e| e.to_string())
}

#[tauri::command]
async fn get_seasons() -> Result<Vec<i32>, String> {
    let current = Local::now().year();
    Ok(vec![current, current - 1, current - 2, current - 3])
}

#[tauri::command]
async fn get_table(
    league: String,
    season: i32,
    state: State<'_, Mutex<AppState>>,
) -> Result<CachedResponse<Vec<TableTeam>>, String> {
    const CAT: &str = "table";
    const COOLDOWN: u64 = 60;

    let (on_cooldown, next_refresh_at) = {
        let s = state.lock().unwrap();
        s.check_cooldown(CAT, COOLDOWN)
    };

    if on_cooldown {
        let cached = state.lock().unwrap().get_cached_response(CAT).cloned();
        if let Some(val) = cached {
            let data: Vec<TableTeam> = serde_json::from_value(val).map_err(|e| e.to_string())?;
            return Ok(CachedResponse {
                data,
                cached: true,
                next_refresh_at,
            });
        }
    }

    let data = TableTeam::get_bl_table(&league, season)
        .await
        .map_err(|e| e.to_string())?;

    {
        let mut s = state.lock().unwrap();
        s.update_cooldown(CAT);
        s.store_response(CAT, serde_json::to_value(&data).unwrap_or_default());
    }

    Ok(CachedResponse {
        data,
        cached: false,
        next_refresh_at: None,
    })
}

#[tauri::command]
async fn get_matchdays(
    league: String,
    season: i32,
    state: State<'_, Mutex<AppState>>,
) -> Result<CachedResponse<Vec<Group>>, String> {
    const CAT: &str = "matchdays";
    const COOLDOWN: u64 = 300;

    let (on_cooldown, next_refresh_at) = {
        let s = state.lock().unwrap();
        s.check_cooldown(CAT, COOLDOWN)
    };

    if on_cooldown {
        let cached = state.lock().unwrap().get_cached_response(CAT).cloned();
        if let Some(val) = cached {
            let data: Vec<Group> = serde_json::from_value(val).map_err(|e| e.to_string())?;
            return Ok(CachedResponse {
                data,
                cached: true,
                next_refresh_at,
            });
        }
    }

    let data = Group::available(&league, season)
        .await
        .map_err(|e| e.to_string())?;

    {
        let mut s = state.lock().unwrap();
        s.update_cooldown(CAT);
        s.store_response(CAT, serde_json::to_value(&data).unwrap_or_default());
    }

    Ok(CachedResponse {
        data,
        cached: false,
        next_refresh_at: None,
    })
}

#[tauri::command]
async fn get_current_matchday(league: String) -> Result<Group, String> {
    Group::current(&league).await.map_err(|e| e.to_string())
}

#[tauri::command]
async fn get_matches_for_matchday(
    league: String,
    season: i32,
    group_order_id: i32,
    state: State<'_, Mutex<AppState>>,
) -> Result<CachedResponse<Vec<Match>>, String> {
    const CAT: &str = "match_data";
    const COOLDOWN: u64 = 30;

    let (on_cooldown, next_refresh_at) = {
        let s = state.lock().unwrap();
        s.check_cooldown(CAT, COOLDOWN)
    };

    if on_cooldown {
        let cached = state.lock().unwrap().get_cached_response(CAT).cloned();
        if let Some(val) = cached {
            let data: Vec<Match> = serde_json::from_value(val).map_err(|e| e.to_string())?;
            return Ok(CachedResponse {
                data,
                cached: true,
                next_refresh_at,
            });
        }
    }

    let data = Match::by_league_group(&league, season, group_order_id)
        .await
        .map_err(|e| e.to_string())?;

    {
        let mut s = state.lock().unwrap();
        s.update_cooldown(CAT);
        s.store_response(CAT, serde_json::to_value(&data).unwrap_or_default());
    }

    Ok(CachedResponse {
        data,
        cached: false,
        next_refresh_at: None,
    })
}

#[tauri::command]
async fn get_match_detail(match_id: i32) -> Result<Match, String> {
    Match::get(match_id).await.map_err(|e| e.to_string())
}

#[tauri::command]
async fn get_teams(league: String, season: i32) -> Result<Vec<Team>, String> {
    Team::available(&league, season)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
async fn get_top_scorers(league: String, season: i32) -> Result<Vec<GoalGetter>, String> {
    GoalGetter::list(&league, season)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
async fn get_last_viewed(
    state: State<'_, Mutex<AppState>>,
) -> Result<Option<AppViewState>, String> {
    let path = {
        let s = state.lock().unwrap();
        s.app_data_dir.join("app_state.json")
    };
    match std::fs::read_to_string(&path) {
        Ok(content) => {
            let parsed: AppViewState =
                serde_json::from_str(&content).map_err(|e| e.to_string())?;
            Ok(Some(parsed))
        }
        Err(_) => Ok(None),
    }
}

#[tauri::command]
async fn save_last_viewed(
    view_state: AppViewState,
    state: State<'_, Mutex<AppState>>,
) -> Result<(), String> {
    let path = {
        let s = state.lock().unwrap();
        s.app_data_dir.join("app_state.json")
    };
    let json = serde_json::to_string(&view_state).map_err(|e| e.to_string())?;
    std::fs::write(path, json).map_err(|e| e.to_string())
}

#[tauri::command]
async fn get_team_detail(
    team_id: i32,
    team_name: String,
    _league: String,
    _season: i32,
    state: State<'_, Mutex<AppState>>,
) -> Result<CachedResponse<TeamDetail>, String> {
    const CAT: &str = "team_detail";
    const COOLDOWN: u64 = 300;

    let app_data_dir = {
        let s = state.lock().unwrap();
        s.app_data_dir.clone()
    };

    if let Some(cached_detail) = read_team_cache(team_id, &app_data_dir) {
        let (on_cooldown, next_refresh_at) = {
            let s = state.lock().unwrap();
            s.check_cooldown(CAT, COOLDOWN)
        };
        return Ok(CachedResponse {
            data: cached_detail,
            cached: true,
            next_refresh_at: if on_cooldown { next_refresh_at } else { None },
        });
    }

    let (on_cooldown, next_refresh_at) = {
        let s = state.lock().unwrap();
        s.check_cooldown(CAT, COOLDOWN)
    };

    if on_cooldown {
        let detail = TeamDetail {
            id: team_id,
            name: Some(team_name),
            short_name: None,
            icon_url: None,
            founded: None,
            stadium: None,
            capacity: None,
            description: None,
            squad: vec![],
            staff: vec![],
            squad_source_found: false,
        };
        return Ok(CachedResponse {
            data: detail,
            cached: true,
            next_refresh_at,
        });
    }

    let tsdb_team = search_thesportsdb_team(&team_name).await;
    let (squad, founded, stadium, capacity, description, squad_source_found) =
        if let Some(ref tsdb) = tsdb_team {
            let players = if let Some(ref id) = tsdb.id {
                fetch_thesportsdb_players(id).await
            } else {
                vec![]
            };
            (
                players,
                tsdb.founded.clone(),
                tsdb.stadium.clone(),
                tsdb.capacity.clone(),
                tsdb.description.clone(),
                true,
            )
        } else {
            (vec![], None, None, None, None, false)
        };

    let detail = TeamDetail {
        id: team_id,
        name: Some(team_name),
        short_name: None,
        icon_url: None,
        founded,
        stadium,
        capacity,
        description,
        squad,
        staff: vec![],
        squad_source_found,
    };

    write_team_cache(team_id, &detail, &app_data_dir);

    {
        let mut s = state.lock().unwrap();
        s.update_cooldown(CAT);
    }

    Ok(CachedResponse {
        data: detail,
        cached: false,
        next_refresh_at: None,
    })
}

// ─── App Entry Point ──────────────────────────────────────────────────────────

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_window_state::Builder::new().build())
        .plugin(tauri_plugin_log::Builder::new().build())
        .setup(|app| {
            let app_data_dir = app
                .path()
                .app_data_dir()
                .expect("failed to get app data dir");
            app.manage(Mutex::new(AppState::new(app_data_dir)));
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            get_leagues,
            get_seasons,
            get_table,
            get_matchdays,
            get_current_matchday,
            get_matches_for_matchday,
            get_match_detail,
            get_teams,
            get_top_scorers,
            get_last_viewed,
            save_last_viewed,
            get_team_detail,
        ])
        .run(tauri::generate_context!())
        .expect("error while running Tauri application");
}
