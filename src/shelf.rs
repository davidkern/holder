use std::cell::UnsafeCell;
use std::mem::MaybeUninit;

pub trait Shelving {
    type Item;

    /// Gets a immutable reference to a shelved item.
    /// 
    /// SAFETY:
    /// Caller must ensure that lifetime 'a does not outlive the item owner.
    /// Caller must not loan an item before it is initialized.
    /// Caller must not loan an item mutably if any immutable loans exist.
    /// Caller must ensure the id is in bounds.
    unsafe fn lend<'a>(&self, id: usize) -> &'a Self::Item;

    /// Gets a mutable reference to a shelved item.
    /// 
    /// SAFETY:
    /// Caller must ensure that lifetime 'a does not outlive the item owner.
    /// Caller must not loan an item before it is initialized.
    /// Caller must not loan an item mutably if any immutable loans exist.
    /// Caller must ensure the id is in bounds.
    unsafe fn lend_mut<'a>(&self, id: usize) -> &'a mut Self::Item;

    /// Initialize the item identified by id.
    /// 
    /// Returns a mutable reference to the newly shelved item.
    /// 
    /// SAFETY:
    /// Caller may call this no more than once per id.
    /// Caller must ensure that lifetime 'a does not outlive the item owner.
    /// Caller must ensure the id is in bounds.
    unsafe fn shelve<'a>(&self, id: usize, value: Self::Item) -> &'a mut Self::Item;
}

/// Owner uses interior mutability so it can be static or stack based
/// Owner contains only the data array, so user-data structures are dense struct-of-arrays
#[repr(transparent)]
pub struct Shelf<T, const S: usize> {
    data: UnsafeCell<[MaybeUninit<T>; S]>,
}

impl<T, const S: usize> Shelving for Shelf<T, S> {
    type Item = T;

    unsafe fn lend<'a>(&self, id: usize) -> &'a T {
        let data = &*self.data.get();
        &*data[id].as_ptr()
    }

    unsafe fn lend_mut<'a>(&self, id: usize) -> &'a mut T {
        let data = &mut *self.data.get();
        &mut *data[id].as_mut_ptr()
    }

    unsafe fn shelve<'a>(&self, id: usize, value: Self::Item) -> &'a mut Self::Item {
        let data = &mut *self.data.get();
        let ptr = data[id].as_mut_ptr();
        *ptr = value;
        &mut *ptr
    }

}
