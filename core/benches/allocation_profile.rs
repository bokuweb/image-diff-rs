use std::alloc::{GlobalAlloc, Layout, System};
use std::sync::atomic::{AtomicUsize, Ordering};

use image_diff_rs::{diff_count, diff_rgba, DiffOption};

const ACTUAL: &[u8] = include_bytes!("../../fixtures/sample0.webp");
const EXPECTED: &[u8] = include_bytes!("../../fixtures/sample1.webp");

struct CountingAllocator;

static ALLOCATED: AtomicUsize = AtomicUsize::new(0);

unsafe impl GlobalAlloc for CountingAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        // SAFETY: `layout` is forwarded unchanged to the system allocator.
        let pointer = unsafe { System.alloc(layout) };
        if !pointer.is_null() {
            ALLOCATED.fetch_add(layout.size(), Ordering::Relaxed);
        }
        pointer
    }

    unsafe fn dealloc(&self, pointer: *mut u8, layout: Layout) {
        // SAFETY: `pointer` and `layout` came from the system allocator.
        unsafe { System.dealloc(pointer, layout) }
    }

    unsafe fn realloc(&self, pointer: *mut u8, layout: Layout, new_size: usize) -> *mut u8 {
        // SAFETY: `pointer` and `layout` came from the system allocator, and
        // `new_size` is forwarded unchanged.
        let new_pointer = unsafe { System.realloc(pointer, layout, new_size) };
        if !new_pointer.is_null() {
            ALLOCATED.fetch_add(new_size, Ordering::Relaxed);
        }
        new_pointer
    }
}

#[global_allocator]
static GLOBAL: CountingAllocator = CountingAllocator;

fn allocated_by<T>(operation: impl FnOnce() -> T) -> (T, usize) {
    let before = ALLOCATED.load(Ordering::Relaxed);
    let result = operation();
    let allocated = ALLOCATED.load(Ordering::Relaxed) - before;
    (result, allocated)
}

fn main() {
    let option = DiffOption {
        threshold: Some(0.01),
        include_anti_alias: Some(true),
        ..Default::default()
    };

    // Warm up lazy process-wide initialization before measuring either path.
    assert_eq!(diff_count(ACTUAL, EXPECTED, &option).unwrap(), 3454);

    let (rgba, rgba_allocated) = allocated_by(|| diff_rgba(ACTUAL, EXPECTED, &option).unwrap());
    let (count, count_allocated) = allocated_by(|| diff_count(ACTUAL, EXPECTED, &option).unwrap());

    assert_eq!(count, rgba.diff_count);
    println!("diff_rgba allocated: {rgba_allocated} bytes");
    println!("diff_count allocated: {count_allocated} bytes");
    println!(
        "count-only reduction: {} bytes",
        rgba_allocated - count_allocated
    );
}
