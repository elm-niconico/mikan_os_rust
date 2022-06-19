use crate::usb::xhci::registers::capability::structs::capability_registers_length::CapLength;
use crate::usb::xhci::registers::create_type::CreateType;
use crate::usb::xhci::registers::operational::create::command_ring_control::ICreateCommandRingControl;
use crate::usb::xhci::registers::operational::create::configure::ICreateConfigure;
use crate::usb::xhci::registers::operational::create::device_context_base_array_pointer::ICreateDeviceBaseAddressArrayPointer;
use crate::usb::xhci::registers::operational::create::device_notify_control::ICreateDeviceNotifyControl;
use crate::usb::xhci::registers::operational::create::page_size::ICreatePageSize;
use crate::usb::xhci::registers::operational::create::usb_cmd::CreateUsbCommand;
use crate::usb::xhci::registers::operational::create::usb_sts::ICreateUsbStatusRegister;
use crate::usb::xhci::registers::operational::structs::operational_registers::OperationalRegisters;


pub trait ICreateAllOperationalRegisters {
    fn new_all_operations(&self, mmio_base_addr: u64, cap_length: CapLength) -> Result<OperationalRegisters, ()>;
}


impl ICreateAllOperationalRegisters for CreateType {
    fn new_all_operations(&self, mmio_base_addr: u64, cap_len: CapLength) -> Result<OperationalRegisters, ()> {
        let cap_len: u8 = cap_len.into();
        let operational_base_addr = mmio_base_addr + cap_len as u64;
        let usb_cmd = self.new_usb_command(operational_base_addr)?;
        let usb_sts = self.new_usb_sts(operational_base_addr)?;
        let page_size = self.new_page_size(operational_base_addr)?;
        let device_notify = self.new_device_notify_control(operational_base_addr)?;
        let command_ring_control = self.new_command_ring_control(operational_base_addr)?;
        let device_context_bae_addr_array_ptr = self.new_device_context_base_address_array_pointer(operational_base_addr)?;
        let configure = self.new_configure(operational_base_addr)?;
        
        Ok(OperationalRegisters::new(
            usb_cmd,
            usb_sts,
            page_size,
            device_notify,
            command_ring_control,
            device_context_bae_addr_array_ptr,
            configure,
        ))
    }
}


// #[test_case]
// #[doc(hidden)]
// pub fn should_uncheck_transmute_operational_registers() {
//     let mmio_base_addr = tmp_find_usb_mouse_base().unwrap();
//     let uncheck = UncheckTransmute;
//     let cap_len = uncheck.new_capability(mmio_base_addr).unwrap().read_volatile().cap_length;
//     // let operational_registers = uncheck.new_operational(mmio_base_addr, cap_len);
//     //
//     // assert!(operational_registers.is_ok());
//     serial_println!("{:?}", cap_len);
// }


