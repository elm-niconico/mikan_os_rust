use core::mem;

use modular_bitfield::private::static_assertions;
use x86_64::VirtAddr;

use crate::{FRAME_ALLOCATOR, log, PAGE_TABLE};
use crate::memory::paging::make_identity_mapping;

mod idt;
mod pic;
mod apic;

#[derive(Debug)]
#[repr(C)]
pub(crate) struct Rsdp {
    signature: [u8; 8],
    checksum: u8,
    oem_id: [u8; 6],
    revision: u8,
    rsdt_address: u32,
    length: u32,
    xsdt_address: u64,
    extended_checksum: u8,
    reserved: [u8; 3],
}

#[derive(Debug)]
#[repr(C)]
struct DescriptionHeader {
    signature: [u8; 4],
    length: u32,
    revision: u8,
    checksum: u8,
    oem_id: [u8; 6],
    oem_table_id: [u8; 8],
    oem_revision: u32,
    creator_id: u32,
    creator_revision: u32,
}
static_assertions::const_assert_eq!(mem::size_of::<DescriptionHeader>(), 36);

/// Extended System Descriptor Table
#[derive(Debug)]
#[repr(C)]
struct Xsdt {
    header: DescriptionHeader,
}

impl DescriptionHeader {
    fn len(&self) -> usize {
        self.length as usize
    }
}

impl Xsdt {
    fn len(&self) -> usize {
        (self.header.len() - mem::size_of::<DescriptionHeader>()) / mem::size_of::<usize>()
    }

    fn entries(&self) -> impl Iterator<Item=u32> {
        // `array_head` is not 8-byte aligned, so we cannot treat it as normal `*const u64`.
        // For example, `slice::from_raw_parts(array_head, len)` panics in debug build.
        let array_head =
            unsafe { (&self.header as *const DescriptionHeader).add(1) as *const [u8; 4] };
        (0..self.len()).map(move |idx| {
            let bytes = unsafe { array_head.add(idx).read() };
            u32::from_le_bytes(bytes)
        })
    }
}

pub(crate) fn init(rsdp: u64) {
    unsafe { idt::init_idt() };
    log!("Init IDT");

    //unsafe { pic::PICS.lock().initialize() }


    const TIMER_FRAME_BASE_ADDR: u64 = 0xfee00000;
    map(TIMER_FRAME_BASE_ADDR);

    apic::timer::timer_manager::APIC_TIMER.lock().init();
    log!("Init APIC Timer");


}

pub fn map(rsdp: u64) {
    let mapper = &mut *PAGE_TABLE.get().lock();
    let frame_allocator = &mut *FRAME_ALLOCATOR.get().lock();
    let base_addr = VirtAddr::new(rsdp).align_down(4096u64).as_u64();
    make_identity_mapping(mapper, frame_allocator, base_addr, 1).expect("Failed Rsdp Mapping");
}