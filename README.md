# FreeRTOS Rust Middleware Example

This is an example project for creating FreeRTOS C project middleware using Rust. Through this project, you can write Rust code, compile it to object files (.o), and then call it from C language FreeRTOS projects.

## Project Structure

```
freertos-api-rs/
├── src/
│   ├── lib.rs          # Main example code
│   ├── api.c           # FreeRTOS API C wrapper functions
│   ├── base.rs         # Basic type definitions
│   ├── task.rs         # Task management API
│   ├── queue.rs        # Queue management API
│   ├── semphr.rs       # Semaphore API
│   ├── timers.rs       # Timer API
│   └── ...             # Other modules
├── Cargo.toml
└── README.md
```

## Example Code Explanation

### Complete lib.rs Example

```rust
pub use base::*;

use task::{freertos_rs_task_delay,
    freertos_rs_task_create,
    freertos_rs_task_start_scheduler};

use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

/// LED task function - executes every 500ms
extern "C" fn led_task(_params: *mut core::ffi::c_void) {
    loop {
        // Toggle LED - add your LED control logic here
        unsafe {
            freertos_rs_task_delay(500); // 500ms delay
        }
    }
}

/// Function exported for C language calls - creates LED task and starts scheduler
#[no_mangle]
pub extern "C" fn rust_create_led_task() {
    // Create task handle
    let task_handle: *mut *const core::ffi::c_void = core::ptr::null_mut();

    unsafe {
        freertos_rs_task_create(
            led_task,                    // Task function pointer
            b"LED_Task\0".as_ptr(),     // Task name
            128,                        // Stack size (in words)
            core::ptr::null_mut(),      // Task parameters
            1,                          // Task priority
            task_handle                 // Task handle
        );

        // Start FreeRTOS scheduler
        freertos_rs_task_start_scheduler();
    }
}
```

### Code Analysis

1. **Module Imports**
   - Import FreeRTOS task-related API wrapper functions
   - These functions ultimately call FreeRTOS C APIs

2. **Panic Handler**
   - `#[panic_handler]` is required in no_std environment
   - Enters infinite loop when panic occurs in embedded environment

3. **Task Function**
   - `led_task`: Standard FreeRTOS task function
   - Uses `extern "C"` to ensure C calling convention compatibility
   - Infinite loop, executes every 500ms

4. **Export Function**
   - `#[no_mangle]` prevents function name from being modified by compiler
   - `pub extern "C"` makes function callable from C code
   - Creates task and starts scheduler

## Build Steps

### 1. Configure Cargo.toml

```toml
[package]
name = "freertos-api-rs"
version = "0.1.0"
edition = "2024"

[lib]
crate-type = ["staticlib"]

[profile.dev]
panic = "abort"

[profile.release]
panic = "abort"
opt-level = "s"
lto = true
```

### 2. Configure .cargo/config.toml

```toml
[build]
target = "thumbv7em-none-eabihf"  # Adjust according to your MCU
# target = "thumbv7m-none-eabi"

[target.thumbv7em-none-eabihf]
rustflags = [
  "--emit=obj",
  "-O",
  "-C", "target-cpu=cortex-m7",
  "-C", "linker=rust-lld",
]
```

### 3. Build Command

```bash
cargo build --release
```

### 4. Get Build Artifacts

After compilation, find the generated files at the following path:

```
target/thumbv7em-none-eabihf/release/
├── libfreertos_api_rs.a                    # Static library file
└── deps/freertos_api_rs-<hash>.o          # Object file
```

## Using in C Projects

### 1. Copy Files

Copy the following files to your C project:
- `freertos_api_rs-<hash>.o` (object file)
- `src/freertos-api-rust.c` (FreeRTOS API wrapper functions)

### 2. Declare and Call in C Code

```c
// main.c
#include "FreeRTOS.h"
#include "task.h"

// Declare Rust exported functions
extern void rust_create_led_task(void);

int main(void) {
    // Hardware initialization
    SystemInit();

    // Call Rust function to create LED task and start scheduler
    rust_create_led_task();

    // Won't execute here after scheduler starts
    while(1);
}
```

## Notes

- Ensure Rust build target matches your MCU architecture
- All exported functions must use `#[no_mangle]` and `extern "C"`
- Include `api.c` file in C project to provide FreeRTOS API wrappers
- Ensure FreeRTOS configuration supports the functional modules you use