use std::{
    path::PathBuf,
    process::{Command, ExitStatus},
    time::Duration,
};

use anyhow::anyhow;

const RUN_ARGS: &[&str] = &[
    "-serial",
    "stdio",
    "-device",
    "isa-debug-exit,iobase=0xf4,iosize=0x04",
    "-device",
    "nec-usb-xhci,id=xhci",
    "-device",
    "usb-mouse",
    "-gdb",
    "tcp::3022",
    "-S",
];
const TEST_ARGS: &[&str] = &[
    "-device",
    "isa-debug-exit,iobase=0xf4,iosize=0x04",
    "-serial",
    "stdio",
    "-display",
    "none",
    "--no-reboot",
];
const TEST_TIMEOUT_SECS: u64 = 10;

fn main() -> anyhow::Result<()> {
    println!("hello");
    let kernel_binary_path = {
        let path = PathBuf::from(std::env::args().nth(1).unwrap());
        path.canonicalize()?
    };
    println!("befoer create Image");
    let disk_image = disk_image::create_disk_image(&kernel_binary_path, true)?;
    println!("create Image");
    let mut run_cmd = Command::new("qemu-system-x86_64");
    run_cmd
        .arg("-drive")
        .arg(format!("format=raw,file={}", disk_image.display()));

    let binary_kind = runner_utils::binary_kind(&kernel_binary_path);
    if binary_kind.is_test() {
        run_cmd.args(TEST_ARGS);

        let exit_status = run_test_command(run_cmd)?;
        match exit_status.code() {
            Some(33) => {} // success
            other => return Err(anyhow!("Test failed (exit code: {:?})", other)),
        }
    } else {
        run_cmd.args(RUN_ARGS);

        let exit_status = run_cmd.status()?;
        if !exit_status.success() {
            std::process::exit(exit_status.code().unwrap_or(1));
        }
    }

    Ok(())
}

fn run_test_command(mut cmd: Command) -> anyhow::Result<ExitStatus> {
    let status = runner_utils::run_with_timeout(&mut cmd, Duration::from_secs(TEST_TIMEOUT_SECS))?;
    Ok(status)
}