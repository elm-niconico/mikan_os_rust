use crate::usb::xhci::registers::doorbells::doorbell::DoorbellRegister;


pub struct UsbDevice{
    slot_id: u8,
    doorbell_register: DoorbellRegister
}

