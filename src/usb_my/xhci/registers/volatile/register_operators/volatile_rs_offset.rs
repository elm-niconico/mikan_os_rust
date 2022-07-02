use crate::usb_my::xhci::registers::capability::structs::rts_off::RuntimeRegisterSpaceOffset;
use crate::usb_my::xhci::registers::volatile::{Volatile, VolatileRegister};

impl Volatile<RuntimeRegisterSpaceOffset> {
    pub fn read_rts_offset(&self) -> u32 {
        self.read().rts_offset() << 5
    }
}