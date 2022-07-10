use x86_64::VirtAddr;

use crate::usb::xhci::trb::trb_base::TrbBase;

#[derive(Debug)]
#[repr(align(64))]
pub struct CommandRing {
    data_buff: [TrbBase; 32],
}

pub trait CommandRingAddress {
    fn command_ring_base_addr(&self) -> VirtAddr;
}


impl CommandRing {
    #[allow(unused)]
    pub fn new() -> Self {
        Self {
            data_buff: [TrbBase::new_zeros(); 32],
        }
    }

    #[allow(unused)]
    fn buff_addr(&self) -> u64 {
        self.data_buff.as_ptr().addr() as u64
    }
}


impl CommandRingAddress for CommandRing {
    fn command_ring_base_addr(&self) -> VirtAddr {
        VirtAddr::new(self.buff_addr())
    }
}