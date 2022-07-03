/*
   ページテーブルを作成するための機構を提供します
*/

use crate::cell::sync_once_cell::SyncOnceCell;
use crate::paging::frame_allocator::boot_info::BootInfoFrameAllocator;

pub(crate) static mut FRAME_ALLOCATOR: SyncOnceCell<BootInfoFrameAllocator> = SyncOnceCell::new();

pub mod boot_info;