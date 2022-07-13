use core::alloc::Layout;
use core::ptr::drop_in_place;

use x86_64::VirtAddr;
use xhci::ring::trb::command::EnableSlot;
use xhci::ring::trb::transfer::Noop;
use xhci::ring::trb::Type::NoopCommand;

use crate::memory::heap::HEAP;

#[derive(Debug)]
#[repr(align(64))]
pub struct CommandRing {
    data_buff: *mut Noop,
}

pub trait CommandRingAddress {
    fn command_ring_base_addr(&self) -> VirtAddr;
}


impl CommandRing {
    #[allow(unused)]
    pub fn new() -> Self {
        let ptr = unsafe {
            HEAP
                .lock()
                .allocate_first_fit(Layout::from_size_align_unchecked(32 * 32, 64))
                .expect("Failed Alloc Command Ring Buff")
                .as_ptr()
                as *mut Noop
        };
        Self {
            data_buff: ptr
        }
    }
    pub fn push(&mut self) {
        // self.data_buff = unsafe { self.data_buff.add(1) };

        unsafe { self.data_buff.write_volatile(Noop::default()) };
    }

    #[allow(unused)]
    fn buff_addr(&self) -> u64 { unsafe { self.data_buff.addr() as u64 } }
}


impl CommandRingAddress for CommandRing {
    fn command_ring_base_addr(&self) -> VirtAddr {
        VirtAddr::new(self.buff_addr())
    }
}

impl Drop for CommandRing {
    fn drop(&mut self) {
        unsafe { drop_in_place(self.data_buff) };
    }
}