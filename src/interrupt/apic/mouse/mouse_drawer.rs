use spin::mutex::SpinMutex;

use crate::error::kernel_error::{KernelError, KernelResult};
use crate::frame_buffer::{frame_buff_size, WRITER};
use crate::spin::sync_once_cell::StaticOnceCell;

pub static MOUSE_CURSOR: StaticOnceCell<SpinMutex<MouseCursor>> = StaticOnceCell::uninit();

pub struct MouseCursor {
    pos_x: usize,
    pos_y: usize,
}

impl MouseCursor {
    pub const fn new() -> Self {
        Self {
            pos_x: 300,
            pos_y: 200,
        }
    }

    pub fn init(&mut self) -> KernelResult<()> {
        self.draw()
    }

    pub fn move_mouse(&mut self, x: isize, y: isize) -> KernelResult<()> {
        let _ = self.erase();

        let pos_x_isize = isize::try_from(self.pos_x).map_err(|e| KernelError::FrameBuffOverFlow)?;
        let pos_y_isize = isize::try_from(self.pos_y).map_err(|e| KernelError::FrameBuffOverFlow)?;

        self.pos_x = usize::try_from(pos_x_isize + x).map_err(|e| KernelError::FrameBuffOverFlow)?;
        self.pos_y = usize::try_from(pos_y_isize + y).map_err(|e| KernelError::FrameBuffOverFlow)?;
        self.draw()
    }

    fn erase(&mut self) -> KernelResult<()> {
        if is_over_window(self.pos_x, self.pos_y) {
            return Err(KernelError::FrameBuffOverFlow);
        }
        for dy in 0..MOUSE_CURSOR_HEIGHT {
            for dx in 0..MOUSE_CURSOR_WIDTH {
                let next_x = dx + self.pos_x as usize;
                let next_y = dy + self.pos_y as usize;
                WRITER
                    .get()
                    .lock()
                    .write_pixel(next_x, next_y, false)
            }
        }

        Ok(())
    }

    fn draw(&mut self) -> KernelResult<()> {
        draw_mouse_cursor(self.pos_x, self.pos_y)
    }
}

const MOUSE_CURSOR_WIDTH: usize = 15;
const MOUSE_CURSOR_HEIGHT: usize = 24;

const MOUSE_CURSOR_SHAPE: [[u8; MOUSE_CURSOR_WIDTH]; MOUSE_CURSOR_HEIGHT] = [
    *b"@              ",
    *b"@@             ",
    *b"@.@            ",
    *b"@..@           ",
    *b"@...@          ",
    *b"@....@         ",
    *b"@.....@        ",
    *b"@......@       ",
    *b"@.......@      ",
    *b"@........@     ",
    *b"@.........@    ",
    *b"@..........@   ",
    *b"@...........@  ",
    *b"@............@ ",
    *b"@......@@@@@@@@",
    *b"@......@       ",
    *b"@....@@.@      ",
    *b"@...@ @.@      ",
    *b"@..@   @.@     ",
    *b"@.@    @.@     ",
    *b"@@      @.@    ",
    *b"@       @.@    ",
    *b"         @.@   ",
    *b"         @@@   ",
];

pub fn draw_mouse_cursor(x: usize, y: usize) -> KernelResult<()> {
    if is_over_window(x, y) {
        return Err(KernelError::FrameBuffOverFlow);
    }
    for dy in 0..MOUSE_CURSOR_HEIGHT {
        for dx in 0..MOUSE_CURSOR_WIDTH {
            let next_x = dx + x as usize;
            let next_y = dy + y as usize;
            let pixel = char::from(MOUSE_CURSOR_SHAPE[dy][dx]);


            if pixel == '@' {
                WRITER
                    .get()
                    .lock()
                    .write_pixel(next_x, next_y, false);
            } else if pixel == '.' {
                WRITER
                    .get()
                    .lock()
                    .write_pixel(next_x, next_y, true);
            }
        }
    }


    Ok(())
}

fn is_over_window(x: usize, y: usize) -> bool {
    let (width, height) = frame_buff_size();

    let next_width_range = x + MOUSE_CURSOR_WIDTH;
    let next_height_range = y + MOUSE_CURSOR_HEIGHT;

    let is_over_flow_width = width < next_width_range;

    let is_over_flow_height = height < next_height_range;


    is_over_flow_height || is_over_flow_width
}