use alloc::alloc::{GlobalAlloc, Layout};
use x86_64::{structures::paging::{
    mapper::MapToError, FrameAllocator, Mapper, Page, PageTableFlags, Size4KiB}, 
    VirtAddr
};
use core::ptr::null_mut;
use linked_list_allocator::LockedHeap;
use bump_allocator::BumpAllocator;

pub mod bump_allocator;

/**
 * Allocator Design Goals
 * 1, manage the available heap memory.
 *      -return unused mem on alloc
 *      -keep track of mem freed by dealloc
 *      -never hand out mem that is already in use
 * 2, effectively utilize the available memory and keep fragmentation low.
 * 3, work well for concurrent apps and scale to any number processors.
 * 4, optimize the mem layout to improve cache locality and avoid false sharing.
 */

#[global_allocator]
// static ALLOCATOR: LockedHeap = LockedHeap::empty(); // use crate dependency
static ALLOCATOR: Lokced<BumpAllocator> = Lokced::new(BumpAllocator::new());

pub struct Dummy;

unsafe impl GlobalAlloc for Dummy {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        null_mut()
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        panic!("dealloc failed.")
    }
}

/// createa heap memory
/// define a virual memory range for the heap region and map this to physical frames
pub const HEAP_START: usize = 0x_4444_4444_0000;
pub const HEAP_SIZE: usize = 100 * 1024; //100kiB

pub fn init_heap(
    mapper: &mut impl Mapper<Size4KiB>,
    frame_allocator: &mut impl FrameAllocator<Size4KiB>,
) -> Result<(), MapToError<Size4KiB>>
{
    // create page range
    let page_range = {
        let heap_start = VirtAddr::new(HEAP_START as u64);
        let heap_end = heap_start + HEAP_SIZE -1 as u64;

        let heap_start_page = Page::containing_address(heap_start);
        let heap_end_page = Page::containing_address(heap_end);

        Page::range_inclusive(heap_start_page, heap_end_page)
    };

    for page in page_range { //mapping the pages
        let frame = frame_allocator
                .allocate_frame()
                .ok_or(MapToError::FrameAllocationFailed)?;

        let flags = PageTableFlags::PRESENT | PageTableFlags::WRITABLE;

        unsafe {
            mapper.map_to(page, frame, flags, frame_allocator)?.flush()
        };
    }

    unsafe { //init allocator after creating heap
        ALLOCATOR.lock().init(HEAP_START, HEAP_SIZE);
    }

    Ok(())
}


// a wrapper aound spin::Mutex to permit trait implemntations
// for the customer allocators
pub struct Lokced<A> {
    inner: spin::Mutex<A>,
}

impl <A> Lokced<A> {
    pub const fn new(inner: A) -> Self {
        Lokced{
            inner: spin::Mutex::new(inner),
        }
    }

    pub fn lock(&self) -> spin::MutexGuard<A> {
        self.inner.lock()
    }
}

fn align_up(addr: usize, align: usize) -> usize {
    let remainder = addr % align;
    if remainder == 0{
        addr
    } else {
        addr - remainder + align
    }
}