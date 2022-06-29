#[macro_export]
macro_rules! test_cap_register {
    ($name: tt, $fn: ident) => {
        crate::test_register!(
            $name,
            $fn,
            crate::utils::test_fn::extract_virtual_mmio_base_addr()
        );
    };
}

#[macro_export]
macro_rules! test_op_register {
    ($name: tt, $fn: ident) => {
        crate::test_register!(
            $name,
            $fn,
            crate::utils::test_fn::extract_operational_base_addr()
        );
    };
}

#[macro_export]
macro_rules! test_runtime_register {
    ($name: tt, $fn: ident) => {
        crate::test_register!($name, $fn, crate::utils::test_fn::extract_runtime_base());
    };
}

#[macro_export]
macro_rules! test_register {
    ($name: tt, $fn: ident, $addr: expr) => {
        #[test_case]
        pub fn $name() {
            let register = $fn($addr);
            //assert!(register.is_ok());
            //crate::serial_println!("{:?}", register.unwrap());
        }
    };
}

#[macro_export]
macro_rules! test_template {
    () => {
        extern crate alloc;

        use bootloader::{entry_point, BootInfo};
        use core::panic::PanicInfo;
        use mikan_os_rust::page::active_level_4_table;
        use x86_64::structures::paging::OffsetPageTable;

        entry_point!(main);

        fn main(boot_info: &'static BootInfo) -> ! {
            use x86_64::VirtAddr;

            let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
            let mut level_4_table = unsafe { active_level_4_table(phys_mem_offset) };
            let mut mapper =
                unsafe { OffsetPageTable::new(&mut (*level_4_table), phys_mem_offset) };
            let mut frame_allocator = unsafe {
                mikan_os_rust::page::frame_allocator::boot_info::BootInfoFrameAllocator::init(
                    &boot_info.memory_map,
                )
            };

            mikan_os_rust::allocators::init_heap(&mut mapper, &mut frame_allocator)
                .expect("heap initialization failed");
            test_main();
            loop {}
        }

        #[panic_handler]
        fn panic(info: &PanicInfo) -> ! {
            mikan_os_rust::test_panic_handler(info);
        }
    };
}