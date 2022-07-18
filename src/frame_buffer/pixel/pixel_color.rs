use bootloader::boot_info::PixelFormat;

use crate::frame_buffer::pixel::pixel_color::PixelColor::{Bgr, Rgb};

#[derive(Debug, Copy, Clone, PartialOrd, PartialEq, Eq, Ord)]
pub enum PixelColor {
    Rgb([u8; 4]),
    Bgr([u8; 4]),
}

impl PixelColor {
    pub fn switch_color(self, pixel_format: PixelFormat) -> Self {
        match self {
            Rgb(buff) => {
                match pixel_format {
                    PixelFormat::RGB => self,
                    _ => Bgr([buff[2], buff[1], buff[0], buff[3]])
                }
            }
            Bgr(buff) => {
                match pixel_format {
                    PixelFormat::BGR => self,
                    _ => Rgb([buff[2], buff[1], buff[0], buff[3]])
                }
            }
        }
    }

    pub fn as_buff(&self) -> [u8; 4] {
        match self {
            Rgb(buff) => buff.clone(),
            Bgr(buff) => buff.clone()
        }
    }
}