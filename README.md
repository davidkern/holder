# Holder

Holder provides [`Hold<T>`], a thread-safe double-ended stack allocator with
phased deallocation for structured memory management.

# Status: Under Development

This crate is in development and is not ready for general usage.

## 0.1 Development Plan

1. in progress &mdash; single-threaded proof-of-concept
2. not started &mdash; introduce allocation/deallocation phases
2. not started &mdash; RwLock-based concurrency
3. not started &mdash; Lock Free concurrency

# Safety

This crate does use unsafe code but only in cases where no other option is available. All usages
of unsafe follow well documented patterns and are isolated from safe code. PRs are welcome to
remove usages of unsafe where a safe alternative has been overlooked.

Unsafe code usage:

* `UnsafeCell` is used to allow interior mutability and concurrent mutable references to
  disjoint allocations;
* `MaybeUninit` is used to allow allocation of types which do not implement `Copy`.

# Contributing

Contributions of any kind are very welcome!

_Thank you for your contribution!_

There are many ways to contribute:

* Use the crate in your projects and share your experience in chat or in a Github issue.
* Improve documentation - I try to write good prose but sometimes I write word salad or no words at all!
* Replace uses of unsafe with alternative safe code, or identify potential UB.
* Fix bugs, though I hope we don't have any. :laughing:
* Suggest alternatives, different APIs, other crates.
* Share the crate with others.
* Or something else entirely!

Please introduce yourself in the
[#holder](https://davidkern.zulipchat.com/#narrow/stream/287264-holder) stream if you'd like some
help, to discuss an idea, or just wish to chat about the project.

For PRs, please add an entry in [CHANGELOG.md] (see [Keep a Changelog] for the pattern).
If this is your first contribution, also add your name and a one-line note of your own choosing
to [CONTRIBUTORS.md].

# Acknowledgements

* [scratchpad] - the primary inspiration for this crate's functionality. The primary difference
   with this crate is [`Hold<T>`] keeps a slice of a single-type, while scratchpad can allocate
   multiple types by internally keeping a slice of bytes. This crate's approach requires less
   unsafe code and enforces a uniform stride of the backing buffer.
* [crossbeam] - their source was used for guidance in implementating the storage for Hold<T>
   since it seems likely they know what they are doing!
* [1024cores Lockfree Algorithms] - I also referred to this guide referenced from [crossbeam]. 

# License

Copyrights in the Holder project are retained by their contributors. No
copyright assignment is required to contribute to the Holder project.

The Holder project is licensed under either of

 * Apache License, Version 2.0, (see ./LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license (see ./LICENSE-MIT or http://opensource.org/licenses/MIT)

at your option.

Unless you explicitly state otherwise, any contribution intentionally
submitted for inclusion in the work by you, as defined in the Apache-2.0
license, shall be dual licensed as above, without any additional terms or
conditions.


[crossbeam]: https://github.com/crossbeam-rs/crossbeam
[scratchpad]: https://github.com/okready/scratchpad
[`ArrayQueue`]: https://docs.rs/crossbeam-queue/0.3.1/src/crossbeam_queue/array_queue.rs.html
[1024cores Lockfree Algorithms]: https://www.1024cores.net/home/lock-free-algorithms/
[CHANGELOG.md]: Changelog.md
[Keep a Changelog]: https://keepachangelog.com/en/1.0.0/
