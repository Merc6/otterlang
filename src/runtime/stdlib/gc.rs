//! Garbage Collection FFI bindings

use crate::runtime::memory::get_gc;

/// Allocate memory on the heap managed by the GC
///
/// # Safety
/// This function is unsafe because it returns a raw pointer
#[unsafe(no_mangle)]
pub unsafe extern "C" fn otter_alloc(size: i64) -> *mut u8 {
    let gc = get_gc();

    // Try to allocate using the current GC strategy
    if let Some(ptr) = gc.alloc(size as usize) {
        ptr
    } else {
        // Fallback to system allocator if GC allocation fails (shouldn't happen with proper GC)
        unsafe { std::alloc::alloc(std::alloc::Layout::from_size_align(size as usize, 8).unwrap()) }
    }
}

/// Add a root object to the GC
///
/// # Safety
/// Caller must ensure `ptr` points to a valid GC-managed object.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn otter_gc_add_root(ptr: *mut u8) {
    get_gc().add_root(ptr as usize);
}

/// Remove a root object from the GC
///
/// # Safety
/// Caller must ensure `ptr` was previously registered as a root.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn otter_gc_remove_root(ptr: *mut u8) {
    get_gc().remove_root(ptr as usize);
}
