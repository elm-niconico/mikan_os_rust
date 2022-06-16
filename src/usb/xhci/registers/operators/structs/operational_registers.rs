use core::fmt::Debug;

use crate::impl_debug_only_fields;
use crate::usb::xhci::registers::operators::structs::command_ring_control::CommandRingControlRegister;
use crate::usb::xhci::registers::operators::structs::configure::ConfigureRegister;
use crate::usb::xhci::registers::operators::structs::device_context_base_address_array_pointer::DeviceContextBaseAddressArrayPointerRegister;
use crate::usb::xhci::registers::operators::structs::device_notification_control::DeviceNotificationControlRegister;
use crate::usb::xhci::registers::operators::structs::page_size::PageSizeRegister;
use crate::usb::xhci::registers::operators::structs::usb_cmd::UsbCmdRegister;
use crate::usb::xhci::registers::operators::structs::usb_sts::UsbStsRegister;
use crate::usb::xhci::registers::read_write::volatile::Volatile;
use crate::usb::xhci::registers::register_info::RegisterInfo;


#[allow(dead_code)]
pub struct OperationalRegisters {
    usb_cmd: Volatile<UsbCmdRegister>,
    usb_sts: Volatile<UsbStsRegister>,
    page_size: Volatile<PageSizeRegister>,
    device_notify: Volatile<DeviceNotificationControlRegister>,
    command_ring_control: Volatile<CommandRingControlRegister>,
    device_context_bae_addr_array_ptr: Volatile<DeviceContextBaseAddressArrayPointerRegister>,
    configure: Volatile<ConfigureRegister>,
}


impl OperationalRegisters {
    pub fn new(
        usb_cmd: RegisterInfo<UsbCmdRegister>,
        usb_sts: RegisterInfo<UsbStsRegister>,
        page_size: RegisterInfo<PageSizeRegister>,
        device_notify: RegisterInfo<DeviceNotificationControlRegister>,
        command_ring_control: RegisterInfo<CommandRingControlRegister>,
        device_context_bae_addr_array_ptr: RegisterInfo<
            DeviceContextBaseAddressArrayPointerRegister,
        >,
        configure: RegisterInfo<ConfigureRegister>,
    ) -> Self {
        Self {
            usb_cmd: new_volatile(usb_cmd),
            usb_sts: new_volatile(usb_sts),
            page_size: new_volatile(page_size),
            device_notify: new_volatile(device_notify),
            command_ring_control: new_volatile(command_ring_control),
            device_context_bae_addr_array_ptr: new_volatile(device_context_bae_addr_array_ptr),
            configure: new_volatile(configure),
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
        device_notify
    }
}

