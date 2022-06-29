use x86_64::registers::control::Cr3;
use x86_64::structures::paging::PageTable;
use x86_64::VirtAddr;

pub mod frame_allocator;

pub unsafe fn active_level_4_table(phys_offset: VirtAddr) -> *mut PageTable {
    let (frame, _) = Cr3::read();
    let physical_start = frame.start_address();
    let virtual_start = phys_offset + physical_start.as_u64();
    virtual_start.as_mut_ptr() as *mut PageTable
}