use bootloader::boot_info::MemoryRegions;
use x86_64::PhysAddr;
use x86_64::structures::paging::{FrameAllocator, PhysFrame, Size4KiB};
use x86_64::structures::paging::frame::PhysFrameRange;

use crate::error::{KernelError, KernelResult};
use crate::memory::frame::bit_map::{BitReadable, BitWritable};
use crate::memory::frame::frame_init::InitAllocator;

const fn kib(data: u64) -> u64 {
    data * 1024
}


const fn mib(num: u64) -> u64 {
    num * kib(1024)
}

const fn gib(num: u64) -> u64 {
    num * mib(1024)
}

pub(crate) const BYTES_PER_FRAME: u64 = kib(4);
const MAX_PHYSICAL_MEMORY_BYTE: u64 = gib(128);
const FRAME_COUNT: u64 = MAX_PHYSICAL_MEMORY_BYTE / BYTES_PER_FRAME;

type MapLine = u64;

const BITS_PER_MAP_LINE: u64 = MapLine::BITS as u64;
const ALLOC_MAP_LEN: usize = (FRAME_COUNT / (BITS_PER_MAP_LINE as u64)) as usize;

fn to_frame_range(memory_regions: &'static MemoryRegions) -> PhysFrameRange {
    let start = PhysFrame::containing_address(PhysAddr::new(memory_regions.first().unwrap().start));
    let end = PhysFrame::containing_address(PhysAddr::new(memory_regions.last().unwrap().start));
    PhysFrame::range(start, end)
}

fn calc_frame_id(frame: PhysFrame) -> u64 {
    frame.start_address().as_u64() / BITS_PER_MAP_LINE as u64
}


/// 恒等変換(Identity Mapping)を行うための機構を提供します。
/// 仮想アドレスと物理アドレスが一致するようにします。
pub(crate) struct BitMapFrameAllocator {
    alloc_map: [u64; ALLOC_MAP_LEN],
    frames: PhysFrameRange,
}

impl InitAllocator for BitMapFrameAllocator {
    fn new(memory_regions: &'static MemoryRegions) -> Self {
        Self {
            alloc_map: [0; ALLOC_MAP_LEN],
            frames: to_frame_range(memory_regions),
        }
    }
}

impl BitMapFrameAllocator {
    pub fn allocate(&mut self, num_frames: usize) -> KernelResult<PhysFrame> {
        loop {
            for i in 0..num_frames {
                if self.frames.start == self.frames.end {
                    return Err(KernelError::None);
                }

                if self.read_bit(self.frames.start) {
                    self.frames.start += 1;
                    break;
                }

                if i == num_frames - 1 {
                    return Ok(self.mark_allocated(num_frames));
                }
            }
        }
    }


    // TODO
    fn mark_allocated(&mut self, num_frames: usize) -> PhysFrame {
        for f in 0..num_frames {
            self.write_bit(self.frames.start, true);
            self.frames.start += 1;
        }

        self.frames.start
    }


    fn calc_index(&self, frame: PhysFrame) -> (usize, usize) {
        let frame_id = frame.start_address().as_u64() / BITS_PER_MAP_LINE as u64;
        let line_index = (frame_id / BITS_PER_MAP_LINE) as usize;
        let bit_index = (frame_id & BITS_PER_MAP_LINE) as usize;
        (line_index, bit_index)
    }
}


impl BitWritable for BitMapFrameAllocator {
    fn write_bit(&mut self, frame: PhysFrame, is_allocated: bool) {
        let (line_index, bit_index) = self.calc_index(frame);
        if is_allocated {
            self.alloc_map[line_index] |= (1) << bit_index;
        } else {
            self.alloc_map[line_index] &= !(1 << bit_index);
        }
    }
}

impl BitReadable for BitMapFrameAllocator {
    fn read_bit(&self, frame: PhysFrame) -> bool {
        let (line_index, bit_index) = self.calc_index(frame);
        (self.alloc_map[line_index] & (1 << bit_index)) != 0
    }
}


unsafe impl FrameAllocator<Size4KiB> for BitMapFrameAllocator {
    fn allocate_frame(&mut self) -> Option<PhysFrame<Size4KiB>> {
        self.allocate(1).ok()
    }
}


fn set_up_identity_page_table() {}