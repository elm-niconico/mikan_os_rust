use bootloader::boot_info::MemoryRegions;

pub(crate) trait InitAllocator {
    fn new(memory_regions: &'static MemoryRegions) -> Self;
}