pub use super::prelude::{ComponentTypeId};
use std::ptr::NonNull;
use std::{
    mem, ptr, slice, 
    alloc::{alloc, dealloc, realloc, Layout}
};
//ignore the million errors

/// Component storage for a single component type. Stores components in a contiguous block of memory.
pub struct ComponentStorage {
    /// u64 generated from type id of stored component
    pub type_id: ComponentTypeId,
    /// std::aloc::Layout of type
    pub layout: Layout,

    pub data: NonNull<u8>,
    /// Components stored in data
    pub len: usize,
    /// Allocated space
    pub capacity: usize,
}

impl ComponentStorage {
    /// Allocations grow by this rate
    const INITIAL_CAPACITY: usize = 16;

    pub fn new(type_id: ComponentTypeId, layout: Layout) -> Self {
        assert!(layout.size() > 0, "ZST components not supported");
        let capacity = Self::INITIAL_CAPACITY;
        let data = unsafe {
            let al = Layout::from_size_align(layout.size() * capacity, layout.align()).unwrap();
            NonNull::new_unchecked(alloc(al))
        };
        ComponentStorage { type_id, layout, data, len: 0, capacity }
    }
    /// push raw component data into struct
    pub unsafe fn push_raw(&mut self, src: *const u8) {
        // grows capacity by INITIAL_CAPACITY 
        if self.len == self.capacity { self.grow(); }
        // allocates one more spot to the raw data
        let dst = self.data.as_ptr().add(self.len * self.layout.size());
        // copies pointer data into raw data
        ptr::copy_nonoverlapping(src, dst, self.layout.size());
        self.len += 1;
    }

    /// # Safety: T matches, row < len
    pub unsafe fn get<T>(&self, row: usize) -> &T {
        &*(self.data.as_ptr().add(row * self.layout.size()) as *const T)
    }
    pub unsafe fn get_mut<T>(&self, row: usize) -> &mut T {
        &mut *(self.data.as_ptr().add(row * self.layout.size()) as *mut T)
    }
    pub unsafe fn ptr_at(&self, row: usize) -> *mut u8 {
        self.data.as_ptr().add(row * self.layout.size())
    }
    pub fn base_ptr(&self) -> *mut u8 { self.data.as_ptr() }
    pub fn stride(&self) -> usize { self.layout.size() }

    /// Swap-remove. Calls drop_fn on the removed element.
    /// Returns true if the last element was moved to `row`.
    pub unsafe fn swap_remove(&mut self, row: usize, drop_fn: Option<unsafe fn(*mut u8)>) -> bool {
        let size = self.layout.size();
        let last = self.len - 1;
        let removed = self.data.as_ptr().add(row * size);
        if let Some(d) = drop_fn { d(removed); }
        let swapped = row != last;
        if swapped {
            ptr::copy_nonoverlapping(self.data.as_ptr().add(last * size), removed, size);
        }
        self.len -= 1;
        swapped
    }
    /// Grows by INITIAL_CAPACIT
    pub unsafe fn grow(&mut self) {
        let new_cap = (self.capacity * 2).max(Self::INITIAL_CAPACITY);
        let old_layout = Layout::from_size_align(self.layout.size() * self.capacity, self.layout.align()).unwrap();
        let new_ptr = realloc(self.data.as_ptr(), old_layout, self.layout.size() * new_cap);
        assert!(!new_ptr.is_null());
        self.data = NonNull::new_unchecked(new_ptr);
        self.capacity = new_cap;
    }

    pub unsafe fn reserve(&mut self, new_cap: usize) {
        if new_cap > self.capacity { self.grow_to(new_cap); }
    }
    unsafe fn grow_to(&mut self, new_cap: usize) {
        let old_layout = Layout::from_size_align(
            self.layout.size() * self.capacity, self.layout.align()).unwrap();
        let new_ptr = realloc(self.data.as_ptr(), old_layout, self.layout.size() * new_cap);
        assert!(!new_ptr.is_null());
        self.data = NonNull::new_unchecked(new_ptr);
        self.capacity = new_cap;
    }
}

impl Drop for ComponentStorage {
    fn drop(&mut self) {
        if self.capacity > 0 {
            unsafe {
                let layout = Layout::from_size_align(
                    self.layout.size() * self.capacity, self.layout.align()).unwrap();
                dealloc(self.data.as_ptr(), layout);
            }
        }
    }
}