use core::fmt::Debug;

use crate::impl_debug_only_fields;
use crate::usb::xhci::registers::operational::structs::command_ring_control::CommandRingControlRegister;
use crate::usb::xhci::registers::operational::structs::configure::ConfigureRegister;
use crate::usb::xhci::registers::operational::structs::dcbaap::Dcbaap;
use crate::usb::xhci::registers::operational::structs::device_notification_control::DeviceNotificationControlRegister;
use crate::usb::xhci::registers::operational::structs::page_size::PageSizeRegister;
use crate::usb::xhci::registers::operational::structs::usb_cmd::UsbCmdRegister;
use crate::usb::xhci::registers::operational::structs::usb_sts::UsbStsRegister;
use crate::usb::xhci::registers::volatile::{Volatile, VolatileRegister};
use crate::usb::xhci::registers::register_info::RegisterInfo;


pub struct OperationalRegisters {
    pub usb_cmd: Volatile<UsbCmdRegister>,
    pub usb_sts: Volatile<UsbStsRegister>,
    pub page_size: Volatile<PageSizeRegister>,
    pub device_notify: Volatile<DeviceNotificationControlRegister>,
    
    //** Command Ring Control Register */
    pub crctl: Volatile<CommandRingControlRegister>,
    
    //** Device Context Base Address Array Pointer Register */
    pub dcbaap: Volatile<Dcbaap>,
    pub configure: Volatile<ConfigureRegister>,
}


impl OperationalRegisters {
    pub fn new(
        usb_cmd: Volatile<UsbCmdRegister>,
        usb_sts: Volatile<UsbStsRegister>,
        page_size: Volatile<PageSizeRegister>,
        device_notify: Volatile<DeviceNotificationControlRegister>,
        crctl: Volatile<CommandRingControlRegister>,
        dcbaap: Volatile<Dcbaap>,
        configure: Volatile<ConfigureRegister>,
    ) -> Self {
        Self {
            usb_cmd,
            usb_sts,
            page_size,
            device_notify,
            crctl,
            dcbaap,
            configure,
        }
    }
}


fn new_volatile<T: Debug>(register: RegisterInfo<T>) -> Volatile<T> {
    Volatile::Core(register)
}

impl_debug_only_fields! {
    OperationalRegisters{
        usb_cmd,
        usb_sts,
        page_size,
        device_notify,
        crctl,
        dcbaap,
        configure
    }
}
pub trait XhcResetOperations {
    fn wait_xhc_halted(&mut self) -> Result<(), ()>;
    fn reset_controller(&mut self) -> Result<(), ()>;
}


impl XhcResetOperations for OperationalRegisters {
    fn wait_xhc_halted(&mut self) -> Result<(), ()> {
        self.usb_cmd.update_volatile(|cmd| {
            cmd.set_interrupt_enable(false);
            cmd.set_host_system_error_enable(false);
            cmd.set_enable_wrap_event(false);
        });
        
        
        let is_not_halted = |o: &OperationalRegisters| { !o.usb_sts.is_halted() };
        
        if is_not_halted(self) {
            self.usb_cmd.update_volatile(|cmd| {
                cmd.set_run_stop(false);
            });
        }
        
        while is_not_halted(self) {}
        
        let is_stop_controller = self.usb_sts.is_halted() && !self.usb_cmd.is_run();
        if is_stop_controller {
            Ok(())
        } else {
            Err(())
        }
    }
    
    fn reset_controller(&mut self) -> Result<(), ()> {
        let read_cmd = |o: &OperationalRegisters| o.usb_cmd.read_volatile();
        let read_sts = |o: &OperationalRegisters| o.usb_sts.read_volatile();
        
        
        self.usb_cmd.update_volatile(|cmd| {
            cmd.set_host_controller_reset(true);
        });
        
        while read_cmd(self).host_controller_reset() {}
        
        while read_sts(self).controller_not_ready() {}
        
        let is_success_reset = (!read_cmd(self).host_controller_reset()) && (!read_sts(self).controller_not_ready());
        if is_success_reset {
            Ok(())
        } else {
            Err(())
        }
    }
}


#[test_case]
pub fn should_reset_xhc() {
    use crate::usb::xhci::registers::operational::create::create_all_registers::ICreateAllOperationalRegisters;
    use crate::utils::test_fn::{extract_virtual_mmio_base_addr, extract_cap_len};
    
    let mut op = crate::usb::xhci::registers::create_type::RegisterCreate::UncheckTransmute
        .new_operations(crate::utils::test_fn::extract_virtual_mmio_base_addr(), extract_cap_len(extract_virtual_mmio_base_addr()))
        .unwrap();
    
    assert!(op.wait_xhc_halted().is_ok())
}


#[test_case]
pub fn should_xhc_reset() {
    use crate::usb::xhci::registers::operational::create::create_all_registers::ICreateAllOperationalRegisters;
    use crate::utils::test_fn::{extract_virtual_mmio_base_addr, extract_cap_len};
    
    let mut op = crate::usb::xhci::registers::create_type::RegisterCreate::UncheckTransmute
        .new_operations(crate::utils::test_fn::extract_virtual_mmio_base_addr(), extract_cap_len(extract_virtual_mmio_base_addr()))
        .unwrap();
    
    assert!(op.reset_controller().is_ok())
}

