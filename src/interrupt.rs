use pic8259::ChainedPics;
use spin::Mutex;


pub const PRIMARY_OFFSET: u8 = 32;
pub const SECONDARY_OFFSET: u8 = 40;


pub static PIC_CONTROLLER: Mutex<ChainedPics> =
    Mutex::new(unsafe { ChainedPics::new(PRIMARY_OFFSET, SECONDARY_OFFSET) });
