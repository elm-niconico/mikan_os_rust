use x86_64::structures::paging::{OffsetPageTable, PageTable};
use x86_64::VirtAddr;

use crate::println;
use crate::spin::sync_mutex::StaticSpinMutex;
use crate::spin::sync_once_cell::StaticOnceCell;

pub(crate) static mut PAGE_TABLE: StaticOnceCell<StaticSpinMutex<OffsetPageTable>> =
    StaticOnceCell::uninit();


pub(crate) unsafe fn init(phys_offset: VirtAddr) {
    let page_table = &mut *active_level_4_table(phys_offset);

    PAGE_TABLE
        .init_once(|| StaticSpinMutex::new(OffsetPageTable::new(page_table, phys_offset)))
}


// レベル4テーブルのポインターを返します
// [map-physical-memory]によって、全物理アドレスが特定のオフセット値をつかって
// 仮想メモリにマップされていなければなりません。
#[allow(unused)]
pub(crate) unsafe fn active_level_4_table(phys_offset: VirtAddr) -> *mut PageTable {
    let (frame, _) = x86_64::registers::control::Cr3::read();
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