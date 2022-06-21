use crate::usb::xhci::registers::runtime::structs::interrupter::interrupter_registers::InterrupterRegisterSet;
use crate::usb::xhci::registers::volatile::{Volatile, VolatileRegister};
use crate::utils::error::CommonResult;


impl Volatile<InterrupterRegisterSet> {
    pub fn set_enable_interrupt(&mut self) -> CommonResult<()> {
        self.update(|interrupt| {
            interrupt.iman.set_ie(true);
            interrupt.iman.set_ip(true);
        });
        
        let is_enable_interrupt = self.read().iman.ie() && self.read().iman.ip();
        if is_enable_interrupt {
            Ok(())
        } else {
            Err("InterrupterRegisterSet: Failed Set Enabled Interrupt")
        }
    }
    
    pub fn set_segment_tbl_base_addr(&mut self, addr: u64) {
        let mut interrupt = self.read();
        interrupt.erstba.set_erstba(addr);
        self.write(interrupt);
    }
    
    pub fn set_segment_size(&mut self, size: u16) {
        let mut interrupt = self.read();
        interrupt.erstsz.set_erstsz(size);
        self.write(interrupt);
    }
    
    
    pub fn get_dequeue_ptr(&self) -> u64 {
        self.read().erdp.erdp() << 4
    }
    
    pub fn set_dequeue_ptr(&mut self, addr: u64) {
        let mut interrupter_set = self.read();
        interrupter_set.erdp.set_erdp(addr >> 4);
        self.write(interrupter_set);
    }
}


#[test_case]
pub fn should_interrupt_set_enable_interrupt() {
    use crate::usb::xhci::registers::runtime::create::create_interrupter_register_set::ICreateInterrupterRegisterSet;
    let mut register = crate::usb::xhci::registers::create_type::RegisterCreate::UncheckTransmute.new_interrupter_register_set(crate::utils::test_fn::extract_runtime_base()).primary();
    assert!(register.set_enable_interrupt().is_ok());
}

