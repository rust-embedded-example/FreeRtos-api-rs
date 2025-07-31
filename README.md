# FreeRTOS Rust 中间件示例

这是一个使用Rust制作FreeRTOS C语言工程中间件的示例项目。通过这个项目，你可以用Rust编写代码，编译成对象文件(.o)，然后在C语言的FreeRTOS工程中调用。

## 项目结构

```
freertos-api-rs/
├── src/
│   ├── lib.rs          # 主要示例代码
│   ├── api.c           # FreeRTOS API C包装函数
│   ├── base.rs         # 基础类型定义
│   ├── task.rs         # 任务管理API
│   ├── queue.rs        # 队列管理API
│   ├── semphr.rs       # 信号量API
│   ├── timers.rs       # 定时器API
│   └── ...             # 其他模块
├── Cargo.toml
└── README.md
```

## 示例代码说明

### lib.rs 完整示例

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

/// Panic handler - 在 no_std 环境中必需的
extern "C" fn led_task(_params: *mut core::ffi::c_void) {
    loop {
        // Toggle LED
        unsafe {
            freertos_rs_task_delay(500); // 500ms delay
        }
    }
}

// Main function
#[unsafe(no_mangle)]
pub extern "C" fn rust_create_led_task() {
    // Create a task
    let task_handle: *mut *const core::ffi::c_void = core::ptr::null_mut();
    
    unsafe {
        freertos_rs_task_create(
            led_task,
            b"LED_Task\0".as_ptr(),
            128, // Stack size
            core::ptr::null_mut(),
            1, // Priority
            task_handle
        );
        
        // Start the scheduler
        freertos_rs_task_start_scheduler();
    }
}
```

### 代码解析

1. **模块导入**
   - 导入FreeRTOS任务相关的API包装函数
   - 这些函数最终调用FreeRTOS的C API

2. **Panic处理器**
   - `#[panic_handler]` 是no_std环境必需的
   - 在嵌入式环境中panic时进入无限循环

3. **任务函数**
   - `led_task`: 标准的FreeRTOS任务函数
   - 使用`extern "C"`确保C调用约定兼容
   - 无限循环，每500ms执行一次

4. **导出函数**
   - `#[no_mangle]` 防止函数名被编译器修改
   - `pub extern "C"` 使函数可被C代码调用
   - 创建任务并启动调度器

## 编译步骤

### 1. 配置Cargo.toml

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

### 2. 配置.cargo/config.toml

```toml
[build]
target = "thumbv7em-none-eabihf"
# target = "thumbv7m-none-eabi"

[target.thumbv7em-none-eabihf]
rustflags = [
  "--emit=obj",
  "-O",
  "-C", "target-cpu=cortex-m7",
  "-C", "linker=rust-lld",
] 
```

### 3. 编译命令

```bash
cargo build --release
```

### 4. 获取编译产物

编译完成后，在以下路径找到生成的文件：

```
target/thumbv7em-none-eabihf/release/
├── libfreertos_api_rs.a                    # 静态库文件
└── deps/freertos_api_rs-<hash>.o          # 对象文件
```

## 在C工程中使用

### 1. 复制文件

将以下文件复制到你的C工程：
- `freertos_api_rs-<hash>.o` (对象文件)
- `src/freertos-api-rust.c` (FreeRTOS API包装函数)

### 2. 在C代码中声明和调用

```c
// main.c
#include "FreeRTOS.h"
#include "task.h"

// 声明Rust导出的函数
extern void rust_create_led_task(void);

int main(void) {
    // 硬件初始化
    SystemInit();

    // 调用Rust函数创建LED任务并启动调度器
    rust_create_led_task();

    // 调度器启动后不会执行到这里
    while(1);
}
```

## 注意事项

- 确保Rust编译目标与你的MCU架构匹配
- 所有导出函数必须使用 `#[no_mangle]` 和 `extern "C"`
- 在C工程中包含 `api.c` 文件提供FreeRTOS API包装
- 确保FreeRTOS配置支持你使用的功能模块