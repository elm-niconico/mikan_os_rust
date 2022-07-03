use crate::usb::xhci::trb::trb_base::TrbBase;

#[derive(Debug)]
#[allow(unused)]
pub struct CommandRing {
    pub ring_buffer: [TrbBase; 32],
}

impl CommandRing {
    #[allow(unused)]
    pub fn new() -> Self {
        Self {
            ring_buffer: [TrbBase::new_zeros(); 32],
        }
    }

    #[allow(unused)]
    pub fn buffer_addr(&self) -> u64 {
        self.ring_buffer.as_ptr().addr() as u64
    }
}