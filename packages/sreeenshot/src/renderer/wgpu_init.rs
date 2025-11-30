use std::sync::Arc;
use winit::window::Window;
use wgpu;

/// 初始化 WGPU 资源
pub fn init_wgpu(
    window: &Arc<Window>,
    width: u32,
    height: u32,
) -> anyhow::Result<(
    wgpu::Surface<'static>,
    wgpu::Device,
    wgpu::Queue,
    wgpu::SurfaceConfiguration,
)> {
    // 创建 WGPU 实例
    let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
        backends: wgpu::Backends::PRIMARY,
        ..Default::default()
    });
    
    // 创建表面
    let surface = instance
        .create_surface(window.clone())
        .map_err(|e| anyhow::anyhow!("Failed to create surface: {:?}", e))?;
    
    // 请求适配器
    let adapter = pollster::block_on(instance.request_adapter(&wgpu::RequestAdapterOptions {
        power_preference: wgpu::PowerPreference::HighPerformance,
        compatible_surface: Some(&surface),
        force_fallback_adapter: false,
    }))
    .ok_or_else(|| anyhow::anyhow!("Failed to find suitable adapter"))?;
    
    // 请求设备和队列
    let (device, queue) = pollster::block_on(adapter.request_device(
        &wgpu::DeviceDescriptor {
            label: None,
            required_features: wgpu::Features::empty(),
            required_limits: wgpu::Limits::default(),
        },
        None,
    ))
    .map_err(|e| anyhow::anyhow!("Failed to request device: {:?}", e))?;
    
    // 配置表面
    let config = create_surface_config(&surface, &adapter, width, height);
    surface.configure(&device, &config);
    
    Ok((surface, device, queue, config))
}

/// 创建表面配置
fn create_surface_config(
    surface: &wgpu::Surface,
    adapter: &wgpu::Adapter,
    width: u32,
    height: u32,
) -> wgpu::SurfaceConfiguration {
    let surface_caps = surface.get_capabilities(adapter);
    
    // 优先选择 sRGB 格式
    let surface_format = surface_caps
        .formats
        .iter()
        .copied()
        .find(|f| f.is_srgb())
        .unwrap_or(surface_caps.formats[0]);
    
    wgpu::SurfaceConfiguration {
        usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
        format: surface_format,
        width,
        height,
        present_mode: surface_caps.present_modes[0],
        alpha_mode: surface_caps.alpha_modes[0],
        view_formats: vec![],
        desired_maximum_frame_latency: 2,
    }
}

