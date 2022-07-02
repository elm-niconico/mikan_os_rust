use crate::usb_my::xhci::registers::operational::structs::usb_cmd::UsbCmdRegister;
use crate::usb_my::xhci::registers::volatile::{Volatile, VolatileRegister};
use crate::utils::error::CommonResult;

impl Volatile<UsbCmdRegister> {
    pub fn set_enable_interrupt(&mut self) -> CommonResult<()> {
        self.update(|usb_cmd| {
            usb_cmd.set_interrupt_enable(true);
        });
        if self.read().interrupt_enable() {
            Ok(())
        } else {
            Err("Failed Set Enable Interrupt")
        }
    }

    pub fn is_run(&self) -> bool {
        self.read().run_stop()
    }
}

#[test_case]
pub fn should_usb_cmd_set_enable_interrupt() {
    use crate::usb_my::xhci::registers::operational::create::create_all_registers::ICreateAllOperationalRegisters;
    let mmio_base = crate::utils::test_fn::extract_virtual_mmio_base_addr();
    let mut register =
        crate::usb_my::xhci::registers::create_type::RegisterCreate::UncheckTransmute
            .new_operations(mmio_base, crate::utils::test_fn::extract_cap_len(mmio_base))
            .unwrap();
    assert!(register.usb_cmd.set_enable_interrupt().is_ok());
}