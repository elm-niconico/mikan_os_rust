use core::fmt::Debug;

use crate::impl_debug_only_fields;
use crate::usb::xhci::registers::operational::structs::command_ring_control::CommandRingControlRegister;
use crate::usb::xhci::registers::operational::structs::configure::ConfigureRegister;
use crate::usb::xhci::registers::operational::structs::dcbaap::Dcbaap;
use crate::usb::xhci::registers::operational::structs::device_notification_control::DeviceNotificationControlRegister;
use crate::usb::xhci::registers::operational::structs::page_size::PageSizeRegister;
use crate::usb::xhci::registers::operational::structs::usb_cmd::UsbCmdRegister;
use crate::usb::xhci::registers::operational::structs::usb_sts::UsbStsRegister;
use crate::usb::xhci::registers::read_write::volatile::Volatile;
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

