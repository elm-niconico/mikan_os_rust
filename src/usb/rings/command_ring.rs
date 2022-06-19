use crate::usb::xhci::trb::trb_base::TrbBase;


pub struct CommandRing {
    
    pub ring_buffer: [Option<TrbBase>; 32],
}



impl CommandRing {

    pub fn new()-> Self{
        Self{
            ring_buffer: [Option::None; 32]
        }
    }
    
    pub fn buffer_addr(&self) -> u64{
        self.ring_buffer.as_ptr().addr() as u64
    }
    
}
