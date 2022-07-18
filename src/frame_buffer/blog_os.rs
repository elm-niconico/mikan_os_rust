use core::fmt;

use bootloader::boot_info::FrameBuffer;
use font8x8::UnicodeFonts;
use volatile::Volatile;

use crate::frame_buffer::pixel::pixel_color::PixelColor;
use crate::frame_buffer::pixel::pixel_color::PixelColor::Rgb;
use crate::frame_buffer::PixelWriter;

pub struct BlogOsWriter {
    buffer: Volatile<&'static mut [u8]>,
    info: bootloader::boot_info::FrameBufferInfo,
    x_pos: usize,
    y_pos: usize,
}


impl BlogOsWriter {
    pub fn new(frame_buffer: &'static mut FrameBuffer) -> Self {
        Self {
            info: frame_buffer.info(),
            buffer: Volatile::new(frame_buffer.buffer_mut()),
            x_pos: 0,
            y_pos: 0,
        }
    }

    fn newline(&mut self) {
        self.y_pos += 8;
        self.carriage_return();
    }

    fn carriage_return(&mut self) {
        self.x_pos = 0;
    }

    /// Erases all text on the screen
    pub fn clear(&mut self) {
        self.x_pos = 0;
        self.y_pos = 0;
        self.buffer.fill(0);
    }

    fn shift_lines_up(&mut self) {
        let offset = self.info.stride * self.info.bytes_per_pixel * 8;
        self.buffer.copy_within(offset.., 0);

        self.y_pos -= 8;
    }

    pub fn width(&self) -> usize {
        self.info.horizontal_resolution
    }

    pub fn height(&self) -> usize {
        self.info.vertical_resolution
    }

    fn write_char(&mut self, c: char) {
        match c {
            '\n' => self.newline(),
            '\r' => self.carriage_return(),
            c => {
                if self.x_pos >= self.width() {
                    self.newline();
                }
                while self.y_pos >= (self.height() - 8) {
                    self.shift_lines_up();
                }
                let rendered = font8x8::BASIC_FONTS
                    .get(c)
                    .expect("character not found in basic font");
                self.write_rendered_char(rendered);
            }
        }
    }

    fn write_rendered_char(&mut self, rendered_char: [u8; 8]) {
        for (y, byte) in rendered_char.iter().enumerate() {
            for (x, bit) in (0..8).enumerate() {
                let is_write = *byte & (1 << bit) != 0;
                if is_write {
                    self.write_pixel(self.x_pos + x, self.y_pos + y, Rgb([0x33, 0xff, 0x66, 0]))
                } else {
                    self.clear_pixel(self.x_pos + x, self.y_pos + y)
                };
            }
        }
        self.x_pos += 8;
    }

    fn do_write_pixel(&mut self, x: usize, y: usize, color: PixelColor) {
        let pixel_offset = y * self.info.stride + x;
        let color = color.switch_color(self.info.pixel_format).as_buff();
        let bytes_per_pixel = self.info.bytes_per_pixel;
        let byte_offset = pixel_offset * bytes_per_pixel;
        self.buffer
            .index_mut(byte_offset..(byte_offset + bytes_per_pixel))
            .copy_from_slice(&color[..bytes_per_pixel]);
    }

    /// Writes the given ASCII string to the buffer.
    ///
    /// Wraps lines at `BUFFER_WIDTH`. Supports the `\n` newline character. Does **not**
    /// support strings with non-ASCII characters, since they can't be printed in the VGA text
    /// mode.
    fn write_string(&mut self, s: &str) {
        for char in s.chars() {
            self.write_char(char);
        }
    }
}

impl PixelWriter for BlogOsWriter {
    fn write_pixel(&mut self, x: usize, y: usize, color: PixelColor) {
        self.do_write_pixel(x, y, color);
    }

    fn clear_pixel(&mut self, x: usize, y: usize) {
        self.do_write_pixel(x, y, Rgb([0, 0, 0, 0]));
    }
}


impl fmt::Write for BlogOsWriter {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.write_string(s);
        Ok(())
    }
}