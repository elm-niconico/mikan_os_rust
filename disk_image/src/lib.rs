use std::{
    path::{Path, PathBuf},
    process::Command,
};

use anyhow::anyhow;

pub fn create_disk_image(kernel_binary_path: &Path) -> anyhow::Result<PathBuf> {
    let bootloader_manifest_path = bootloader_locator::locate_bootloader("bootloader")?;
    let kernel_manifest_path = locate_cargo_manifest::locate_manifest()?;

    let mut build_cmd = Command::new(env!("CARGO"));
    build_cmd.current_dir(bootloader_manifest_path.parent().unwrap());
    build_cmd.arg("builder");
    build_cmd
        .arg("--kernel-manifest")
        .arg(&kernel_manifest_path);
    build_cmd.arg("--kernel-binary").arg(&kernel_binary_path);

    build_cmd
        .arg("--target-dir")
        .arg(kernel_manifest_path.parent().unwrap().join("target"));
    build_cmd
        .arg("--out-dir")
        .arg(kernel_binary_path.parent().unwrap());
    build_cmd.arg("--quiet");

    // if bios_only {
    //     build_cmd.arg("--firmware").arg("bios");
    // }

    if !build_cmd.status()?.success() {
        return Err(anyhow!("build failed"));
    }

    let kernel_binary_name = kernel_binary_path.file_name().unwrap().to_str().unwrap();
    let disk_image = kernel_binary_path
        .parent()
        .unwrap()
        .join(format!("boot-uefi-{}.img", kernel_binary_name));
    if !disk_image.exists() {
        return Err(anyhow!(
            "Disk image does not exist at {} after bootloader build",
            disk_image.display()
        ));
    }
    Ok(disk_image)
}