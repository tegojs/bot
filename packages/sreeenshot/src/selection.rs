use glam::Vec2;

#[derive(Clone, Copy, Debug, Default)]
pub struct Selection {
    start: Option<Vec2>,
    end: Option<Vec2>,
}

impl Selection {
    pub fn new() -> Self {
        Self {
            start: None,
            end: None,
        }
    }

    pub fn start(&mut self, pos: Vec2) {
        self.start = Some(pos);
        self.end = Some(pos);
    }

    pub fn update(&mut self, pos: Vec2) {
        if self.start.is_some() {
            self.end = Some(pos);
        }
    }

    pub fn finish(&mut self) -> Option<((u32, u32), (u32, u32))> {
        self.coords()
        // Don't clear start and end - keep selection visible after completion
    }

    pub fn cancel(&mut self) {
        self.start = None;
        self.end = None;
    }

    pub fn is_active(&self) -> bool {
        self.start.is_some()
    }

    pub fn coords(&self) -> Option<((u32, u32), (u32, u32))> {
        self.coords_with_scale(1.0)
    }
    
    /// Get coordinates in physical pixels, converting from logical points
    pub fn coords_with_scale(&self, scale_factor: f64) -> Option<((u32, u32), (u32, u32))> {
        let start = self.start?;
        let end = self.end?;

        // Convert from logical points to physical pixels
        let min_x = (start.x.min(end.x).max(0.0) * scale_factor as f32).floor() as u32;
        let max_x = (start.x.max(end.x) * scale_factor as f32).floor() as u32;
        let min_y = (start.y.min(end.y).max(0.0) * scale_factor as f32).floor() as u32;
        let max_y = (start.y.max(end.y) * scale_factor as f32).floor() as u32;

        Some(((min_x, min_y), (max_x, max_y)))
    }

    pub fn rect(&self) -> Option<(f32, f32, f32, f32)> {
        let start = self.start?;
        let end = self.end?;

        let x = start.x.min(end.x);
        let y = start.y.min(end.y);
        let width = (end.x - start.x).abs();
        let height = (end.y - start.y).abs();

        Some((x, y, width, height))
    }
}

