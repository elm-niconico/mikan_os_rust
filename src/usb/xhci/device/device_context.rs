use core::simd::u32x8;

use crate::impl_debug_packed_fields;
use crate::usb::xhci::device::slot_context::SlotContext;


#[repr(packed)]
pub struct DeviceContext {
    pub slot_context: SlotContext,
    pub end_points: [u32x8; 8],
}
impl_debug_packed_fields! {
    DeviceContext{
        slot_context,
        end_points
    }
}
