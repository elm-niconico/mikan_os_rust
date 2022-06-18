use core::fmt::Debug;
use core::mem;

use crate::usb::xhci::registers::capability::capability_register::CapLength;
use crate::usb::xhci::registers::create_type::CreateType;
use crate::usb::xhci::registers::operators::structs::command_ring_control::CommandRingControlRegister;
use crate::usb::xhci::registers::operators::structs::configure::ConfigureRegister;
use crate::usb::xhci::registers::operators::structs::device_context_base_address_array_pointer::DeviceContextBaseAddressArrayPointerRegister;
use crate::usb::xhci::registers::operators::structs::device_notification_control::DeviceNotificationControlRegister;
use crate::usb::xhci::registers::operators::structs::operational_registers::OperationalRegisters;
use crate::usb::xhci::registers::operators::structs::page_size::PageSizeRegister;
use crate::usb::xhci::registers::operators::structs::usb_cmd::UsbCmdRegister;
use crate::usb::xhci::registers::operators::structs::usb_sts::UsbStsRegister;
use crate::usb::xhci::registers::register_info::RegisterInfo;


pub trait ICreateOperationalRegisters {
    fn new_operational(&self, mmio_base_addr: u64, cap_length: CapLength) -> Result<OperationalRegisters, ()>;
}


impl ICreateOperationalRegisters for CreateType {
    fn new_operational(&self, mmio_base_addr: u64, cap_len: CapLength) -> Result<OperationalRegisters, ()> {
        let cap_len: u8 = cap_len.into();
        match self {
            CreateType::UncheckTransmute => { Ok(uncheck_transmute(mmio_base_addr, cap_len)) }
            _ => { todo!("Not Impl!") }
        }
    }
}


fn uncheck_transmute(mmio_base_addr: u64, cap_len: u8) -> OperationalRegisters {
    let mut addr = mmio_base_addr + cap_len as u64;
    let usb_cmd = transmute_register::<UsbCmdRegister>(&mut addr);
    let usb_sts = transmute_register::<UsbStsRegister>(&mut addr);
    let page_size = transmute_register::<PageSizeRegister>(&mut addr);
    let device_notify = transmute_register::<DeviceNotificationControlRegister>(&mut addr);
    let command_ring_control = transmute_register::<CommandRingControlRegister>(&mut addr);
    let device_context_bae_addr_array_ptr =
        transmute_register::<DeviceContextBaseAddressArrayPointerRegister>(&mut addr);
    let configure = transmute_register::<ConfigureRegister>(&mut addr);
    OperationalRegisters::new(
        usb_cmd,
        usb_sts,
        page_size,
        device_notify,
        command_ring_control,
        device_context_bae_addr_array_ptr,
        configure,
    )
}


fn transmute_register<T: Debug>(addr: &mut u64) -> RegisterInfo<T> {
    let register = crate::utils::raw_ptr::transmute_register::<T>((*addr).clone());
    *addr += mem::size_of::<T>() as u64;
    register
}


// #[test_case]
// #[doc(hidden)]
// pub fn should_uncheck_transmute_operational_registers(){
//     let mmio_base = tmp_find_usb_mouse_base().unwrap();
//     let cap_len: u8 = CreateType::UncheckTransmute.new_capability(mmio_base).unwrap().read_volatile().cap_length.into();
//
//     let registers = uncheck_transmute(mmio_base, cap_len);
// }
