use bootloader::boot_info::{MemoryRegionKind, MemoryRegions};
use x86_64::structures::paging::{FrameAllocator, PhysFrame, Size4KiB};
use x86_64::PhysAddr;

pub(crate) struct BootInfoFrameAllocator {
    memory_regions: &'static MemoryRegions,
    next: usize,
}

impl BootInfoFrameAllocator {
    pub(crate) unsafe fn init(memory_map: &'static MemoryRegions) -> Self {
        Self {
            memory_regions: memory_map,
            next: 0,
        }
    }
    fn usable_frames(&mut self) -> impl Iterator<Item = PhysFrame> {
        // 使用できるフレームを抽出
        let usable_regions = self
            .memory_regions
            .iter()
            .filter(|r| r.kind == MemoryRegionKind::Usable);

        let addr_ranges = usable_regions.map(|r| r.start..r.end);

        let frame_addresses = addr_ranges.flat_map(|r| r.step_by(4096));

        frame_addresses.map(|addr| PhysFrame::containing_address(PhysAddr::new(addr)))
    }
}

unsafe impl FrameAllocator<Size4KiB> for BootInfoFrameAllocator {
    fn allocate_frame(&mut self) -> Option<PhysFrame<Size4KiB>> {
        let frame = self.usable_frames().nth(self.next);
        self.next += 1;
        frame
    }
}