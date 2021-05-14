//! A Ledger keeps track of the double-ended stack bookkeeping.
//! 
//! ## Design
//! 
//! * zero-based forward index
//! * one-based reverse index
//! * successful allocation returns a zero-based index to the first element of the allocation
//! 
//! ## Consequences
//! 
//! * zero-sized allocations are not permitted
//! * remaining capacity = back - front
//! * front == back is full
//! * no unallocated space remains in the buffer when full
//! * all indices are unsigned integers

use std::num::NonZeroUsize;
use crate::Direction;

/// Single-threaded allocator bookkeeping
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Ledger {
    /// Forward stack free slot, zero-indexed
    pub(crate) front: usize,

    /// Reverse stack free slot, zero-indexed
    pub(crate) back: usize,

    /// The total capacity available to allocate
    pub(crate) capacity: usize,    
}

/// Errors returned by the Ledger
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum LedgerError {
    /// Requested allocation exceeds the `free` capacity
    ExceedsCapacity { free: usize },

    /// Requested deallocation exceeds the total `used`
    ExceedsAllocation { used: usize },
}

impl Ledger {
    /// Creates a new ledger
    pub fn new(capacity: usize) -> Self {
        Self {
            front: 0,
            back: capacity,
            capacity: capacity,
        }
    }

    /// The number of slots in use for the `direction` stack
    pub fn used(&self, direction: Direction) -> usize {
        match direction {
            Direction::Forward => self.front,
            Direction::Reverse => self.capacity - self.back,
        }
    }

    /// The number of slots free for use by either stack
    pub fn free(&self) -> usize {
        self.back - self.front
    }

    /// Allocate `len` slots from the `direction` stack if capacity is available.
    pub fn allocate(&mut self, direction: Direction, len: NonZeroUsize) -> Result<usize, LedgerError> {
        let len = len.get();
        let free = self.free();
        if free < len {
            Err(LedgerError::ExceedsCapacity { free })
        } else {
            match direction {
                Direction::Forward => {
                    // post-increment 0-based index
                    self.front += len;
                    Ok(self.front - len)
                },
                Direction::Reverse => {
                    // pre-decrement 1-based index
                    self.back -= len;

                    // return zero-based
                    Ok(self.back)
                }    
            }
        }
    }

    /// Deallocate `len` slots from the `direction` stack if `len` doesn't exceed total allocations.
    pub fn deallocate(&mut self, direction: Direction, len: NonZeroUsize) -> Result<(), LedgerError> {
        let len = len.get();
        let used = self.used(direction);
        if len > used {
            Err(LedgerError::ExceedsAllocation { used })
        } else {
            match direction {
                Direction::Forward => self.front -= len,
                Direction::Reverse => self.back += len,
            }
            Ok(())
        }
    }
}

#[cfg(test)]
mod test {
    use std::fmt::Debug;
    use std::num::NonZeroUsize;
    use crate::{Direction, Ledger, LedgerError};

    pub const ONE: NonZeroUsize = unsafe { NonZeroUsize::new_unchecked(1) };
    pub const TWO: NonZeroUsize = unsafe { NonZeroUsize::new_unchecked(2) };
    pub const THREE: NonZeroUsize = unsafe { NonZeroUsize::new_unchecked(3) };

    /// Asserts free and forward/reversed used
    fn assert_usage((free, forward, reverse): (usize, usize, usize), ledger: &Ledger) {
        assert_eq!(free, ledger.free());
        assert_eq!(forward, ledger.used(Direction::Forward));
        assert_eq!(reverse, ledger.used(Direction::Reverse));
    }

    /// Asserts error ExceedsCapacity
    fn assert_exceeds_capacity<T: Debug + PartialEq>(free: usize, result: Result<T, LedgerError>) {
        assert_eq!(Err(LedgerError::ExceedsCapacity { free }), result);
    }

    /// Asserts error ExceedsAllocation
    fn assert_exceeds_allocation<T: Debug + PartialEq>(used: usize, result: Result<T, LedgerError>) {
        assert_eq!(Err(LedgerError::ExceedsAllocation { used}), result);
    }

    #[test]
    fn zero_capacity() {
        let mut ledger = Ledger::new(0);

        // Initial
        assert_usage((0, 0, 0), &ledger);

        // Allocations fail
        assert_exceeds_capacity(
            0,
            ledger.allocate(Direction::Forward, ONE));

        assert_exceeds_capacity(
            0,
            ledger.allocate(Direction::Reverse, ONE));
    
        // Failed allocations don't change anything
        assert_usage((0, 0, 0), &ledger);

        // Deallocations fail
        assert_exceeds_allocation(
            0, 
            ledger.deallocate(Direction::Forward, ONE));

        assert_exceeds_allocation(
            0, 
            ledger.deallocate(Direction::Forward, ONE));

        // Failed deallocations don't change anything
        assert_usage((0, 0, 0), &ledger);
    }

    #[test]
    fn forward() {
        let mut ledger = Ledger::new(3);

        // Initial
        assert_usage((3, 0, 0), &ledger);

        // Forward 1
        assert_eq!(Ok(0), ledger.allocate(Direction::Forward, ONE));
        assert_usage((2, 1, 0), &ledger);

        // Forward 2
        assert_eq!(Ok(1), ledger.allocate(Direction::Forward, TWO));
        assert_usage((0, 3, 0), &ledger);

        // Out of space
        assert_exceeds_capacity(0, ledger.allocate(Direction::Forward, ONE));
        assert_exceeds_capacity(0, ledger.allocate(Direction::Reverse, ONE));

        // Free one
        assert_eq!(Ok(()), ledger.deallocate(Direction::Forward, ONE));
        assert_usage((1, 2, 0), &ledger);

        // Reverse 1
        assert_eq!(Ok(2), ledger.allocate(Direction::Reverse, ONE));
        assert_usage((0, 2, 1), &ledger);

        // Free too many
        assert_exceeds_allocation(2, ledger.deallocate(Direction::Forward, THREE));

        // Free all forward
        assert_eq!(Ok(()), ledger.deallocate(Direction::Forward, TWO));
        assert_usage((2, 0, 1), &ledger);
    }

    #[test]
    fn reverse() {
        let mut ledger = Ledger::new(3);

        // Initial
        assert_usage((3, 0, 0), &ledger);

        // Reverse 1
        assert_eq!(Ok(2), ledger.allocate(Direction::Reverse, ONE));
        assert_usage((2, 0, 1), &ledger);

        // Reverse 2
        assert_eq!(Ok(0), ledger.allocate(Direction::Reverse, TWO));
        assert_usage((0, 0, 3), &ledger);

        // Out of space
        assert_exceeds_capacity(0, ledger.allocate(Direction::Reverse, ONE));
        assert_exceeds_capacity(0, ledger.allocate(Direction::Forward, ONE));

        // Free one
        assert_eq!(Ok(()), ledger.deallocate(Direction::Reverse, ONE));
        assert_usage((1, 0, 2), &ledger);

        // Forward 1
        assert_eq!(Ok(0), ledger.allocate(Direction::Forward, ONE));
        assert_usage((0, 1, 2), &ledger);

        // Free too many
        assert_exceeds_allocation(2, ledger.deallocate(Direction::Reverse, THREE));

        // Free all reverse
        assert_eq!(Ok(()), ledger.deallocate(Direction::Reverse, TWO));
        assert_usage((2, 1, 0), &ledger);
    }
}