use core::ptr;

use bitfield_struct::bitfield;


#[bitfield(u128)]
pub struct TrbBase {
    pub parameter: u64,
    pub status: u32,
    pub cycle_bit: bool,
    pub evaluate_next_trb: bool,
    _reserve: u8,
    #[bits(6)]
    pub trb_type: u8,
    pub control: u16,
}


impl TrbBase {
    pub fn from(addr: u64) -> Self {
        unsafe {
            let ptr = addr as *mut Self;
            ptr::read_volatile(ptr)
        }
    }
    
    pub fn test_new() -> Self {
        Self::new()
    }
}
