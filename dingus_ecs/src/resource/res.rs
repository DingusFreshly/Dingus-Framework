use std::ops::{Deref, DerefMut};
/// A resources is a globally accessible value. Resources are stored in the world and can be accessed by systems
/// Res gets a read only reference to a resources when put as a system parameter
pub struct Res<'w, T: 'static>    { pub value: &'w T }
/// A resources is a globally accessible value. Resources are stored in the world and can be accessed by systems
/// Res gets a mutable reference to a resources when put as a system parameter
pub struct ResMut<'w, T: 'static> { pub value: &'w mut T }

impl<'w, T: 'static> Deref    for Res<'w, T>    { type Target = T; fn deref(&self) -> &T { self.value } }
impl<'w, T: 'static> Deref    for ResMut<'w, T> { type Target = T; fn deref(&self) -> &T { self.value } }
impl<'w, T: 'static> DerefMut for ResMut<'w, T> { fn deref_mut(&mut self) -> &mut T { self.value } }