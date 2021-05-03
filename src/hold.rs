use std::marker::PhantomData;
use std::mem::{self, MaybeUninit};
use std::cell::UnsafeCell;

use crate::{Ledger, LedgerError};

// /// A double-ended stack allocator
// pub struct Hold<T> {
//     ledger: Ledger,
//     buffer: *mut Slot<T>,
//     _marker: PhantomData<T>,
// }

// struct Slot<T> {
//     value: UnsafeCell<MaybeUninit<T>>,
// }

// impl<T> Hold<T> {
//     pub fn new(capacity: usize) -> Self {
//         let buffer = {
//             let mut boxed: Box<[Slot<T>]> = (0..capacity)
//                 .map(|i| {
//                     Slot {
//                         value: UnsafeCell::new(MaybeUninit::uninit()),
//                     }
//                 })
//                 .collect();
//             let ptr = boxed.as_mut_ptr();
//             mem::forget(boxed);
//             ptr
//         };

//         Self {
//             ledger: Ledger::new(capacity),
//             buffer,
//             _marker: PhantomData,
//         }
//     }
// }
