use muda::{Menu, MenuItem, PredefinedMenuItem, MenuEvent};
use tray_icon::{TrayIcon, TrayIconBuilder};
use std::sync::{Arc, Mutex};
use crate::State;

pub struct TrayHandler {
    _tray: TrayIcon,
    status_item: MenuItem,
    lock_item: MenuItem,
    quit_item: MenuItem,
}

impl TrayHandler {
    pub fn new() -> (Self, Menu) {
        let menu = Menu::new();
        let status_item = MenuItem::new("Status: Unknown", false, None);
        let lock_item = MenuItem::new("Lock", true, None);
        let quit_item = MenuItem::new("Quit", true, None);

        let _ = menu.append_items(&[
            &status_item,
            &PredefinedMenuItem::separator(),
            &lock_item,
            &PredefinedMenuItem::separator(),
            &quit_item,
        ]);

        // Just use a dummy/blank icon for now. In a real app, we'd load a PNG/ICO.
        // For macOS/Windows, tray-icon requires an Icon object.
        let icon = load_icon();

        let tray = TrayIconBuilder::new()
            .with_menu(Box::new(menu.clone()))
            .with_tooltip("JKI Agent")
            .with_icon(icon)
            .build()
            .unwrap();

        (
            Self {
                _tray: tray,
                status_item,
                lock_item,
                quit_item,
            },
            menu,
        )
    }

    pub fn update_status(&self, state: &State) {
        let text = if state.secrets.is_some() {
            "Status: Unlocked"
        } else {
            "Status: Locked"
        };
        self.status_item.set_text(text);
    }

    pub fn handle_menu_event(&self, event: MenuEvent, state: Arc<Mutex<State>>) -> bool {
        if event.id == self.lock_item.id() {
            println!("Tray: Lock requested");
            let mut s = state.lock().unwrap();
            s.secrets = None;
            s.master_key = None;
            s.last_unlocked = None;
            self.update_status(&s);
            false
        } else if event.id == self.quit_item.id() {
            println!("Tray: Quit requested");
            true // Signal to quit
        } else {
            false
        }
    }
}

fn load_icon() -> tray_icon::Icon {
    // Load embedded PNG data
    let icon_bytes = include_bytes!("../assets/icon.png");
    let image = image::load_from_memory(icon_bytes)
        .expect("Failed to load icon from assets/icon.png")
        .into_rgba8();
    
    let (width, height) = image.dimensions();
    let rgba = image.into_raw();
    
    tray_icon::Icon::from_rgba(rgba, width, height).unwrap()
}
