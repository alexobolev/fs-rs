use std::alloc::{GlobalAlloc, System, Layout};

/// A custom system allocator wrapper that logs all allocations
/// and deallocations to console / stderr.
struct LoggingAllocator;

unsafe impl GlobalAlloc for LoggingAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        eprintln!("[ALLCTR] Allocating {} byte(s) [aligned to {}]:", layout.size(), layout.align());
        let allocated_ptr = System.alloc(layout);
        eprintln!("[ALLCTR]   ... allocated = {:p}", allocated_ptr);
        allocated_ptr
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        eprintln!("[ALLCTR] Deallocating {:p} with {} byte(s) [aligned to {}]",
            ptr, layout.size(), layout.align());
        System.dealloc(ptr, layout);
        eprintln!("[ALLCTR]   ... done!");
    }
}

#[global_allocator]
static GLOBAL: LoggingAllocator = LoggingAllocator;
