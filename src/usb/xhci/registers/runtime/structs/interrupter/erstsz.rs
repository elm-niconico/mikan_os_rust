use bitfield_struct::bitfield;

use crate::impl_debug_bit_fields;


#[bitfield(u32)]
pub struct EventRingSegmentTableSizeRegister {
    /** Event Ring Segment Table Size */
    pub erstsz: u16,
    _reserve: u16,
}

impl_debug_bit_fields! {
    EventRingSegmentTableSizeRegister{
        erstsz
    }
}
