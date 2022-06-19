use crate::usb::xhci::registers::operational::structs::usb_cmd::UsbCmdRegister;
use crate::usb::xhci::registers::volatile::{Volatile, VolatileRegister};


impl Volatile<UsbCmdRegister> {
    pub fn set_enable_interrupt(&mut self) -> Result<(), ()> {
        self.update_volatile(|usb_cmd| {
            usb_cmd.set_interrupt_enable(true);
        });
        if self.read_volatile().interrupt_enable() {
            Ok(())
        } else {
            Err(())
        }
    }
    
    pub fn is_run(&self) -> bool {
        self.read_volatile().run_stop()
    }
}


#[test_case]
pub fn should_usb_cmd_set_enable_interrupt() {
    use crate::usb::xhci::registers::operational::create::create_all_registers::ICreateAllOperationalRegisters;
    let mmio_base = crate::utils::test_fn::extract_virtual_mmio_base_addr();
    let mut register = crate::usb::xhci::registers::create_type::RegisterCreate::UncheckTransmute
        .new_operations(mmio_base, crate::utils::test_fn::extract_cap_len(mmio_base))
        .unwrap();
    assert!(register.usb_cmd.set_enable_interrupt().is_ok());
}
