# vessel

RAII wrapper for C to Rust interop.

## Overview

vessel helps manage the lifetime of resources originating from C. It focuses on three primary areas:

* **Memory**: Owned buffers with deallocation logic.
* **Handles**: Managed file descriptors using `OwnedFd`.
* **Mapping**: Safe interfaces for `mmap` regions and volatile memory access.

## Core functions

### Blob<T, D>

A smart pointer for heap-allocated C memory.

* Implements `Deref<Target = [T]>` and `DerefMut`.
* Supports `Deallocator` implementations.
* Prevents memory leaks by automatically calling the deallocation logic on drop.

### Handle

A wrapper around `std::os::fd::OwnedFd`.

* Ensures file descriptors are closed exactly once.
* Provides helper methods for Linux-specific configuration (e.g., O_NONBLOCK).
* Integrates with `AsRawFd` for compatibility.

### View

A wrapper for `mmap` regions.

* Guarantees memory is unmapped on drop.
* Provides `read_volatile<T>` for hardware-backed or shared memory.

## Why?

If you work with Rust and C code regularly you'll find yourself writing a lot of the same boiler plate code. I got tired of doing that so I wrote this library, which helps me be a lazy piece of shit.
 
* All constructors for raw pointers or descriptors are `unsafe`. As such, the caller MUST ensure the source is valid.
* Uses `NonNull` and `addr` logic.
* All wrappers are `repr(transparent)` or thin structures with no runtime overhead.

## License

vessel is [LGPL-3.0](./LICENSE) licensed.
