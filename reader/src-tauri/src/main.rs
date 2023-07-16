// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::{Runtime, Window, Manager, WindowBuilder};
use tauri::{CustomMenuItem, Menu, MenuItem, Submenu};
use cocoa::appkit::{NSWindow, NSWindowStyleMask};
use window_vibrancy::{apply_blur, apply_vibrancy, NSVisualEffectMaterial};

mod myepub;

pub trait WindowExt {
    #[cfg(target_os = "macos")]
    fn set_transparent_titlebar(&self, transparent: bool);
}

impl<R: Runtime> WindowExt for Window<R> {
    #[cfg(target_os = "macos")]
    fn set_transparent_titlebar(&self, transparent: bool) {
        use cocoa::appkit::NSWindowTitleVisibility;
  
        unsafe {
            let id = self.ns_window().unwrap() as cocoa::base::id;

            id.setMovableByWindowBackground_(true);
  
            let mut style_mask = id.styleMask();
            style_mask.set(
                NSWindowStyleMask::NSFullSizeContentViewWindowMask,
                transparent,
            );

            id.setStyleMask_(style_mask);
  
            id.setTitleVisibility_(if transparent {
                NSWindowTitleVisibility::NSWindowTitleHidden
            } else {
                NSWindowTitleVisibility::NSWindowTitleVisible
            });
            
            id.setTitlebarAppearsTransparent_(if transparent {
            cocoa::base::YES
            } else {
                cocoa::base::NO
            });
        }
    }
}

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

fn main() { 
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet])
        .setup(|app| {
            let window = app.get_window("main").unwrap();

            #[cfg(target_os = "macos")]
            let _ = apply_vibrancy(&window, NSVisualEffectMaterial::HudWindow, None, None);
            let _ = &window.set_transparent_titlebar(true);
            // .expect("Unsupported platform! 'apply_vibrancy' is only supported on macOS");

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
