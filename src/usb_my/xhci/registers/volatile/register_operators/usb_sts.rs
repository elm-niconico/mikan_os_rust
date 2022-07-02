use crate::usb_my::xhci::registers::operational::structs::usb_sts::UsbStsRegister;
use crate::usb_my::xhci::registers::volatile::{Volatile, VolatileRegister};

impl Volatile<UsbStsRegister> {
    pub fn is_halted(&self) -> bool {
        self.read().hc_halted()
    }
}