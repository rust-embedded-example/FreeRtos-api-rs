# FreeRtos-api-rs API 补全状态清单

本文档整合了 `MISSING_APIS.md`、`FreeRTOS_API_Gap_Analysis.md` 和 `docs/missing-api-report.md` 三份分析报告的内容，跟踪所有缺失 API 的补全状态。

**更新日期**: 2026-04-16
**Crate 状态**: 编译通过 (`cargo build --target thumbv7em-none-eabihf` + `cargo doc`)

---

## 状态标记

| 标记 | 说明 |
|------|------|
| **已完成** | 已添加 C 包装、FFI 声明、安全包装（如适用） |
| **已排除** | 不适合或不需要在库级别包装 |
| **待实现** | 已确认缺失，计划实现 |

---

## 1. 常量 (Constants)

### 1.1 任务优先级常量

| API | 状态 | 说明 |
|-----|------|------|
| `tskIDLE_PRIORITY` | **已完成** | `base::TSK_IDLE_PRIORITY = 0` |
| `tskNO_AFFINITY` | **已完成** | `base::TSK_NO_AFFINITY = !0` |
| `tskDEFAULT_INDEX_TO_NOTIFY` | **已完成** | `base::TSK_DEFAULT_INDEX_TO_NOTIFY = 0` |

### 1.2 内核版本常量

| API | 状态 | 说明 |
|-----|------|------|
| `tskKERNEL_VERSION_NUMBER` | **已排除** | 字符串常量，需运行时获取 |
| `tskKERNEL_VERSION_MAJOR` | **已排除** | 依赖 FreeRTOS 编译配置 |
| `tskKERNEL_VERSION_MINOR` | **已排除** | 依赖 FreeRTOS 编译配置 |
| `tskKERNEL_VERSION_BUILD` | **已排除** | 依赖 FreeRTOS 编译配置 |

### 1.3 错误代码常量

| API | 状态 | 说明 |
|-----|------|------|
| `errQUEUE_EMPTY` | **已完成** | `projdefs::ERR_QUEUE_EMPTY = -1` |
| `errQUEUE_FULL` | **已完成** | `projdefs::ERR_QUEUE_FULL = -2` |
| `errCOULD_NOT_ALLOCATE_REQUIRED_MEMORY` | **已完成** | `projdefs::ERR_COULD_NOT_ALLOCATE_REQUIRED_MEMORY = -3` |
| `errQUEUE_BLOCKED` | **已完成** | `projdefs::ERR_QUEUE_BLOCKED = -4` |
| `errQUEUE_YIELD` | **已完成** | `projdefs::ERR_QUEUE_YIELD = -5` |

### 1.4 事件组控制位常量

| API | 状态 | 说明 |
|-----|------|------|
| `eventCLEAR_EVENTS_ON_EXIT_BIT` | **已完成** | `projdefs::EVENT_CLEAR_EVENTS_ON_EXIT_BIT` |
| `eventUNBLOCKED_DUE_TO_BIT_SET` | **已完成** | `projdefs::EVENT_UNBLOCKED_DUE_TO_BIT_SET` |
| `eventWAIT_FOR_ALL_BITS` | **已完成** | `projdefs::EVENT_WAIT_FOR_ALL_BITS` |
| `eventEVENT_BITS_CONTROL_BYTES` | **已完成** | `projdefs::EVENT_EVENT_BITS_CONTROL_BYTES` |

### 1.5 定时器命令常量

| API | 状态 | 说明 |
|-----|------|------|
| `tmrCOMMAND_*` (13个) | **已完成** | 通过 `base::FreeRtosTimerCommand` 枚举覆盖 |

### 1.6 信号量常量

| API | 状态 | 说明 |
|-----|------|------|
| `semBINARY_SEMAPHORE_QUEUE_LENGTH` | **已完成** | `semphr::SEM_BINARY_SEMAPHORE_QUEUE_LENGTH` |
| `semSEMAPHORE_QUEUE_ITEM_LENGTH` | **已完成** | `semphr::SEM_SEMAPHORE_QUEUE_ITEM_LENGTH` |
| `semGIVE_BLOCK_TIME` | **已完成** | `semphr::SEM_GIVE_BLOCK_TIME` |

### 1.7 流缓冲类型常量

| API | 状态 | 说明 |
|-----|------|------|
| `sbTYPE_STREAM_BUFFER` | **已完成** | `stream_buffer::SB_TYPE_STREAM_BUFFER` |
| `sbTYPE_MESSAGE_BUFFER` | **已完成** | `stream_buffer::SB_TYPE_MESSAGE_BUFFER` |
| `sbTYPE_STREAM_BATCHING_BUFFER` | **已完成** | `stream_buffer::SB_TYPE_STREAM_BATCHING_BUFFER` |

### 1.8 MPU 相关常量 (11个)

| API | 状态 | 说明 |
|-----|------|------|
| `tskMPU_REGION_*` / `tskMPU_*` (全部) | **已排除** | MPU 功能，架构特定 |

---

## 2. 安全包装缺失项 (Safe Wrappers)

### 2.1 Task 模块

| API | FFI | Safe Wrapper | 状态 |
|-----|-----|-------------|------|
| `vTaskPreemptionDisable/Enable` | 已完成 | **已完成** | `task::PreemptionGuard` RAII |
| `portYIELD` | 已完成 | **已完成** | `task::freertos_rs_port_yield` |
| `vTaskSetTimeOutState` | 已完成 | 仅FFI | 底层工具，应用层使用 |
| `xTaskCheckForTimeOut` | 已完成 | 仅FFI | 底层工具，应用层使用 |
| `xTaskGetStaticBuffers` | 已完成 | 仅FFI | 需原始指针输出参数 |
| `vTaskList` / `vTaskGetRunTimeStats` | 已完成 | 仅FFI | 字符串输出，不适合安全包装 |
| `vTaskListTasks` (带长度) | 已完成 | 仅FFI | 字符串输出 |
| `xTaskGetCurrentTaskHandleForCore` | 已完成 | 仅FFI | SMP 专用 |

### 2.2 Queue 模块

| API | FFI | Safe Wrapper | 状态 |
|-----|-----|-------------|------|
| `xQueueOverwrite` | 已完成 | **已完成** | `Queue::overwrite()` |
| `xQueueOverwriteFromISR` | 已完成 | **已完成** | `Queue::overwrite_from_isr()` |
| `xQueuePeekFromISR` | 已完成 | **已完成** | `Queue::peek_from_isr()` |
| `Queue::new_static()` | - | **待实现** | 需要静态内存参数 |
| `QueueSet` 安全包装 | 已有FFI | **待实现** | 复杂度较高 |
| `vQueueWaitForMessageRestricted` | 已完成 | 仅FFI | MPU 专用 |

### 2.3 Semaphore 模块

| API | FFI | Safe Wrapper | 状态 |
|-----|-----|-------------|------|
| `xSemaphoreGetMutexHolder` | 已完成 | **已完成** | `Mutex::get_holder()` |
| `xSemaphoreGetMutexHolderFromISR` | 已完成 | **已完成** | `Mutex::get_holder_from_isr()` |
| `xSemaphoreGetStaticBuffer` | 已有FFI | 仅FFI | 需原始指针输出参数 |

### 2.4 Timer 模块

| API | FFI | Safe Wrapper | 状态 |
|-----|-----|-------------|------|
| `vTimerGetTimerID` | 已完成 | **已完成** | `Timer::get_timer_id()` |
| `vTimerSetTimerID` | 已完成 | **已完成** | `Timer::set_timer_id()` |
| `xTimerPendFunctionCall` | 已有FFI | 仅FFI | 函数指针参数，unsafe 用途 |
| `xTimerGetTimerDaemonTaskHandle` | 已有FFI | 仅FFI | 返回原始句柄 |

### 2.5 Event Group 模块

| API | FFI | Safe Wrapper | 状态 |
|-----|-----|-------------|------|
| `xEventGroupClearBitsFromISR` | 已完成 | **已完成** | `EventGroup::clear_bits_from_isr()` |
| `vEventGroupSetBitsCallback` | 已完成 | 仅FFI | 定时器回调辅助函数 |
| `vEventGroupClearBitsCallback` | 已完成 | 仅FFI | 定时器回调辅助函数 |
| `xEventGroupGetStaticBuffer` | 已有FFI | 仅FFI | 需原始指针输出参数 |

### 2.6 Stream Buffer 模块

| API | FFI | Safe Wrapper | 状态 |
|-----|-----|-------------|------|
| `vStreamBufferSetStreamBufferNotificationIndex` | 已完成 | **已完成** | `StreamBuffer::set_notification_index()` |
| `uxStreamBufferGetStreamBufferNotificationIndex` | 已完成 | **已完成** | `StreamBuffer::get_notification_index()` |
| `xStreamBufferCreateWithCallback` | 已完成 | 仅FFI | 回调函数指针参数 |
| `xStreamBufferCreateStaticWithCallback` | 已完成 | 仅FFI | 回调+静态内存参数 |
| `xStreamBatchingBufferCreate` | 已完成 | **已完成** | `stream_buffer::BatchingBuffer` |
| `xStreamBatchingBufferCreateWithCallback` | 已完成 | 仅FFI | 回调函数指针参数 |
| `xStreamBatchingBufferCreateStatic` | 已完成 | 仅FFI | 静态内存参数 |
| `xStreamBatchingBufferCreateStaticWithCallback` | 已完成 | 仅FFI | 回调+静态内存参数 |
| `xStreamBufferGetStaticBuffers` | 已有FFI | 仅FFI | 需原始指针输出参数 |

### 2.7 Message Buffer 模块

| API | FFI | Safe Wrapper | 状态 |
|-----|-----|-------------|------|
| `xMessageBufferResetFromISR` | 已完成 | **已完成** | `MessageBuffer::reset_from_isr()` |
| `xMessageBufferCreateWithCallback` | 已完成 | 仅FFI | 回调函数指针参数 |
| `xMessageBufferCreateStaticWithCallback` | 已完成 | 仅FFI | 回调+静态内存参数 |
| `xMessageBufferGetStaticBuffers` | **已完成** | 仅FFI | 需原始指针输出参数 |
| `xMessageBufferSendCompletedFromISR` | **已完成** | 仅FFI | ISR 发送完成回调 |
| `xMessageBufferReceiveCompletedFromISR` | **已完成** | 仅FFI | ISR 接收完成回调 |

---

## 3. C 包装函数缺失项 (C Wrappers)

| API | 状态 | 说明 |
|-----|------|------|
| `xQueueCRSend` / `xQueueCRReceive` | **已完成** | 协程队列操作 |
| `xQueueCRSendFromISR` / `xQueueCRReceiveFromISR` | **已完成** | 协程队列 ISR |
| `vQueueWaitForMessageRestricted` | **已完成** | MPU 限制等待 |
| `vEventGroupSetBitsCallback` | **已完成** | 事件组位设置回调 |
| `vEventGroupClearBitsCallback` | **已完成** | 事件组位清除回调 |
| `xStreamBufferCreateWithCallback` | **已完成** | 带回调创建流缓冲 |
| `xStreamBufferCreateStaticWithCallback` | **已完成** | 静态创建带回调 |
| `xStreamBatchingBufferCreate` | **已完成** | 创建批处理缓冲 |
| `xStreamBatchingBufferCreateWithCallback` | **已完成** | 带回调创建 |
| `xStreamBatchingBufferCreateStatic` | **已完成** | 静态创建批处理缓冲 |
| `xStreamBatchingBufferCreateStaticWithCallback` | **已完成** | 静态创建带回调 |
| `xMessageBufferCreateWithCallback` | **已完成** | 带回调创建消息缓冲 |
| `xMessageBufferCreateStaticWithCallback` | **已完成** | 静态创建带回调 |
| `xMessageBufferResetFromISR` | **已完成** | ISR 重置消息缓冲 |
| `portYIELD` | **已完成** | 端口让出 |
| `vTaskGetIdleTaskMemory` | **已完成** | 空闲任务静态内存 |
| `vTaskGetPassiveIdleTaskMemory` | **已完成** | 被动空闲任务内存 |
| `ulTaskGetIdleRunTimeCounter` | **已完成** | 空闲任务运行时间计数器 |
| `ulTaskGetIdleRunTimePercent` | **已完成** | 空闲任务 CPU 使用百分比 |
| `xTaskCreateRestrictedAffinitySet` | **已完成** | MPU 受限任务+核心亲和力 |
| `xMessageBufferSendCompletedFromISR` | **已完成** | 消息缓冲 ISR 发送完成回调 |
| `xMessageBufferReceiveCompletedFromISR` | **已完成** | 消息缓冲 ISR 接收完成回调 |

---

## 4. Task Notification Indexed/Query 系列

这些 API 在 `docs/missing-api-report.md` 中列为缺失。当前 crate 通过 `freertos_rs_task_generic_notify*` 系列函数提供了底层支持，但未直接暴露 FreeRTOS 公开的 indexed 宏名称。

| API | 底层等价 | 状态 |
|-----|---------|------|
| `xTaskNotifyIndexed` | `freertos_rs_task_generic_notify(handle, index, value, action, &prev)` | **已完成** (通过 generic) |
| `xTaskNotifyAndQuery` | `freertos_rs_task_generic_notify(handle, 0, value, action, &prev)` | **已完成** (通过 generic) |
| `xTaskNotifyAndQueryIndexed` | `freertos_rs_task_generic_notify(handle, index, value, action, &prev)` | **已完成** (通过 generic) |
| `xTaskNotifyIndexedFromISR` | `freertos_rs_task_generic_notify_from_isr(...)` | **已完成** (通过 generic) |
| `xTaskNotifyAndQueryFromISR` | `freertos_rs_task_generic_notify_from_isr(handle, 0, ...)` | **已完成** (通过 generic) |
| `xTaskNotifyAndQueryIndexedFromISR` | `freertos_rs_task_generic_notify_from_isr(...)` | **已完成** (通过 generic) |
| `xTaskNotifyWaitIndexed` | `freertos_rs_task_generic_notify_wait(index, ...)` | **已完成** (通过 generic) |
| `xTaskNotifyGiveIndexed` | `freertos_rs_task_generic_notify(handle, index, 0, eIncrement, null)` | **已完成** (通过 generic) |
| `vTaskNotifyGiveIndexedFromISR` | `freertos_rs_task_generic_notify_give_from_isr(handle, index, ...)` | **已完成** (通过 generic) |
| `ulTaskNotifyTakeIndexed` | `freertos_rs_task_notify_take(clear, wait)` + index 0 | **已完成** (通过 generic) |
| `xTaskNotifyStateClearIndexed` | `freertos_rs_task_generic_notify_state_clear(handle, index)` | **已完成** (通过 generic) |
| `ulTaskNotifyValueClearIndexed` | `freertos_rs_task_generic_notify_value_clear(handle, index, bits)` | **已完成** (通过 generic) |

**说明**: FreeRTOS 的 indexed notification 宏最终调用的是 `xTaskGenericNotify` 等 generic 函数。当前 crate 已包装所有 generic 函数，indexed 版本可通过传递 `index` 参数实现。

---

## 5. 明确排除的 API

以下 API 不适合在库级别包装，原因如下：

### 5.1 应用钩子函数 (Application Hooks)
由用户在 C 层自行定义，不应在库中包装：
- `vApplicationStackOverflowHook()`
- `vApplicationIdleHook()`
- `vApplicationTickHook()`
- `vApplicationGetIdleTaskMemory()`
- `vApplicationGetPassiveIdleTaskMemory()`
- `vApplicationGetTimerTaskMemory()`
- `vApplicationDaemonTaskStartupHook()`

### 5.2 内核内部函数
仅供 FreeRTOS 内核内部使用：
- `vTaskResetState()`, `vTaskYieldWithinAPI()`
- `xTaskGetMPUSettings()`, `vGrantAccessToKernelObject()`
- `vRevokeAccessToKernelObject()`, `vPortGrantAccessToKernelObject()`
- `vPortRevokeAccessToKernelObject()`
- `vCoRoutineResetState()`, `xCoRoutineRemoveFromEventList()`

### 5.3 配置宏
编译时 C 配置，不应在 Rust 中定义：
- `configUSE_PREEMPTION`, `configUSE_IDLE_HOOK` 等全部 `config*` 宏

### 5.4 MPU 相关
架构特定，低优先级：
- `tskMPU_REGION_*` / `tskMPU_*` 全部 MPU 常量
- `vTaskAllocateMPURegions()`
- `vPortStoreTaskMPUSettings()`, `xPortIsAuthorizedToAccessBuffer()`

### 5.5 协程宏
复杂 C 宏 + 已废弃 API：
- `crSTART()`, `crEND()`, `crSET_STATE0/1()`, `crDELAY()`
- `crQUEUE_SEND()`, `crQUEUE_RECEIVE()` 等

### 5.6 列表完整性检查
调试功能，不适合 Rust 包装：
- `list*_LIST_ITEM_INTEGRITY_CHECK_VALUE*`
- `listSET/TEST_LIST_*_INTEGRITY*`

### 5.7 原子临界区宏
应使用 Rust `core::sync::atomic` 替代：
- `ATOMIC_ENTER_CRITICAL`, `ATOMIC_EXIT_CRITICAL`

### 5.8 调度器/端口内部
内核启动函数和端口级临界区：
- `xPortStartScheduler()`, `vPortEndScheduler()`
- `vPortEnterCritical()`, `vPortExitCritical()`
- `vPortSetInterruptMask()`, `vPortClearInterruptMask()`
- `portMEMORY_BARRIER()`
- `pvPortMallocStack()`, `vPortFreeStack()`

### 5.9 列表宏（部分）
已有函数版本覆盖的宏 API：
- `listSET/GET_LIST_ITEM_OWNER()`, `listSET/GET_LIST_ITEM_VALUE()`
- `listGET_HEAD_ENTRY()`, `listGET_NEXT()`, `listGET_END_MARKER()`
- `listLIST_IS_EMPTY()`, `listCURRENT_LIST_LENGTH()`
- `listGET_OWNER_OF_NEXT_ENTRY()`, `listGET_OWNER_OF_HEAD_ENTRY()`
- `listIS_CONTAINED_WITHIN()`, `listLIST_IS_INITIALISED()`
- `listINSERT_END()`, `listREMOVE_ITEM()`

---

## 6. 统计

### 已实现（本次补全）

| 类别 | 数量 |
|------|------|
| 常量 | 18 个已添加 |
| C 包装函数 | 18 个已添加 |
| FFI 声明 | 18 个已添加 |
| 安全包装方法 | 10 个已添加 |
| 安全包装类型 | 2 个已添加 (`PreemptionGuard`, `BatchingBuffer`) |

### 已排除

| 类别 | 数量 | 原因 |
|------|------|------|
| 应用钩子 | 7 | 由用户定义 |
| 内核内部 | 6+ | 内部使用 |
| 配置宏 | 16+ | 编译时 C 配置 |
| MPU | 17+ | 架构特定 |
| 协程宏 | 9+ | 已废弃 |
| 调试/内部 | 20+ | 调试功能 |

### 待实现（低优先级）

| 项目 | 说明 |
|------|------|
| `Queue::new_static()` | 静态分配构造 |
| `QueueSet` 安全包装 | 较复杂 |
| WithCallback 安全构造器 | 回调函数指针使安全包装困难 |

---

## 7. 循环审计修复记录 (Loop Audit Fixes)

### 迭代 1 — 2026-04-16

#### 严重修复（链接器错误）

| 问题 | 修复 |
|------|------|
| `croutine.rs` 命名不匹配 | Rust FFI `freertos_rs_co_routine_queue_*` 改为匹配 C 的 `freertos_rs_queue_cr_*` |
| `croutine.rs` 无效声明 | 移除 `freertos_rs_co_routine_delay`（crDELAY 是宏，无法独立包装） |
| `message_buffer.rs` 命名不匹配 | `freertos_rs_message_buffer_space_available` → `freertos_rs_message_buffer_spaces_available` |
| `task.rs` 重复声明 | 移除 `freertos_rs_port_yield` 重复声明（保留 portable.rs） |

#### 缺失 FFI 声明补充（C 包装已有，Rust 声明缺失）

| API | 添加到 |
|-----|--------|
| `freertos_rs_queue_wait_for_message_restricted` | `queue.rs` |
| `freertools_rs_event_group_set_bits_callback` | `event_groups.rs` |
| `freertos_rs_event_group_clear_bits_callback` | `event_groups.rs` |
| `freertos_rs_message_buffer_get_static_buffers` | `message_buffer.rs` |

#### 新增缺失 API（C 包装 + Rust FFI）

| API | FreeRTOS 原型 | 条件编译 |
|-----|---------------|----------|
| `freertos_rs_task_get_idle_run_time_counter` | `ulTaskGetIdleRunTimeCounter()` | `configGENERATE_RUN_TIME_STATS` |
| `freertos_rs_task_get_idle_run_time_percent` | `ulTaskGetIdleRunTimePercent()` | `configGENERATE_RUN_TIME_STATS` |
| `freertos_rs_task_create_restricted_affinity_set` | `xTaskCreateRestrictedAffinitySet()` | `portUSING_MPU_WRAPPERS` |
| `freertos_rs_message_buffer_send_completed_from_isr` | `xMessageBufferSendCompletedFromISR()` | `configUSE_STREAM_BUFFERS` |
| `freertos_rs_message_buffer_receive_completed_from_isr` | `xMessageBufferReceiveCompletedFromISR()` | `configUSE_STREAM_BUFFERS` |

---

*本文档由三份差距分析报告整合生成，并通过循环审计持续更新。反映当前 `FreeRtos-api-rs` 的 API 覆盖状态。*
