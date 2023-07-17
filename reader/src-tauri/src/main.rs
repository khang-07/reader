// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::{Runtime, Window, Manager, WindowBuilder};
use tauri::{CustomMenuItem, Menu, MenuItem, Submenu};
use cocoa::appkit::{NSWindow, NSWindowStyleMask};
use window_vibrancy::{apply_blur, apply_vibrancy, NSVisualEffectMaterial};

mod myepub;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn abc(index: i64) -> String {
    let chapters = myepub::get_book("/Users/khangnguyen/Code/testing epub rust/foo/src/kafka.epub");
    let i = (index + 2) as usize; // chapter 1 is index1 and not index0
    let text = chapters[i].join(" ");
    println!("{:?}", chapters[i]);
    text
}

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

fn main() { 
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![abc])
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
