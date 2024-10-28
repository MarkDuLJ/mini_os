use x86_64::{ structures::paging::{ OffsetPageTable, PageTable}, PhysAddr, VirtAddr};

/// Return a mut ref to the active level 4 table
/// Only be called once to avoid multiple &mut references
pub unsafe fn active_lvl_4_table(physical_memory_offset: VirtAddr)
        -> &'static mut PageTable
{
    use x86_64::registers::control::Cr3;
    let (level_4_table_frame, _) = Cr3::read();

    let phys = level_4_table_frame.start_address();
    let virt = physical_memory_offset + phys.as_u64();
    let page_table_ptr: *mut PageTable = virt.as_mut_ptr();

    &mut *page_table_ptr
}


/// Translate a given virtual address to the mapped physical address or None if not mapped
pub unsafe fn translate_addr(addr: VirtAddr, physical_memory_offset: VirtAddr) -> Option<PhysAddr> {
    translate_addr_inner(addr, physical_memory_offset)
}

/// Private function is called by translate_addr
/// 
/// this function is safe to limit the scope of unsafe translate_addr function because
/// Rust treats the whole body of unsafe functions as an unsafe block. Current function 
/// is only reachable through unsafe fn from outside of this module.
fn translate_addr_inner(addr: VirtAddr, physical_memory_offset: VirtAddr) -> Option<PhysAddr> {
    use x86_64::structures::paging::page_table::FrameError;
    use x86_64::registers::control::Cr3;

    let (level_4_table_frame, _) = Cr3::read();

    let table_indexes = [
        addr.p4_index(),
        addr.p3_index(),
        addr.p2_index(),
        addr.p1_index(),
    ];

    let mut frame = level_4_table_frame;

    //traverse the multi-level page table
    for &index in &table_indexes {
        //convert frame into a page table reference
        let virt = physical_memory_offset + frame.start_address().as_u64();
        let table_ptr: *const PageTable = virt.as_ptr();
        let table = unsafe {
            &*table_ptr
        };

        // read the page table entry and update frame
        let entry = &table[index];
        frame = match entry.frame() {
            Ok(frame) => frame,
            Err(FrameError::FrameNotPresent) => return  None,
            Err(FrameError::HugeFrame) => panic!("Not support huge pages"),
        };
    }

    Some(frame.start_address() + u64::from(addr.page_offset()))
}

//instead write tanslate function by myself, use x86_64 crate OffsetPageTable

/// Initialize a new OffsetPageTable
pub unsafe fn init(physical_memory_offset: VirtAddr) -> OffsetPageTable<'static> {
    let l4_table = active_lvl_4_table(physical_memory_offset);
    OffsetPageTable::new(l4_table, physical_memory_offset)
}