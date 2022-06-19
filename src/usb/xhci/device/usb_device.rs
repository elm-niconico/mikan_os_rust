use crate::usb::xhci::registers::doorbell::doorbell::DoorbellRegister;


pub struct UsbDevice{
    slot_id: u8,
    doorbell_register: DoorbellRegister
}

