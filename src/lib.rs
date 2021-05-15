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

mod catalog;
mod library;
mod shelf;

pub use catalog::{Cataloging, DualStack};
pub use shelf::{Shelf, Shelving};
