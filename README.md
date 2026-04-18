# freertos-api-rs

[![MIT licensed](https://img.shields.io/badge/license-MIT-blue.svg)](./LICENSE)

Rust FFI bindings and safe RAII wrappers for the [FreeRTOS](https://www.freertos.org/) real-time operating system kernel.

`#![no_std]` — zero external dependencies — Rust Edition 2024

---

## Architecture

```
┌──────────────────────────────────────────────┐
│             Your Rust Application            │
├──────────────────────────────────────────────┤
│   Safe RAII Wrappers                         │
│   Queue<T>, Mutex, Task, Timer, EventGroup,  │
│   StreamBuffer, MessageBuffer, ...           │
├──────────────────────────────────────────────┤
│   Raw FFI Declarations (unsafe extern "C")   │
├──────────────────────────────────────────────┤
│   C Wrapper Layer (freertos-api-rust.c)      │
│   Conditional compilation via FreeRTOS       │
│   config macros (configUSE_*, INCLUDE_*)     │
├──────────────────────────────────────────────┤
│   FreeRTOS C Kernel                          │
└──────────────────────────────────────────────┘
```

The C wrapper layer (~4000 lines) isolates all conditional compilation behind a uniform `freertos_rs_*()` function interface. The Rust FFI layer declares these as `unsafe extern "C"` functions. Safe Rust wrappers then provide RAII semantics, type safety, and ergonomic error handling on top.

---

## Features

- **Complete API coverage** — Tasks, queues, semaphores, mutexes, recursive mutexes, timers, event groups, stream buffers, message buffers, atomic operations, linked lists, memory management
- **Safe RAII wrappers** — Every kernel object type has a Rust wrapper that releases the resource on `Drop`
- **Type-safe generics** — `Queue<T>` enforces `T: Send + Copy` at compile time
- **ISR-safe variants** — Every blocking API has a `_from_isr()` counterpart returning `bool` (higher-priority task woken flag)
- **Static allocation** — `new_static()` constructors accept pre-allocated memory for kernel objects, no heap required
- **SMP / multi-core** — Core affinity APIs, per-core task handles (`configNUMBER_OF_CORES`)
- **MPU support** — Restricted task creation with MPU region configuration
- **`no_std`** — Bare-metal compatible, no `std` dependency
- **`GlobalAlloc` support** — `FreeRtosAllocator` wraps `pvPortMalloc`/`vPortFree` to enable `Vec`, `Box`, `String` on the FreeRTOS heap
- **Compile-time verification** — Static assertions verify type sizes, alignments, enum discriminants, and struct layouts match the C ABI
- **Zero dependencies** — No external crates, `Cargo.lock` contains only this package

---

## Module Map

| Module | Kernel Header | Safe Wrapper Types |
|---|---|---|
| `base` | — | Core FFI types, enums (`FreeRtosError`, `FreeRtosTaskState`, `FreeRtosNotifyAction`), handle aliases, constants |
| `task` | `task.h` | `Task`, `CriticalSection`, `CriticalSectionFromIsr`, `PreemptionGuard`, `TimeoutState` |
| `queue` | `queue.h` | `Queue<T>`, `QueueSet` |
| `semphr` | `semphr.h` | `BinarySemaphore`, `CountingSemaphore`, `Mutex`, `RecursiveMutex` |
| `timers` | `timers.h` | `Timer` |
| `event_groups` | `event_groups.h` | `EventGroup` |
| `stream_buffer` | `stream_buffer.h` | `StreamBuffer`, `BatchingBuffer` |
| `message_buffer` | `message_buffer.h` | `MessageBuffer` |
| `portable` | `portable.h` | `FreeRtosAllocator` (`GlobalAlloc`), `FreeRtosHeapRegion` |
| `projdefs` | `projdefs.h` | Time conversion (`ms_to_ticks`, `ticks_to_ms`), errno, scheduler states |
| `atomic` | — | `FreeRtosAtomicU32` (heap-backed atomic via FreeRTOS atomics) |
| `list` | `list.h` | Safe inline helpers for linked list operations |

All commonly used types from `base` are re-exported at the crate root.

---

## Quick Start

### 1. Build

```bash
# Default target: ARM Cortex-M7 (thumbv7em-none-eabihf)
cargo build --release
```

The `.cargo/config.toml` preconfigures the target, CPU, linker (`rust-lld`), and optimization level.

### 2. Integrate with FreeRTOS C Project

Copy these files into your C project:

```bash
cp target/thumbv7em-none-eabihf/release/deps/freertos_api_rs-*.o  <your_project>/
cp src/freertos-api-rust.c                                        <your_project>/
```

Compile `freertos-api-rust.c` alongside your FreeRTOS kernel sources using your cross toolchain. Link the resulting `.o` files together with the Rust static library.

### 3. Use Safe Wrappers

```rust
#![no_std]
extern crate alloc;

use freertos_api_rs::prelude::*;
use freertos_api_rs::queue::Queue;
use freertos_api_rs::semphr::Mutex;
use freertos_api_rs::task::{self, Task, CriticalSection};

// Set up the global allocator
#[global_allocator]
static ALLOC: FreeRtosAllocator = FreeRtosAllocator;

#[export_name = "main"]
pub extern "C" fn main() {
    // Create a type-safe queue
    let queue: Queue<u32> = Queue::new(10).unwrap();
    queue.send(&42, portMAX_DELAY).unwrap();

    // Spawn a task
    let _task = Task::spawn(
        "worker",
        128,
        1,
        Some(|_| {
            loop {
                let val = queue.receive(portMAX_DELAY).unwrap();
                // process val
            }
        }),
        None,
    ).unwrap();

    // RAII mutex
    let mutex = Mutex::new().unwrap();
    if mutex.lock(100) {
        // critical section
        mutex.unlock();
    }

    // RAII critical section (disables interrupts)
    {
        let _cs = CriticalSection::enter();
        // interrupts disabled
    } // interrupts re-enabled on drop

    task::start_scheduler();
}
```

---

## API Overview

### Task Management (`task`)

```rust
// Spawn a task — handle owns the task, Drop calls vTaskDelete
let task = Task::spawn("name", 128, tskIDLE_PRIORITY + 1, Some(my_fn), None)?;

// Spawn with static storage (no heap)
let task = Task::spawn_static("name", 128, 1, Some(my_fn), None, &mut static_storage)?;

// SMP: pin task to core 0
let task = task::spawn_affinity("name", 128, 1, 0, Some(my_fn), None)?;

// Delay and tick queries
task::delay(100);                              // delay 100 ticks
task::delay_until(&mut last_wake, 100);        // periodic delay
let ticks = task::get_tick_count();

// Task notifications (lightweight IPC)
task.notify(handle, 0x01, FreeRtosNotifyAction::SetBits);
let (value, _) = task::notify_wait(0, 0xFFFF, 100)?;

// RAII guards
let _cs = CriticalSection::enter();            // taskENTER_CRITICAL
let _cs_isr = CriticalSectionFromIsr::enter(); // taskENTER_CRITICAL_FROM_ISR
let _guard = PreemptionGuard::enter();         // disable preemption
```

### Queues (`queue`)

```rust
// Type-safe queue — T must be Send + Copy
let queue: Queue<MyStruct> = Queue::new(16)?;

queue.send(&data, portMAX_DELAY)?;            // blocking send
queue.send_from_isr(&data)?;                   // ISR variant returns bool

let received = queue.receive(100)?;            // blocking receive
let peeked = queue.peek(100)?;                 // peek without removing

let count = queue.messages_waiting();

// Static allocation
let queue = Queue::new_static(16, &mut storage)?;

// Queue sets — wait on multiple queues/semaphores simultaneously
let qset = QueueSet::new(3)?;
qset.add(queue.handle())?;
let handle = qset.select(100)?;
```

### Semaphores & Mutexes (`semphr`)

```rust
// Binary semaphore
let sem = BinarySemaphore::new()?;
sem.give();
sem.take(portMAX_DELAY);

// Counting semaphore
let sem = CountingSemaphore::new(10, 0)?;
sem.give_from_isr();

// Mutex with ownership tracking
let mutex = Mutex::new()?;
assert!(!mutex.is_owned());
mutex.lock(100);
assert!(mutex.is_owned());
mutex.unlock(); // Drop also attempts unlock + delete

// Recursive mutex — lock/unwind count tracked automatically
let rmutex = RecursiveMutex::new()?;
rmutex.lock(100);
rmutex.lock(100); // recursive lock OK
rmutex.unlock();
rmutex.unlock();
// Drop unwinds all remaining lock counts
```

### Software Timers (`timers`)

```rust
// Timer requires unsafe callback (FreeRTOS callback has no user data context)
let timer = unsafe {
    Timer::new("my_timer", 100, true, || {
        // timer callback (runs in timer daemon context)
    })?
};

timer.start(0)?;                  // start the timer
timer.stop(0)?;                   // stop it
timer.change_period(200, 0)?;     // change period to 200 ticks
let active = timer.is_active();
```

### Event Groups (`event_groups`)

```rust
let eg = EventGroup::new()?;

// Set / clear / wait on individual bits
eg.set_bits(0x03)?;
let bits = eg.wait_bits(0x03, true, true, portMAX_DELAY)?;

// Sync — set bits and wait for all parties
let bits = eg.sync(0x01, 0x03, portMAX_DELAY)?;

// ISR-safe variants
eg.set_bits_from_isr(0x01)?;
eg.clear_bits_from_isr(0x02)?;
```

### Stream & Message Buffers

```rust
// Stream buffer — byte stream, no framing
let sb = StreamBuffer::new(256, 1)?;
let sent = sb.send(&data, portMAX_DELAY)?;
let received = sb.receive(&mut buf, 100)?;

// Batching buffer — accumulates data until trigger level reached
let bb = BatchingBuffer::new(256, 64)?;

// Message buffer — preserves message boundaries
let mb = MessageBuffer::new(256)?;
mb.send(&message, portMAX_DELAY)?;
let len = mb.receive(&mut buf, 100)?;
let next_len = mb.next_length_bytes(); // peek at next message size
```

### Memory Management (`portable`)

```rust
#[global_allocator]
static ALLOC: FreeRtosAllocator = FreeRtosAllocator;

// Query heap state
let free = portable::get_free_heap_size();
let min = portable::get_minimum_ever_free_heap_size();
let stats = portable::get_heap_stats();

// Define heap_5 regions (multiple memory areas)
let regions = [
    FreeRtosHeapRegion::new(0x20000000, 0x10000),
    FreeRtosHeapRegion::new(0x60000000, 0x80000),
];
portable::define_heap_regions(&regions);
```

### Atomic Operations (`atomic`)

```rust
// Heap-backed atomic u32 using FreeRTOS atomic APIs
let atomic = FreeRtosAtomicU32::new(42)?;
let old = atomic.fetch_add(1);
let success = atomic.compare_and_swap(43, 100);

// Swap two pointers atomically
atomic::swap_pointers(&mut ptr_a, &mut ptr_b);
```

### Time Conversion (`projdefs`)

```rust
// Compile-time conversion (const fn, assumes configTICK_RATE_HZ == 1000)
const TIMEOUT: FreeRtosTickType = projdefs::ms_to_ticks(100); // 100

// Runtime conversion using the actual tick period
let ticks = projdefs::ms_to_ticks_runtime(100);
let ms = projdefs::ticks_to_ms_runtime(ticks);
let period = projdefs::get_tick_period_ms();
```

---

## Error Handling

Fallible operations return `Result<T, FreeRtosError>`:

```rust
#[non_exhaustive]
pub enum FreeRtosError {
    QueueEmpty,
    QueueFull,
    CouldNotAllocateRequiredMemory,
    // ... 11 variants total
}
```

ISR-safe variants return `bool` (the `pxHigherPriorityTaskWoken` flag) instead of blocking.

---

## Configuration

The C wrapper layer (`freertos-api-rust.c`) uses conditional compilation to match your `FreeRTOSConfig.h`. Only APIs whose corresponding `configUSE_*` / `INCLUDE_*` macros are enabled will be compiled.

| FreeRTOS Config | Enables |
|---|---|
| `configSUPPORT_STATIC_ALLOCATION` | `*_static()` constructors |
| `configSUPPORT_DYNAMIC_ALLOCATION` | `new()` dynamic constructors |
| `configUSE_MUTEXES` | `Mutex` |
| `configUSE_RECURSIVE_MUTEXES` | `RecursiveMutex` |
| `configUSE_COUNTING_SEMAPHORES` | `CountingSemaphore` |
| `configUSE_TIMERS` | `Timer`, `pend_function_call` |
| `configUSE_QUEUE_SETS` | `QueueSet` |
| `configUSE_TRACE_FACILITY` | `get_system_state`, `list_tasks` |
| `configUSE_TASK_NOTIFICATIONS` | Task notification APIs |
| `configNUMBER_OF_CORES` | SMP core affinity, per-core handles |
| `configUSE_CORE_AFFINITY` | `set_core_affinity`, `spawn_affinity` |
| `portUSING_MPU_WRAPPERS` | `create_restricted`, MPU region APIs |
| `configGENERATE_RUN_TIME_STATS` | `get_run_time_statistics` |
| `configUSE_TICKLESS_IDLE` | `step_tick`, `increment_tick`, `catch_up_ticks` |
| `configUSE_TASK_PREEMPTION_DISABLE` | `PreemptionGuard` |

### Cargo Features

| Feature | Default | Description |
|---|---|---|
| `panic-handler` | Yes | Provides a `#[panic_handler]` that loops forever. Disable if your application provides its own. |

---

## Thread Safety

| Type | `Send` | `Sync` | Notes |
|---|---|---|---|
| `Task` | Yes | Yes | Handle to a FreeRTOS task |
| `Queue<T>` | Yes | Yes | `T: Send + Copy` required for data methods |
| `BinarySemaphore` | Yes | Yes | |
| `CountingSemaphore` | Yes | Yes | |
| `Mutex` | Yes | Yes | Tracks ownership via `AtomicBool` |
| `RecursiveMutex` | Yes | Yes | Tracks lock count via `AtomicUsize` |
| `Timer` | Yes | Yes | |
| `EventGroup` | Yes | Yes | |
| `StreamBuffer` | Yes | Yes | |
| `BatchingBuffer` | Yes | Yes | |
| `MessageBuffer` | Yes | Yes | |
| `CriticalSection` | Yes | **No** | Intentionally `!Sync` via `PhantomData` |
| `CriticalSectionFromIsr` | Yes | **No** | |
| `FreeRtosAtomicU32` | Yes | Yes | |

All RAII wrappers release their underlying kernel object in `Drop`, preventing resource leaks.

---

## License

MIT
