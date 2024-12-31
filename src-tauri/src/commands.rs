// Basic Greet

#[tauri::command]
pub fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}


// Arguments

#[tauri::command]
pub fn greet2(first_name: &str, last_name: &str) -> String {
    format!("The name is {last_name}, {first_name} {last_name}")
}

// if you want snake_case in js then use.
// #[tauri::command(rename_all = "snake_case")]

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Preferences {
    pub first_name: String,
    pub theme: Theme,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum Theme {
    Light,
    Dark,
}

#[tauri::command]
pub fn save_preferences(preferences: Preferences) {
    println!("Saving user's preferences {preferences:#?}");
}

// Async

use std::{thread::sleep, time::Duration};

#[tauri::command]
pub fn blocking_cmd() -> String {
    sleep(Duration::from_secs(3));

    "Done!".into()
}

#[tauri::command]
pub async fn non_blocking_cmd() -> String {
    sleep(Duration::from_secs(3));

    "Done!".into()
}

// Getting the AppHandle

use tauri::{AppHandle, Manager};

#[tauri::command]
pub fn using_app_handle(app: AppHandle) -> tauri::Result<()> {
    let dir = app.path().app_data_dir()?;

    println!("the data dir is located at {dir:?}");
    // do more things.

    Ok(())
}


// Getting the Window.

#[tauri::command]
pub async fn hide_window(window: tauri::WebviewWindow) {
    window.hide();
}


// Getting State

use crate::AppState;

#[tauri::command]
pub fn using_state(state: tauri::State<'_, AppState>) {
    println!("app state: {}", state.foo);
}

// #[tauri::command]
// pub async fn using_state(state: tauri::State<'_, AppState>) -> Result<(), ()> {
//     println!("app state: {}", state.foo);
// }

// Errors

#[tauri::command]
pub fn errors() -> Result<(), String> {
    Err("Bad chicken, mess you up!".into())
}
