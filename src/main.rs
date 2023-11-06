#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use eframe::{egui, epaint::Pos2};
#[cfg(debug_assertions)]
use tracing::trace;

pub mod app_state;
pub mod utils;

fn main() -> eframe::Result<()> {
    #[cfg(debug_assertions)]
    {
        utils::initialize_logging();
        trace!("Logging successfully initialized");
    }

    // Start up window options
    // Without title bar or borders
    // Initial position top left corner
    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(320.0, 240.0)),
        always_on_top: true,
        decorated: false,
        initial_window_pos: Some(Pos2 { x: 0.0, y: 0.0 }),
        active: true,
        transparent: true,
        ..Default::default()
    };

    eframe::run_native(
        "Notes",
        options,
        Box::new(|cc| Box::new(app_state::NotesApp::new(cc))),
    )
}
