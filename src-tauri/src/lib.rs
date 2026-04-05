#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use openligadb::models::league::League;
use openligadb::models::table::TableTeam;
use tauri::State;

#[derive(Debug)]
struct BundesligaState {
    league: String,
    season: i32,
}

#[tauri::command]
async fn get_leagues(state: State<'_, BundesligaState>) -> Result<Vec<League>, String> {
    dbg!(&state);
    League::list().await.map_err(|e| e.to_string())
}

#[tauri::command]
async fn get_seasons(state: State<'_, BundesligaState>) -> Result<Vec<i32>, String> {
    dbg!(&state);
    Ok(vec![2024])
}

#[tauri::command]
async fn get_table(state: State<'_, BundesligaState>) -> Result<Vec<TableTeam>, String> {
    dbg!(&state);
    let response = openligadb::models::table::TableTeam::get_bl_table(&state.league, state.season)
        .await
        .map_err(|e| e.to_string())?;

    Ok(response)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_window_state::Builder::new().build())
        .plugin(tauri_plugin_log::Builder::new().build())
        .manage(BundesligaState {
            league: "bl1".to_string(),
            season: 2024,
        })
        .invoke_handler(tauri::generate_handler![get_table, get_leagues, get_seasons])
        .run(tauri::generate_context!())
        .expect("error while running Tauri application");
}
