use crate::usb::xhci::registers::runtime::structs::interrupter::erdp::EventRingDequePointerRegister;
use crate::usb::xhci::registers::runtime::structs::interrupter::erstba::EventRingSegmentTableBaseAddressRegister;
use crate::usb::xhci::registers::runtime::structs::interrupter::erstsz::EventRingSegmentTableSizeRegister;
use crate::usb::xhci::registers::runtime::structs::interrupter::iman::InterrupterManagementRegister;
use crate::usb::xhci::registers::runtime::structs::interrupter::imod::InterrupterModerationRegister;


#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct InterrupterRegisters {
    pub iman: InterrupterManagementRegister,
    pub imod: InterrupterModerationRegister,
    pub erstsz: EventRingSegmentTableSizeRegister,
    pub erstba: EventRingSegmentTableBaseAddressRegister,
    pub erdp: EventRingDequePointerRegister,
}
