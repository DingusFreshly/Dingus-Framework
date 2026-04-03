use std::any::TypeId;
use hashed_type_def::{HashedTypeDef, HashedTypeUuid};
use super::prelude::{ComponentTypeId};

/// Computes a stable ComponentTypeId from std::any::TypeId.
/// TypeId is not a u64 internally, so we hash it.
pub fn type_id_to_component_id<T: 'static>() -> ComponentTypeId {
    use std::hash::{Hash, Hasher};
    // FxHasher-style manual hash for no-dep compile-time stability.
    struct FxHasher(u64);
    impl Hasher for FxHasher {
        fn finish(&self) -> u64 { self.0 }
        fn write(&mut self, bytes: &[u8]) {
            for &b in bytes {
                self.0 = self.0.wrapping_mul(0x517cc1b727220a95).wrapping_add(b as u64);
            }
        }
    }
    let mut h = FxHasher(0);
    TypeId::of::<T>().hash(&mut h);
    h.finish()
}

/// Produces the drop function for T if T is not Copy/trivially droppable.
/// Returns None for types where std::mem::needs_drop is false.
pub fn make_drop_fn<T>() -> Option<unsafe fn(*mut u8)> {
    if std::mem::needs_drop::<T>() {
        Some(|ptr: *mut u8| unsafe {
            std::ptr::drop_in_place(ptr as *mut T);
        })
    } else {
        None
    }
}

//--compile time string hash
pub const fn const_fnv1a(s: &str) -> u64 {
    let bytes = s.as_bytes();
    let mut hash: u64 = 0xcbf29ce484222325;
    let mut i = 0;
    while i < bytes.len() {
        hash ^= bytes[i] as u64;
        hash = hash.wrapping_mul(0x00000100000001b3);
        i += 1;
    }
    hash
}