# app-path-over-pathbuf — Tasks

1. [ ] 16.1 Add `use std::path::Path;` to the imports in `app/src-tauri/src/lib.rs` (if `Path` is not already imported)
2. [ ] 16.2 Change the signature of `read_team_cache` from `fn read_team_cache(team_id: i32, app_data_dir: &PathBuf)` to `fn read_team_cache(team_id: i32, app_data_dir: &Path)`
3. [ ] 16.3 Change the signature of `write_team_cache` from `fn write_team_cache(team_id: i32, data: &TeamDetail, app_data_dir: &PathBuf)` to `fn write_team_cache(team_id: i32, data: &TeamDetail, app_data_dir: &Path)`
4. [ ] 16.4 Run `cargo build` to confirm no compilation errors (call sites should compile without changes due to automatic `Deref` coercion)
