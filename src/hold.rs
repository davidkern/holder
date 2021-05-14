use std::marker::PhantomData;
use std::mem::{self, MaybeUninit};
use std::cell::UnsafeCell;

use crate::{Ledger, LedgerError, A};

type Mark = (usize, usize);

trait IHold {
    fn mark(&mut self) -> Mark;
    unsafe fn release(&mut self, marker: Mark);
}

impl<T> IHold for Hold<T> {
    fn mark(&mut self) -> Mark {
        (self.ledger.front, self.ledger.back)
    }

    unsafe fn release(&mut self, marker: Mark) {
        self.ledger.front = marker.0;
        self.ledger.back = marker.1;
    }
}

/// A double-ended stack allocator
pub struct Hold<T> {
    ledger: Ledger,
    buffer: *mut Slot<T>,
    _marker: PhantomData<T>,
}


struct Slot<T> {
    slot: UnsafeCell<MaybeUninit<T>>,
}

impl<T> Slot<T> {
    fn new() -> Self {
        Self {
            slot: UnsafeCell::new(MaybeUninit::uninit()),
        }
    }
}

impl<T> Hold<T> {
    pub fn new(capacity: usize) -> Self {
        let buffer = {
            let mut boxed: Box<[Slot<T>]> = (0..capacity)
                .map(|i| {
                    Slot {
                        slot: UnsafeCell::new(MaybeUninit::uninit()),
                    }
                })
                .collect();
            let ptr = boxed.as_mut_ptr();
            mem::forget(boxed);
            ptr
        };

        Self {
            ledger: Ledger::new(capacity),
            buffer,
            _marker: PhantomData,
        }
    }

    // pub fn mark<'id, F, R>(&'id mut self, func: F) -> R
    // where
    //     F: FnOnce(A) -> R
    // {
    //     let a = A {
    //         holder: self,
    //     };
    //     (func)(a)
    // }
}
