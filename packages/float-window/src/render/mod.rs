//! Rendering backend

mod painter;

#[cfg(feature = "gpu")]
mod wgpu_backend;

pub use painter::WindowPainter;

#[cfg(feature = "gpu")]
pub use wgpu_backend::WgpuBackend;
