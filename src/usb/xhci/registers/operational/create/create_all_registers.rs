use crate::usb::xhci::registers::capability::structs::capability_registers_length::CapLength;
use crate::usb::xhci::registers::create_type::RegisterCreate;
use crate::usb::xhci::registers::operational::create::create_configure::ICreateConfigure;
use crate::usb::xhci::registers::operational::create::create_crcr::ICreateCrcr;
use crate::usb::xhci::registers::operational::create::create_dcbaap::ICreateDcbaap;
use crate::usb::xhci::registers::operational::create::create_dnc::ICreateDnctrl;
use crate::usb::xhci::registers::operational::create::create_page_size::ICreatePageSize;
use crate::usb::xhci::registers::operational::create::create_usb_cmd::CreateUsbCommand;
use crate::usb::xhci::registers::operational::create::create_usb_sts::ICreateUsbStatusRegister;
use crate::usb::xhci::registers::operational::structs::operational_registers::OperationalRegisters;
use crate::utils::error::CommonResult;

pub trait ICreateAllOperationalRegisters {
    fn new_operations(
        &self,
        mmio_base_addr: u64,
        cap_length: CapLength,
    ) -> CommonResult<OperationalRegisters>;
}

impl ICreateAllOperationalRegisters for RegisterCreate {
    fn new_operations(
        &self,
        mmio_base_addr: u64,
        cap_len: CapLength,
    ) -> CommonResult<OperationalRegisters> {
        let cap_len: u8 = cap_len.into();
        let operational_base_addr = mmio_base_addr + cap_len as u64;
        let usb_cmd = self.new_usb_command(operational_base_addr)?;
        let usb_sts = self.new_usb_sts(operational_base_addr)?;
        let page_size = self.new_page_size(operational_base_addr)?;
        let device_notify = self.new_dnctrl(operational_base_addr)?;
        let command_ring_control = self.new_crcr(operational_base_addr)?;
        let device_context_bae_addr_array_ptr = self.new_dcbaap(operational_base_addr)?;
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