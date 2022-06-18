use core::alloc::Layout;
use core::cell::UnsafeCell;

use crate::{exit_qemu, println, QemuExitCode};
use crate::allocators::dummy_allocator::BumpPointerAlloc;


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
    exit_qemu(QemuExitCode::Failed);
    loop {}
}
