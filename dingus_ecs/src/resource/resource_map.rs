use std::ptr::NonNull;
use std::alloc::{alloc, dealloc, Layout};
use std::collections::HashMap;
use crate::component::prelude::{ComponentTypeId};
use crate::component::utils::{type_id_to_component_id, make_drop_fn};
use crate::resource::{ResourceId, ResourceTrait};

/// A type erased heap allocation of a resource value plus its drop function.
struct ResourceEntry {
    data: NonNull<u8>,
    layout: Layout,
    drop_fn: Option<unsafe fn(*mut u8)>,//
}

impl Drop for ResourceEntry {
    fn drop(&mut self) {
        unsafe {
            if let Some(drop) = self.drop_fn {
                drop(self.data.as_ptr());
            }
            dealloc(self.data.as_ptr(), self.layout);
        }
    }
}

/// Storage for multiple unique resources
pub struct ResourceMap {
    map: HashMap<ResourceId, ResourceEntry>,
}

impl ResourceMap {//TODO! proper error handling here instead of panicking
    pub fn new() -> Self {
        ResourceMap { map: HashMap::new() }
    }

    /// Insert a resource, replacing any existing value of the same type.
    pub fn insert<T: ResourceTrait + 'static>(&mut self, value: T) {//Send + Sync + 
        let type_id = T::RESOURCE_INDEX;
        let layout = Layout::new::<T>();
        let data = unsafe {
            let ptr = alloc(layout);
            assert!(!ptr.is_null());
            (ptr as *mut T).write(value);
            NonNull::new_unchecked(ptr)
        };
        let entry = ResourceEntry { data, layout, drop_fn: make_drop_fn::<T>() };
        if let Some(old) = self.map.insert(type_id, entry) {
            drop(old); // ResourceEntry::drop handles cleanup
        }
    }

    /// Get an immutable reference to a resource.
    pub fn get<T:ResourceTrait+ 'static>(&self) -> Option<&T> {
        let type_id = T::RESOURCE_INDEX;
        self.map.get(&type_id).map(|e| unsafe { &*(e.data.as_ptr() as *const T) })
    }

    /// Get a mutable reference to a resource.
    pub fn get_mut<T:ResourceTrait+ 'static>(&self) -> Option<&mut T> {
        let type_id = T::RESOURCE_INDEX;
        self.map.get(&type_id).map(|e| unsafe { &mut *(e.data.as_ptr() as *mut T) })
    }

    /// Remove and return a resource.
    pub fn remove<T: ResourceTrait+'static>(&mut self) -> Option<T> {
        let type_id = T::RESOURCE_INDEX;
        self.map.remove(&type_id).map(|e| unsafe {
            let value = (e.data.as_ptr() as *const T).read();
            // Skip drop_fn bc were taking ownership. Dealloc manually
            dealloc(e.data.as_ptr(), e.layout);
            std::mem::forget(e); // prevent ResourceEntry::drop from running
            value
        })
    }

    pub fn contains<T: ResourceTrait + 'static>(&self) -> bool {
        self.map.contains_key(&T::RESOURCE_INDEX)
    }
}