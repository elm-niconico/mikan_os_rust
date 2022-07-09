use x86_64::instructions::segmentation;
use x86_64::instructions::segmentation::Segment;
use x86_64::structures::gdt::{Descriptor, GlobalDescriptorTable, SegmentSelector};

use crate::spin::sync_once_cell::StaticOnceCell;

pub const DOUBLE_FAULT_IST_INDEX: u16 = 0;


static mut GDT: StaticOnceCell<GlobalDescriptorTable> = StaticOnceCell::uninit();

#[derive(Debug)]
pub(crate) struct Selectors {
    pub(crate) kernel_code_selector: SegmentSelector,
    pub(crate) kernel_stack_selector: SegmentSelector,
}

static mut SELECTORS: StaticOnceCell<Selectors> = StaticOnceCell::uninit();

pub fn init() {
    let null_segment = SegmentSelector(0);
    let mut selectors = Selectors {
        kernel_code_selector: null_segment,
        kernel_stack_selector: null_segment,
    };
    set_up_segment(&mut selectors);

    unsafe {
        segmentation::load_ds(null_segment);
        segmentation::load_es(null_segment);
        segmentation::load_fs(null_segment);
        segmentation::load_gs(null_segment);
    }
    
    // Set CS SS
    unsafe { segmentation::SS::set_reg(selectors.kernel_stack_selector) }
    unsafe { segmentation::CS::set_reg(selectors.kernel_code_selector) };

    unsafe { SELECTORS.init_once(|| selectors); }
}

fn set_up_segment(selectors: &mut Selectors) {
    let mut gdt = GlobalDescriptorTable::new();
    selectors.kernel_code_selector = gdt.add_entry(Descriptor::kernel_code_segment());
    selectors.kernel_stack_selector = gdt.add_entry(Descriptor::kernel_data_segment());
    unsafe { GDT.init_once(|| gdt) };
    unsafe { GDT.get().load(); }
}