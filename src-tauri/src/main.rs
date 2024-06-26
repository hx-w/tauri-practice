#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use tauri::api::shell;
use tauri::{ CustomMenuItem, Manager, Menu, Submenu };
mod api_handler;
mod converter;


#[tokio::main]
async fn main() {
    tauri::async_runtime::set(tokio::runtime::Handle::current());
    let ctx = tauri::generate_context!();
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            // api_handler::backend_restore_full,
            // api_handler::backend_register_obj,
            // api_handler::backend_restore_preprocess,
            // api_handler::backend_restore_embedding,
            // api_handler::backend_restore_download,
            api_handler::backend_load_obj,
            api_handler::backend_segment_jaw
        ])
        .menu(
            tauri::Menu::os_default("BeauTee").add_submenu(Submenu::new(
                "Help",
                Menu::with_items([CustomMenuItem::new(
                    "Online Documentation",
                    "Online Documentation",
                )
                .into()]),
            )),
        )
        .on_menu_event(|event| {
            let event_name: &str = event.menu_item_id();
            match event_name {
                "Online Documentation" => {
                    let url = "https://github.com/hx-w/tauri-practice".to_string();
                    shell::open(&event.window().shell_scope(), url, None).unwrap();
                }
                _ => {}
            }
        })
        .setup(|_app| {
            #[cfg(debug_assertions)]
            {
                let main_window = _app.get_window("main").unwrap();
                main_window.open_devtools();
            }
            Ok(())
        })
        .run(ctx)
        .expect("error while running tauri application");
}
