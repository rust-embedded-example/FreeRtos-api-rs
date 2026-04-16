# freertos-api-rs

Rust FFI bindings and safe wrappers for the [FreeRTOS](https://www.freertos.org/) real-time operating system kernel.

## Overview

This crate provides a complete three-layer architecture for calling FreeRTOS APIs from Rust:

```
┌─────────────────────────────────────┐
│        Your Rust Application        │
├─────────────────────────────────────┤
│  Safe Rust Wrappers (RAII types)    │  ← Queue<T>, Mutex, Timer, EventGroup, ...
├─────────────────────────────────────┤
│  Rust FFI Declarations              │  ← unsafe extern "C" { ... }
├─────────────────────────────────────┤
│  C Wrapper Layer (freertos-api-rust.c)│  ← freertos_rs_*() functions
├─────────────────────────────────────┤
│  FreeRTOS C Kernel                  │  ← xTaskCreate(), xQueueSend(), ...
└─────────────────────────────────────┘
```

## Features

- **Complete API coverage** — All FreeRTOS public APIs wrapped (tasks, queues, semaphores, mutexes, timers, event groups, stream/message buffers, atomic operations, lists, co-routines)
- **Safe RAII wrappers** — `Queue<T>`, `BinarySemaphore`, `CountingSemaphore`, `Mutex`, `RecursiveMutex`, `Timer`, `EventGroup`, `StreamBuffer`, `MessageBuffer`, `CriticalSection`
- **`no_std` compatible** — Designed for bare-metal embedded targets
- **`GlobalAlloc` support** — `FreeRtosAllocator` implements Rust's global allocator trait on top of `pvPortMalloc`/`vPortFree`
- **Comprehensive documentation** — Full rustdoc for all public APIs

## Module Map

| Module           | FreeRTOS Header         | Safe Wrapper Types                                    |
|------------------|-------------------------|-------------------------------------------------------|
| `base`           | —                       | Core types, enums, constants                          |
| `task`           | `task.h`                | `CriticalSection`, `CriticalSectionFromIsr`           |
| `queue`          | `queue.h`               | `Queue<T>`                                            |
| `semphr`         | `semphr.h`              | `BinarySemaphore`, `CountingSemaphore`, `Mutex`, `RecursiveMutex` |
| `timers`         | `timers.h`              | `Timer`                                               |
| `event_groups`   | `event_groups.h`        | `EventGroup`                                          |
| `stream_buffer`  | `stream_buffer.h`       | `StreamBuffer`                                        |
| `message_buffer` | `message_buffer.h`      | `MessageBuffer`                                       |
| `portable`       | `portable.h`            | `FreeRtosAllocator` (implements `GlobalAlloc`)        |
| `projdefs`       | `projdefs.h`            | Constants (`pdTRUE`, `pdPASS`, `PORT_MAX_DELAY`, etc.)|
| `atomic`         | `atomic.h`              | Atomic operation FFI bindings                         |
| `list`           | `list.h`                | Linked list FFI bindings                              |
| `croutine`       | `croutine.h` (deprecated)| Co-routine FFI bindings                              |

## Quick Start

### 1. Build the Rust Crate

```bash
cargo build --release --target thumbv7em-none-eabihf
```

### 2. Copy Build Artifacts

Copy these files to your C FreeRTOS project:
- `target/thumbv7em-none-eabihf/release/deps/freertos_api_rs-*.o`
- `src/freertos-api-rust.c`

### 3. Use in Your C Project

```c
#include "FreeRTOS.h"
#include "task.h"

// The C wrapper file provides freertos_rs_*() functions
// Your Rust code provides additional exported functions

int main(void) {
    // Initialize hardware, create tasks using FreeRTOS or Rust wrappers
    vTaskStartScheduler();
    while(1);
}
```

### 4. Use Safe Wrappers in Rust

```rust
use freertos_api_rs::queue::Queue;
use freertos_api_rs::semphr::Mutex;
use freertos_api_rs::task::CriticalSection;

// Type-safe queue
let queue: Queue<u32> = Queue::new(10).unwrap();
queue.send(&42, 100).unwrap();

// Mutex with RAII
let mut mutex = Mutex::new().unwrap();
if mutex.lock(100) {
    // Critical section
    mutex.unlock();
}

// RAII critical section
{
    let _cs = CriticalSection::enter();
    // Interrupts disabled here
} // Interrupts re-enabled on drop
```

## Configuration

This crate mirrors FreeRTOS configuration through conditional compilation in the C wrapper layer:

| FreeRTOS Config                     | Enables                       |
|-------------------------------------|-------------------------------|
| `configSUPPORT_STATIC_ALLOCATION`   | Static allocation variants    |
| `configSUPPORT_DYNAMIC_ALLOCATION`  | Dynamic allocation variants   |
| `configUSE_MUTEXES`                 | Mutex APIs                    |
| `configUSE_RECURSIVE_MUTEXES`       | Recursive mutex APIs          |
| `configUSE_COUNTING_SEMAPHORES`     | Counting semaphore APIs       |
| `configUSE_TIMERS`                  | Software timer APIs           |
| `configUSE_QUEUE_SETS`              | Queue set APIs                |
| `configUSE_TRACE_FACILITY`          | Debug/tracing APIs            |

## License

MIT
