use core::fmt::Debug;

use crate::usb::xhci::registers::create_type::CreateType;
use crate::usb::xhci::registers::operational::create::create_all_registers::ICreateAllOperationalRegisters;
use crate::usb::xhci::registers::operational::structs::command_ring_control::CommandRingControlRegister;
use crate::usb::xhci::registers::operational::structs::usb_cmd::UsbCmdRegister;
use crate::usb::xhci::registers::register_info::RegisterInfo;
use crate::usb::xhci::registers::runtime::create::create_interrupter_register_set::ICreateInterrupterRegisterSet;
use crate::usb::xhci::registers::runtime::structs::interrupter::interrupter_registers::InterrupterRegisterSet;
use crate::utils::test_fn::{extract_cap_len, extract_virtual_mmio_base_addr};


pub trait IVolatile<T> {
    fn read_volatile(&self) -> T;
    fn write_volatile(&mut self, src: T);
    fn update_volatile(&mut self, update_fn: fn(r: &mut T));
}


#[derive(Debug, Clone, Copy)]
pub enum Volatile<T: Debug> {
    Core(RegisterInfo<T>),
}


impl<T: Debug> IVolatile<T> for Volatile<T> {
    fn read_volatile(&self) -> T {
        match self {
            Volatile::Core(r) => unsafe { core::ptr::read_volatile(r.get_register_raw_ptr()) },
        }
    }
    
    fn write_volatile(&mut self, src: T) {
        match self {
            Volatile::Core(r) => {
                unsafe {
                    core::ptr::write_volatile(r.get_register_raw_ptr(), src);
                };
            }
        }
    }
    
    fn update_volatile(&mut self, update_fn: fn(&mut T)) {
        let mut r = self.read_volatile();
        update_fn(&mut r);
        self.write_volatile(r);
    }
}


impl Volatile<InterrupterRegisterSet> {
    pub fn set_enable_interrupt(&mut self) -> Result<(), ()> {
        self.update_volatile(|interrupt| {
            interrupt.iman.set_ie(true);
            interrupt.iman.set_ip(true);
        });
        if self.read_volatile().iman.ie() && self.read_volatile().iman.ip() {
            Ok(())
        } else {
            Err(())
        }
    }
}


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
}


impl Volatile<CommandRingControlRegister> {
    pub fn set_command_ring_pointer(&mut self, addr: u64) {
        let next_addr = addr >> 6;
        let mut crcr = self.read_volatile();
        crcr.set_command_ring_pointer(next_addr);
        self.write_volatile(crcr);
    }
    
    pub fn get_command_ring_pointer(&self) -> u64 {
        self.read_volatile().command_ring_pointer() << 6
    }
    
    pub fn register_command_ring(&mut self, command_ring_buff_addr: u64) {
        let mut crcr = self.read_volatile();
        crcr.set_ring_cycle_state(true);
        crcr.set_command_stop(false);
        crcr.set_command_abort(false);
        self.write_volatile(crcr);
        
        self.set_command_ring_pointer(command_ring_buff_addr);
    }
}


#[test_case]
pub fn should_interrupt_set_enable_interrupt() {
    let mut register = CreateType::UncheckTransmute.new_interrupter_register_set(crate::utils::test_fn::extract_runtime_base())[0];
    assert!(register.set_enable_interrupt().is_ok());
}


#[test_case]
pub fn should_usb_cmd_set_enable_interrupt() {
    let mmio_base = extract_virtual_mmio_base_addr();
    let mut register = CreateType::UncheckTransmute.new_all_operations(mmio_base, extract_cap_len(mmio_base)).unwrap();
    assert!(register.usb_cmd.set_enable_interrupt().is_ok());
}


#[test_case]
pub fn should_set_command_ring_control_ptr() {
    let mmio_base = extract_virtual_mmio_base_addr();
    let mut register = CreateType::UncheckTransmute
        .new_all_operations(mmio_base, extract_cap_len(mmio_base))
        .unwrap();
    let addr = mmio_base + 0xFFFFF;
    register.crctl.set_command_ring_pointer(addr);
    
    assert_ne!(0, register.crctl.get_command_ring_pointer());
}
