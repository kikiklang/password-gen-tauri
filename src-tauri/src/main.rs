// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]


fn generate_password(length: usize) -> String {
    use rand::{thread_rng, Rng};

    const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789!@#$%^&*()";
    let charset_length = CHARSET.len();
    
    let mut rng = thread_rng();
    let password: String = (0..length)
        .map(|_| {
            let random_index = rng.gen_range(0..charset_length);
            CHARSET[random_index] as char
        })
        .collect();

    password
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![generate_password])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
