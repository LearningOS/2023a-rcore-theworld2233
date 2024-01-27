//! Types related to task management

//use alloc::vec::Vec;

use super::TaskContext;

/// The task control block (TCB) of a task.
#[derive(Clone, Copy)]
pub struct TaskControlBlock {
    /// The task status in it's lifecycle
    pub task_status: TaskStatus,
    /// The task context
    pub task_cx: TaskContext,
    /// sys call num
    pub task_sys: [u32;500],
    /// time
    pub task_time:usize,
}

/// The status of a task
#[derive(Copy, Clone, PartialEq)]
pub enum TaskStatus {
    /// uninitialized
    UnInit,
    /// ready to run
    Ready,
    /// running
    Running,
    /// exited
    Exited,
}
