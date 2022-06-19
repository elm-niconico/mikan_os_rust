use bitfield_struct::bitfield;

use crate::impl_debug_bit_fields;


#[bitfield(u64)]
pub struct EventRingDequePointerRegister {
    #[bits(3)]
    /** Dequeue ERST Segment Index */
    pub desi: u8,
    
    /** Event Handler Busy */
    pub ehb: bool,
    
    /** Event Ring Dequeue Pointer */
    #[bits(60)]
    pub erdp: u64,
}

impl_debug_bit_fields! {
    EventRingDequePointerRegister{
        desi,
        ehb,
        erdp
    }
}
