use core::alloc::{GlobalAlloc, Layout};
use core::cell::UnsafeCell;
use core::ptr;
use crate::{exit_qemu, println};
use crate::QemuExitCode::Failed;


// グローバルメモリアロケータの宣言
// ユーザはメモリ領域の`[0x2000_0100, 0x2000_0200]`がプログラムの他の部分で使用されないことを
// 保証しなければなりません
#[global_allocator]
pub static HEAP: BumpPointerAlloc = BumpPointerAlloc {
    head: UnsafeCell::new(0x2000_0100),
    end: 0x2000_0200,
};


#[alloc_error_handler]
pub fn on_oom(_layout: Layout) -> ! {
    
    println!("alloc error!");
    exit_qemu(Failed);
    loop {}
}


pub struct BumpPointerAlloc {
    pub head: UnsafeCell<usize>,
    pub end: usize,
}


unsafe impl Sync for BumpPointerAlloc {}


unsafe impl GlobalAlloc for BumpPointerAlloc {
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
