//! Aumate GUI Application
//!
//! This is the main entry point for the aumate GUI application.
//! It launches the unified GUI with floating windows and screenshot functionality.

fn main() -> anyhow::Result<()> {
    env_logger::init();

    log::info!("Starting Aumate GUI...");

    #[cfg(feature = "gui")]
    {
        aumate::gui::run()?;
        Ok(())
    }

    #[cfg(not(feature = "gui"))]
    {
        eprintln!("GUI feature is not enabled. Please compile with --features gui");
        std::process::exit(1);
    }
}
