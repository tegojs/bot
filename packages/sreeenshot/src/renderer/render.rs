use egui_wgpu::ScreenDescriptor;
use wgpu;

/// 将 Egui 输出渲染到 WGPU
pub fn render_to_wgpu(
    surface: &wgpu::Surface,
    device: &wgpu::Device,
    queue: &wgpu::Queue,
    egui_renderer: &mut egui_wgpu::Renderer,
    egui_context: &egui::Context,
    egui_output: &egui::FullOutput,
    width: u32,
    height: u32,
    scale_factor: f64,
) -> anyhow::Result<()> {
    let screen_descriptor = ScreenDescriptor {
        size_in_pixels: [width, height],
        pixels_per_point: scale_factor as f32,
    };

    // 将形状转换为绘制任务
    let paint_jobs = egui_context.tessellate(
        egui_output.shapes.clone(),
        scale_factor as f32,
    );
    
    // 更新纹理
    update_textures(egui_renderer, device, queue, &egui_output.textures_delta)?;
    
    // 获取表面纹理
    let output = surface
        .get_current_texture()
        .map_err(|e| anyhow::anyhow!("Failed to get current texture: {:?}", e))?;
    let view = output
        .texture
        .create_view(&wgpu::TextureViewDescriptor::default());
    
    // 创建命令编码器并渲染
    let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
        label: Some("egui_render_encoder"),
    });
    
    // 更新缓冲区
    egui_renderer.update_buffers(
        device,
        queue,
        &mut encoder,
        &paint_jobs,
        &screen_descriptor,
    );
    
    // 执行渲染
    execute_render_pass(
        &mut encoder,
        &view,
        egui_renderer,
        &paint_jobs,
        &screen_descriptor,
    )?;
    
    // 提交并呈现
    queue.submit(std::iter::once(encoder.finish()));
    output.present();

    Ok(())
}

/// 更新 Egui 纹理
fn update_textures(
    egui_renderer: &mut egui_wgpu::Renderer,
    device: &wgpu::Device,
    queue: &wgpu::Queue,
    textures: &egui::TexturesDelta,
) -> anyhow::Result<()> {
    for (id, image_delta) in &textures.set {
        egui_renderer.update_texture(device, queue, *id, image_delta);
    }
    Ok(())
}

/// 执行渲染通道
fn execute_render_pass(
    encoder: &mut wgpu::CommandEncoder,
    view: &wgpu::TextureView,
    egui_renderer: &mut egui_wgpu::Renderer,
    paint_jobs: &[egui::ClippedPrimitive],
    screen_descriptor: &ScreenDescriptor,
) -> anyhow::Result<()> {
    let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
        label: Some("egui_render_pass"),
        color_attachments: &[Some(wgpu::RenderPassColorAttachment {
            view,
            resolve_target: None,
            ops: wgpu::Operations {
                load: wgpu::LoadOp::Clear(wgpu::Color::TRANSPARENT),
                store: wgpu::StoreOp::Store,
            },
        })],
        depth_stencil_attachment: None,
        occlusion_query_set: None,
        timestamp_writes: None,
    });
    
    egui_renderer.render(&mut render_pass, paint_jobs, screen_descriptor);
    
    Ok(())
}

