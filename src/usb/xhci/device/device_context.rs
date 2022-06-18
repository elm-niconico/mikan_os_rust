use core::simd::u32x8;
use crate::usb::xhci::device::slot_context::SlotContext;


#[repr(packed)]
pub struct DeviceContext{
    pub slot_context: SlotContext,
    pub end_points: [u32x8; 8]
}
