use core::alloc::{GlobalAlloc, Layout};
use core::cell::UnsafeCell;
use core::ptr;

use crate::memory::heap::{HEAP_SIZE, HEAP_START};

/// 制作初期段階で実装した不完全なメモリアロケータです。
/// 動作確認用途以外では使用しません。
pub(crate) struct DummyAllocator {
    pub head: UnsafeCell<usize>,
    pub end: usize,
}

#[allow(unused)]
impl DummyAllocator {
    pub fn new() -> Self {
        DummyAllocator {
            head: UnsafeCell::new(HEAP_START),
            end: HEAP_START + HEAP_SIZE,
        }
    }
}

unsafe impl Sync for DummyAllocator {}

unsafe impl GlobalAlloc for DummyAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let head = self.head.get();

        let align = layout.align();
        let res = *head % align;
        let start = if res == 0 { *head } else { *head + align - res };
        if start + align > self.end {
            // ヌルポインタはメモリ不足の状態を知らせます
            ptr::null_mut()
        } else {
            *head = start + align;
            start as *mut u8
        }
    }

    unsafe fn dealloc(&self, _: *mut u8, _: Layout) {
        // このアロケータはメモリを解放しません
    }
}