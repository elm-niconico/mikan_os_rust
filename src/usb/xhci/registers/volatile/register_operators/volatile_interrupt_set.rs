use crate::usb::xhci::registers::volatile::{Volatile, VolatileRegister};
use crate::usb::xhci::registers::runtime::structs::interrupter::interrupter_registers::InterrupterRegisterSet;


impl Volatile<InterrupterRegisterSet> {
    pub fn set_enable_interrupt(&mut self) -> Result<(), ()> {
        self.update_volatile(|interrupt| {
            interrupt.iman.set_ie(true);
            interrupt.iman.set_ip(true);
        });
        
        
        let is_enable_interrupt = self.read_volatile().iman.ie() && self.read_volatile().iman.ip();
        
        if is_enable_interrupt {
            Ok(())
        } else {
            Err(())
        }
    }
}


#[test_case]
pub fn should_interrupt_set_enable_interrupt() {
    use crate::usb::xhci::registers::runtime::create::create_interrupter_register_set::ICreateInterrupterRegisterSet;
    let mut register = crate::usb::xhci::registers::create_type::RegisterCreate::UncheckTransmute.new_interrupter_register_set(crate::utils::test_fn::extract_runtime_base())[0];
    assert!(register.set_enable_interrupt().is_ok());
}

