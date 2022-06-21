use crate::usb::xhci::registers::operational::structs::command_ring_control::CommandRingControlRegister;
use crate::usb::xhci::registers::volatile::{Volatile, VolatileRegister};


impl Volatile<CommandRingControlRegister> {
    pub fn register_command_ring(&mut self, command_ring_buff_addr: u64) {
        let mut crctl = self.read();
        crctl.set_command_stop(false);
        crctl.set_command_abort(false);
        self.write(crctl);
        self.set_command_ring_pointer(command_ring_buff_addr);
    }
    
    pub fn set_command_ring_pointer(&mut self, command_ring_buff_addr: u64) {
        let mut crctl = self.read();
        crctl.set_command_ring_pointer(command_ring_buff_addr >> 6);
        self.write(crctl);
    }
    
    pub fn get_command_ring_pointer(&self) -> u64 {
        self.read().command_ring_pointer() << 6
    }
}


#[test_case]
pub fn should_set_command_ring_control_ptr() {
    use crate::usb::xhci::registers::operational::create::create_all_registers::ICreateAllOperationalRegisters;
    
    let mmio_base = crate::utils::test_fn::extract_virtual_mmio_base_addr();
    let mut register = crate::usb::xhci::registers::create_type::RegisterCreate::UncheckTransmute
        .new_operations(mmio_base, crate::utils::test_fn::extract_cap_len(mmio_base))
        .unwrap();
    
    let addr = mmio_base + 0xFFFFF;
    register.crctl.set_command_ring_pointer(addr);
    
    assert_ne!(0, register.crctl.get_command_ring_pointer());
}
