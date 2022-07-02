use core::fmt::{Debug, Formatter};

use crate::usb_my::xhci::registers::operational::structs::port_register_set::PortRegisterSet;
use crate::usb_my::xhci::registers::register_info::RegisterInfo;
use crate::usb_my::xhci::registers::volatile::{Volatile, VolatileRegister};
use crate::utils::error::CommonResult;

pub struct Port {
    port_register_set: Volatile<PortRegisterSet>,
}

impl Debug for Port {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("Port")
            .field("port_register_set", &self.port_register_set.read())
            .finish()
    }
}

impl Port {
    pub fn new(port_register_set: &PortRegisterSet) -> Self {
        let addr = (port_register_set as *const PortRegisterSet).addr() as u64;
        Self {
            port_register_set: Volatile::Core(RegisterInfo::new(addr)),
        }
    }

    pub fn is_current_connect(&self) -> bool {
        self.port_register_set.read().current_connect_status()
    }

    pub fn reset(&mut self) -> CommonResult<()> {
        let mut port_register_set = self.port_register_set.read();
        port_register_set.set_connect_status_change(true);
        port_register_set.set_port_reset(true);
        self.port_register_set.write(port_register_set);

        while self.port_register_set.read().port_reset() {}

        if self.port_register_set.read().connect_status_change() {
            Ok(())
        } else {
            Err("Failed Reset Port")
        }
    }
}

// #[test_case]
// #[ignore]
// pub fn should_reset_port() {
//     let mut xhc = XhcController::initialize(extract_virtual_mmio_base_addr(), 1, 8).unwrap();
//     xhc.run().unwrap();
//
//     for n in 0..xhc.max_pots() {}
// }