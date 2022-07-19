use core::fmt::Debug;

use crate::PixelColor;

pub trait PixelWriter {
    fn write_pixel(&mut self, x: usize, y: usize, color: PixelColor);
    fn clear_pixel(&mut self, x: usize, y: usize);
    fn fill_rect(&mut self, rect: Rect, color: PixelColor) {
        for h in 0..rect.height {
            for w in 0..rect.width {
                self.write_pixel(w + rect.x, h + rect.y, color);
            }
        }
    }

    fn draw_rect(&mut self,
                 rect: Rect,
                 color: PixelColor,
    ) {
        for w in 0..rect.width {
            // 上辺
            &mut self.write_pixel(rect.x + w, rect.y, color);
            // 底辺
            &mut self.write_pixel(rect.x + w, rect.y + rect.height - 1, color);
        }

        for h in 0..rect.height {
            // 左辺
            &mut self.write_pixel(rect.x, rect.y + h, color);
            // 右辺
            &mut self.write_pixel(rect.x + rect.width - 1, rect.y + h, color);
        }
    }
}

#[derive(Debug, Copy, Clone, PartialOrd, PartialEq, Eq)]
pub struct Rect {
    pub x: usize,
    pub y: usize,
    pub width: usize,
    pub height: usize,
}

impl Rect {
    pub fn new(x: usize, y: usize, width: usize, height: usize) -> Self {
        Self {
            x,
            y,
            width,
            height,
        }
    }
}