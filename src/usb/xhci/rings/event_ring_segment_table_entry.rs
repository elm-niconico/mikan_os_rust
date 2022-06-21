use bitfield_struct::bitfield;

use crate::impl_debug_bit_fields;


#[bitfield(u128)]
pub struct EventRingSegmentTableEntry {
    pub ring_segment_base_address: u64,
    
    pub ring_segment_size: u16,
    
    _reserve: u16,
    
    _reserve2: u32,
}


impl_debug_bit_fields! {
    EventRingSegmentTableEntry{
        ring_segment_base_address,
        ring_segment_size
    }
}
