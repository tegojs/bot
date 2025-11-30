//! Float Window Demo Application - Controller Mode

use float_window::prelude::*;

fn main() {
    // Initialize logging
    env_logger::init();

    // Create controller window (rectangular, with egui UI)
    let controller = FloatingWindow::builder()
        .title("Flow Window Controller")
        .size(800, 450)
        .position(100.0, 100.0)
        .shape(WindowShape::Rectangle)
        .draggable(true)
        .always_on_top(true)
        .build()
        .expect("Failed to create controller window");

    // Run as controller - this enables dynamic window creation/management
    if let Err(e) = FloatingWindow::run_controller(controller) {
        eprintln!("Error running controller: {}", e);
    }
}
