use core::fmt::Debug;

use core::{mem, ptr};

use crate::usb::xhci::registers::r#struct::operators::command_ring_control::CommandRingControlRegister;
use crate::usb::xhci::registers::r#struct::operators::configure::ConfigureRegister;
use crate::usb::xhci::registers::r#struct::operators::device_context_base_address_array_pointer::DeviceContextBaseAddressArrayPointerRegister;
use crate::usb::xhci::registers::r#struct::operators::device_notification_control::DeviceNotificationControlRegister;
use crate::usb::xhci::registers::r#struct::operators::operational_registers::OperationalRegisters;
use crate::usb::xhci::registers::r#struct::operators::page_size::PageSizeRegister;
use crate::usb::xhci::registers::r#struct::operators::usb_cmd::UsbCmdRegister;
use crate::usb::xhci::registers::r#struct::operators::usb_sts::UsbStsRegister;
use crate::usb::xhci::registers::r#struct::register_info::RegisterInfo;

pub trait ICreateOperationalRegisters {
    fn create(&self, mmio_base_addr: u64, cap_length: u8) -> Result<OperationalRegisters, ()>;
}

#[allow(dead_code)]
pub enum CreateOperationalRegisters {
    // 生ポインタから強制的にキャスト
    UncheckTransmute,
}

impl ICreateOperationalRegisters for CreateOperationalRegisters {
    fn create(&self, mmio_base_addr: u64, cap_len: u8) -> Result<OperationalRegisters, ()> {
        match self {
            CreateOperationalRegisters::UncheckTransmute => {
                Ok(uncheck_transmute(mmio_base_addr, cap_len))
            }
        }
    }
}

fn uncheck_transmute(mmio_base_addr: u64, cap_len: u8) -> OperationalRegisters {
    let mut addr = mmio_base_addr + cap_len as u64;
    let usb_cmd = transmute::<UsbCmdRegister>(&mut addr);
    let usb_sts = transmute::<UsbStsRegister>(&mut addr);
    let page_size = transmute::<PageSizeRegister>(&mut addr);
    let device_notify = transmute::<DeviceNotificationControlRegister>(&mut addr);
    let command_ring_control = transmute::<CommandRingControlRegister>(&mut addr);
    let device_context_bae_addr_array_ptr =
        transmute::<DeviceContextBaseAddressArrayPointerRegister>(&mut addr);
    let configure = transmute::<ConfigureRegister>(&mut addr);
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

fn transmute<T: Debug>(addr: &mut u64) -> RegisterInfo<T> {
    let ptr = *addr as *const T;
    let size = mem::size_of::<T>();

    let register = unsafe { ptr::read_volatile(ptr) };
    let info = RegisterInfo::new(addr.clone(), register);
    *addr += size as u64;
    info
}
