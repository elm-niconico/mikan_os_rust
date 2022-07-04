use x86_64::registers::control::Cr3;
use x86_64::structures::paging::{
    Mapper, OffsetPageTable, Page, PageTable, PageTableFlags, PhysFrame, Size4KiB,
};
use x86_64::{PhysAddr, VirtAddr};

use crate::cell::sync_once_cell::SyncOnceCell;
use crate::error::KernelResult;
use crate::paging::frame_allocator::FRAME_ALLOCATOR;
use crate::{log, println};

pub(crate) static mut PAGE_MAPPER: SyncOnceCell<OffsetPageTable> = SyncOnceCell::new();

pub(crate) unsafe fn init(phys_offset: VirtAddr) {
    let page_table = &mut *active_level_4_table(phys_offset);
    PAGE_MAPPER
        .set(OffsetPageTable::new(page_table, phys_offset))
        .expect("Failed Init Page Mapper");
}

pub(crate) fn map(addr: u64, physical_offset: VirtAddr) -> KernelResult<()> {
    let page = Page::<Size4KiB>::containing_address(VirtAddr::new(addr));

    let frame = PhysFrame::containing_address(PhysAddr::new(addr));
    let allocator = unsafe { FRAME_ALLOCATOR.get_mut().unwrap() };
    // let frame = allocator.allocate_frame().unwrap();

    let frags = PageTableFlags::empty();

    let map = unsafe {
        PAGE_MAPPER
            .get_mut()
            .unwrap()
            .map_to(page, frame, frags, allocator)
    }?;

    log!("Success Mapping");
    map.flush();
    Ok(())
}

// レベル4テーブルのポインターを返します
// [map-physical-memory]によって、全物理アドレスが特定のオフセット値をつかって
// 仮想メモリにマップされていなければなりません。
#[allow(unused)]
pub(crate) unsafe fn active_level_4_table(phys_offset: VirtAddr) -> *mut PageTable {
    let (frame, _) = Cr3::read();
    let physical_start = frame.start_address();
    let virtual_start = phys_offset + physical_start.as_u64();
    virtual_start.as_mut_ptr() as *mut PageTable
}

// 動作確認用用
// ページテーブルから空ではないエントリを出力します。
#[allow(unused)]
pub(crate) fn print_all_use_entries(page_table: &PageTable) {
    for (i, entry) in page_table.iter().enumerate() {
        if !entry.is_unused() {
            println!("Table Entry {}: {:?}", i, entry);
        }
    }
}