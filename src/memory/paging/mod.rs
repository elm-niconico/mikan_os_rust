pub(crate) use identity::make_identity_mapping;
pub(crate) use offset::init;
pub(crate) use offset::PAGE_TABLE;

pub mod identity;
mod offset;


// pub(crate) unsafe fn init(
//     physical_memory_offset: VirtAddr,
//     memory_regions: &'static mut MemoryRegions,
// ) {
//     // heap::init(memory_regions);
//     // page_mapper::init(physical_memory_offset);
// }