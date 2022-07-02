use crate::usb_my::xhci::registers::runtime::structs::interrupter::erdp::EventRingDequePointerRegister;
use crate::usb_my::xhci::registers::runtime::structs::interrupter::erstba::EventRingSegmentTableBaseAddressRegister;
use crate::usb_my::xhci::registers::runtime::structs::interrupter::erstsz::EventRingSegmentTableSizeRegister;
use crate::usb_my::xhci::registers::runtime::structs::interrupter::iman::InterrupterManagementRegister;
use crate::usb_my::xhci::registers::runtime::structs::interrupter::imod::InterrupterModerationRegister;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct InterrupterRegisterSet {
    pub iman: InterrupterManagementRegister,
    pub imod: InterrupterModerationRegister,
    pub erstsz: EventRingSegmentTableSizeRegister,
    _reserve: u32,
    pub erstba: EventRingSegmentTableBaseAddressRegister,
    pub erdp: EventRingDequePointerRegister,
}

// impl_debug_packed_fields!{
//     InterrupterRegisterSet{
//         iman,
//         imod,
//         erstsz,
//         erstba,
//         erdp
//     }
// }