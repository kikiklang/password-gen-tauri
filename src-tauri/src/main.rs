// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]

fn generate_password(length: usize, numbers: bool, lowercase_letters: bool, uppercase_letters: bool, symbols: bool, spaces: bool, exclude_similar_characters: bool, strict: bool) -> String {
    use passwords::PasswordGenerator;
    let pg = PasswordGenerator {
        length,
        numbers,
        lowercase_letters,
        uppercase_letters,
        symbols,
        spaces,
        exclude_similar_characters,
        strict,
    };

    let generated_password = pg.generate_one().unwrap();
    generated_password
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![generate_password])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
