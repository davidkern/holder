//! # Aspiration
//! 
//! Holder provides [`Hold<T>`], a thread-safe double-ended stack allocator with
//! marker-based deallocation and scopes for structured memory management.
//! 
//! # Status: Under Development
//! 
//! ## Development Plan
//! 
//! 1. in progress &mdash; single-threaded proof-of-concept
//! 2. not started &mdash; RwLock-based concurrency
//! 3. not started &mdash; Lock Free concurrency
//! 
//! # Safety
//! 
//! This crate does use unsafe code but only in cases where no other option is available. All usages
//! of unsafe follow well documented patterns for safe usage. PRs are welcome to remove usages of
//! unsafe where I have overlooked a safe alternative.
//! 
//! Unsafe code usage:
//! 
//! * `UnsafeCell` is used to allow interior mutability and allow concurrent mutable references to
//!   disjoint allocations;
//! * `MaybeUninit` is used to allow allocation of `?Copy` types
//! 
//! # Acknowledgements
//! 
//! * [scratchpad] - the primary inspiration for this crate's functionality. The primary difference
//!    with this crate is [`Hold<T>`] keeps a slice of a single-type, while scratchpad can allocate
//!    multiple types by internally keeping a slice of bytes. This crate's approach requires less
//!    unsafe code and enforces a uniform stride of the backing buffer.
//! * [crossbeam] - their source was used for guidance in implementating the storage for Hold<T>
//!    since it seems likely they know what they are doing!
//! * [1024cores Lockfree Algorithms] - I also referred to this guide referenced from [crossbeam]. 
//! 
//! [crossbeam]: https://github.com/crossbeam-rs/crossbeam
//! [scratchpad]: https://github.com/okready/scratchpad
//! [`ArrayQueue`]: https://docs.rs/crossbeam-queue/0.3.1/src/crossbeam_queue/array_queue.rs.html
//! [`Hold<T>`]: crate::Hold
//! [1024cores Lockfree Algorithms]: https://www.1024cores.net/home/lock-free-algorithms/

mod ledger;
mod direction;
mod hold;

pub use direction::Direction;
//pub use hold::Hold;
pub use ledger::{Ledger, LedgerError};
