//! Float Window Demo Application

use float_window::prelude::*;

fn main() {
    // Initialize logging
    env_logger::init();

    // Create a circular floating window with particle effects
    let window = FloatingWindow::builder()
        .title("Float Window Demo")
        .size(50, 50)
        .position(100.0, 100.0)
        .shape(WindowShape::Circle)
        .draggable(true)
        .always_on_top(true)
        // .content(Content::text("Hello!"))
        .effect(
            PresetEffect::RotatingHalo,
            PresetEffectOptions::default()
                .with_intensity(0.8)
                .with_colors(vec![
                    [0.4, 0.8, 1.0, 1.0], // Cyan
                    [0.8, 0.4, 1.0, 1.0], // Purple
                    [1.0, 0.8, 0.4, 1.0], // Gold
                ]),
        )
        .on_event(|event| match event {
            FloatingWindowEvent::Click { x, y } => {
                println!("Clicked at ({}, {})", x, y);
            }
            FloatingWindowEvent::DragStart { .. } => {
                println!("Drag started");
            }
            FloatingWindowEvent::DragEnd { .. } => {
                println!("Drag ended");
            }
            FloatingWindowEvent::Close => {
                println!("Window closing");
            }
            _ => {}
        })
        .build()
        .expect("Failed to create window");

    // Run the window (blocking)
    if let Err(e) = window.run() {
        eprintln!("Error running window: {}", e);
    }
}
