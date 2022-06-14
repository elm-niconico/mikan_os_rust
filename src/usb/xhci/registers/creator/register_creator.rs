use core::ptr;

use crate::usb::xhci::registers::r#struct::capability::capability_register::CapabilityRegister;
use crate::usb::xhci::registers::r#struct::register_info::RegisterInfo;
use crate::usb::xhci::registers::read_write::volatile::Volatile;


pub trait ICapabilityRegisterCreate {
    fn create(&self, mmio_base_address: u64) -> Result<Volatile<CapabilityRegister>, ()>;
}


#[allow(dead_code)]
pub enum CapabilityRegisterCreate {
    // 生ポインタからレジスタの構造体に強制的に解釈
    // 安全ではない
    Transmute,
}


impl ICapabilityRegisterCreate for CapabilityRegisterCreate {
    fn create(&self, mmio_base_address: u64) -> Result<Volatile<CapabilityRegister>, ()> {
        match self {
            CapabilityRegisterCreate::Transmute => {
                let raw_ptr = mmio_base_address as *const CapabilityRegister;
                let register = unsafe { ptr::read_volatile(raw_ptr) };
                Ok(Volatile::Core(RegisterInfo::new(mmio_base_address, register)))
            }
        }
    }
}
