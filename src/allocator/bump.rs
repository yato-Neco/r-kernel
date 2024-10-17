use core::{
    alloc::{GlobalAlloc, Layout},
    cell::{Cell, RefCell},
};

#[global_allocator]
static mut ALLOCATOR: BumpAllocator = BumpAllocator::new();

const ARENA_SIZE: usize = 32 * 1024 * 1024; // 32MB

pub struct BumpAllocator {
    arena: RefCell<[u8; ARENA_SIZE]>,
    next: Cell<usize>,
}

impl BumpAllocator {
    #[no_mangle]
    const fn new() -> Self {
        Self {
            arena: RefCell::new([0; ARENA_SIZE]),
            next: Cell::new(0),
        }
    }
}

unsafe impl GlobalAlloc for BumpAllocator {
    #[no_mangle]
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let next = self.next.get();

        let size = layout.size();
        let align = layout.align();

        let alloc_start = aligned_addr(next, align);

        let mut arena = self.arena.borrow_mut();
        let alloc_end = alloc_start + size;

        if alloc_end > arena.len() {
            panic!("out of memory");
        }

        self.next.set(alloc_end);

        let ptr = arena.as_mut_ptr();

        return ptr.add(alloc_start);
    }

    #[no_mangle]
    // BumpAllocatorはメモリを開放しない
    unsafe fn dealloc(&self, _ptr: *mut u8, _layout: Layout) {}
}

#[no_mangle]
fn aligned_addr(addr: usize, align: usize) -> usize {
    if addr % align == 0 {
        addr
    } else {
        addr + align - (addr % align)
    }
}
