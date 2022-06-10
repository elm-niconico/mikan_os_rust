use x86_64::instructions::port::Port;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
#[repr(u32)]
pub enum QemuExitCode {
    Success = 0x10,
    Failed = 0x11,
}

pub fn exit_qemu(exit_code: QemuExitCode) {
    //isa-debug-exitデバイスのiobase
    let io_base = 0xf4;
    let mut port = Port::new(io_base);

    // QEMUは(exit_code << 1) | 1を終了コードにする
    unsafe {
        port.write(exit_code as u32);
    }
}
