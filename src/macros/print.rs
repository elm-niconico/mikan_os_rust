#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::frame_buffer::_print(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::print!("{}\n", format_args!($($arg)*)));
}

#[macro_export]
macro_rules! serial_print {
    ($($arg:tt)*) => {
        $crate::serial_port::_print(format_args!($($arg)*));
    };
}

#[macro_export]
macro_rules! serial_println {
    () => {$crate::serial_print!("\n")};
    ($fmt:expr) => {$crate::serial_print!(concat!($fmt, "\n"))};
    ($fmt:expr, $($arg:tt)*) => ($crate::serial_print!(concat!($fmt, "\n"), $($arg)*));
}

#[macro_export]
macro_rules! log {
    () => {
        $crate::println!();
        $crate::serial_println!();
    };

    ($fmt: expr) => {
        //$crate::println!($fmt);
        $crate::serial_println!($fmt);
    };

    ($fmt: expr, $($arg:tt)*) => {
        //$crate::println!($fmt,  $($arg)*);
        $crate::serial_println!($fmt,  $($arg)*);
    };

}

// 動作確認用
#[macro_export]
macro_rules! print_virtual_addr {
    ($addr: expr) => {{
        let virtual_addr = x86_64::VirtAddr::new($addr);
        $crate::println!("virtual_addr u64: {:?}", virtual_addr);
        $crate::println!("virtual_addr Page1 Index {:?}", virtual_addr.p1_index());
        $crate::println!("virtual_addr Page2 Index {:?}", virtual_addr.p2_index());
        $crate::println!("virtual_addr Page3 Index {:?}", virtual_addr.p3_index());
        $crate::println!("virtual_addr Page4 Index {:?}", virtual_addr.p4_index());
        $crate::println!("virtual_addr Page Offset {:?}", virtual_addr.page_offset());
    }};

    ($description: expr, $addr: expr) => {
        $crate::println!("{:?}", $description);
        $crate::print_virtual_addr!($addr);
    };
}