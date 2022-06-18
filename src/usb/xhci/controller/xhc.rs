use crate::usb::xhci::registers::capability::create::all_registers::ICreateAllCapabilityRegisters;
use crate::usb::xhci::registers::capability::structs::capability_register::CapabilityRegisters;
use crate::usb::xhci::registers::create_type::CreateType;
use crate::usb::xhci::registers::operators::create::operationals::ICreateOperationalRegisters;
use crate::usb::xhci::registers::operators::structs::operational_registers::OperationalRegisters;
use crate::usb::xhci::registers::read_write::volatile::IVolatile;


#[allow(dead_code)]
#[derive(Debug)]
pub struct XhcController {
    capability_register: CapabilityRegisters,
    operational_registers: OperationalRegisters,
}


impl XhcController {
    pub fn new(mmio_base: u64) -> Result<Self, ()> {
        let create = CreateType::UncheckTransmute;
        let capability_register = create.new_all_capabilities(mmio_base)?;
        let operational_registers = create.new_operational(
            mmio_base,
            capability_register.cap_length.read_volatile())?;
        
        Ok(Self {
            capability_register,
            operational_registers,
        })
    }
    
    
    pub fn wait_usb_halted(&mut self) -> Result<(), ()> {
        self.operational_registers.usb_cmd.update_volatile(|cmd| {
            cmd.set_interrupt_enable(false);
            cmd.set_host_system_error_enable(false);
            cmd.set_enable_wrap_event(false);
        });
        
        if self.operational_registers.usb_sts.read_volatile().hc_halted() {
            self.operational_registers.usb_cmd.update_volatile(|cmd| {
                cmd.set_run_stop(false);
            });
        }
        
        while self.operational_registers.usb_cmd.read_volatile().run_stop() {}
        
        if self.operational_registers.usb_sts.read_volatile().hc_halted() {
            Ok(())
        } else {
            Err(())
        }
    }
}


// #[test_case]
// pub fn should_new_xhc() {
//     let xhc = XhcController::new(tmp_find_usb_mouse_base().unwrap());
//
//     assert!(xhc.is_ok());
// }


// #[test_case]
// pub fn should_wait_hc_halted() {
//     let xhc = XhcController::new(tmp_find_usb_mouse_base().unwrap());
//
//     assert!(xhc.is_ok());
//     let mut xhc = xhc.unwrap();
//     let is_halted = xhc.wait_usb_halted();
//     assert!(is_halted.is_ok())
// }
//
//


