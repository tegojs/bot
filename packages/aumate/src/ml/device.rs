//! Device management for ML inference
//!
//! Provides device selection (CPU, CUDA, Metal) for Candle operations.

#![allow(dead_code)]

use crate::error::Result;
use candle_core::Device as CandleDevice;

/// Re-export Candle's Device type
pub type Device = CandleDevice;

/// Configuration for device selection
#[derive(Debug, Clone, Default)]
pub struct DeviceConfig {
    /// Prefer GPU acceleration if available
    pub prefer_gpu: bool,
    /// Specific CUDA device ordinal (if using CUDA)
    pub cuda_ordinal: usize,
}

impl DeviceConfig {
    /// Create a new device config with default settings
    pub fn new() -> Self {
        Self::default()
    }

    /// Create a config that prefers GPU acceleration
    pub fn with_gpu() -> Self {
        Self { prefer_gpu: true, cuda_ordinal: 0 }
    }

    /// Create a config for CPU only
    pub fn cpu_only() -> Self {
        Self { prefer_gpu: false, cuda_ordinal: 0 }
    }
}

/// Get the appropriate device based on configuration and available hardware
pub fn get_device(config: &DeviceConfig) -> Result<Device> {
    if config.prefer_gpu {
        // Try Metal on macOS
        #[cfg(all(target_os = "macos", feature = "ml-metal"))]
        {
            match CandleDevice::new_metal(0) {
                Ok(device) => {
                    log::info!("Using Metal device for acceleration");
                    return Ok(device);
                }
                Err(e) => {
                    log::warn!("Metal not available, falling back to CPU: {}", e);
                }
            }
        }

        // Try CUDA on Linux/Windows
        #[cfg(feature = "ml-cuda")]
        {
            match CandleDevice::new_cuda(config.cuda_ordinal) {
                Ok(device) => {
                    log::info!("Using CUDA device {} for acceleration", config.cuda_ordinal);
                    return Ok(device);
                }
                Err(e) => {
                    log::warn!("CUDA not available, falling back to CPU: {}", e);
                }
            }
        }
    }

    log::info!("Using CPU device");
    Ok(CandleDevice::Cpu)
}

/// Get the best available device (prefers GPU)
pub fn get_best_device() -> Result<Device> {
    get_device(&DeviceConfig::with_gpu())
}

/// Check if GPU acceleration is available
pub fn is_gpu_available() -> bool {
    #[cfg(all(target_os = "macos", feature = "ml-metal"))]
    {
        if CandleDevice::new_metal(0).is_ok() {
            return true;
        }
    }

    #[cfg(feature = "ml-cuda")]
    {
        if CandleDevice::new_cuda(0).is_ok() {
            return true;
        }
    }

    false
}

/// Get the name of the current device backend
pub fn device_name(device: &Device) -> &'static str {
    match device {
        CandleDevice::Cpu => "CPU",
        CandleDevice::Cuda(_) => "CUDA",
        CandleDevice::Metal(_) => "Metal",
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cpu_device() {
        let config = DeviceConfig::cpu_only();
        let device = get_device(&config).unwrap();
        assert!(matches!(device, Device::Cpu));
    }

    #[test]
    fn test_device_name() {
        let device = Device::Cpu;
        assert_eq!(device_name(&device), "CPU");
    }
}
