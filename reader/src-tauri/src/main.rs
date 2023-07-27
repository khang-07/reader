// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::{Runtime, Window, Manager, WindowBuilder, WindowEvent};
use tauri::{CustomMenuItem, Menu, MenuItem, Submenu};
use cocoa::appkit::{NSWindow, NSWindowStyleMask};
use window_vibrancy::{apply_vibrancy, NSVisualEffectMaterial};

mod myepub;
mod myreader;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn print_from_back(message: &str) {
    println!("Frontend Print: {}", message);
}

#[tauri::command]
fn book_to_json() {
    myepub::book_to_json("/Users/khangnguyen/Documents/Books/Breasts and Eggs.epub");
}

#[tauri::command]
fn get_chapter_titles() -> Vec<String> {
    myreader::get_chapter_titles()
}

#[tauri::command]
fn get_chapter(title: &str) -> Vec<Vec<String>> {
    println!("Getting Chapter: {}", title);
    let mut test = myreader::get_chapter(title);
    // test.pop();
    // test.remove(0);
    test
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
        .invoke_handler(tauri::generate_handler![
            book_to_json, 
            get_chapter_titles, 
            print_from_back,
            get_chapter
        ])
        .setup(|app| {
            let window = app.get_window("main").unwrap();

            #[cfg(target_os = "macos")]
            let _ = apply_vibrancy(&window, NSVisualEffectMaterial::HudWindow, None, Some(16.0));
            let _ = &window.set_transparent_titlebar(true);
            // .expect("Unsupported platform! 'apply_vibrancy' is only supported on macOS");

            myepub::main();
            Ok(())
        })
        .on_window_event(|e| {
            if let WindowEvent::Resized(_) = e.event() {
                println!("hi");
            }
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
