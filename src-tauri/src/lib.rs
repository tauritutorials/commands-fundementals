use tauri::Manager;

mod commands;

pub struct AppState {
    pub foo: String,
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![
            commands::greet,
            commands::greet2,
            commands::save_preferences,
            commands::blocking_cmd,
            commands::non_blocking_cmd,
            commands::using_app_handle,
            commands::hide_window,
            commands::using_state,
            commands::errors
        ])
        .setup(|app| {
            app.manage(AppState { foo: "Bar".into() });

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
