//! Process management syscalls

use core::usize;

use crate::{
    config::MAX_SYSCALL_NUM,
    mm::{translated_virtual_ptr, VirtAddr},
    task::{
        change_program_brk, current_user_token, exit_current_and_run_next, get_frist_run_time,
        get_syscall_times, suspend_current_and_run_next, task_add_map_area, task_remove_map_area,
        TaskStatus,
    },
    timer::{get_time_ms, get_time_us},
};

#[repr(C)]
#[derive(Debug)]
pub struct TimeVal {
    pub sec: usize,
    pub usec: usize,
}

/// Task information
#[allow(dead_code)]
pub struct TaskInfo {
    /// Task status in it's life cycle
    status: TaskStatus,
    /// The numbers of syscall called by task
    syscall_times: [u32; MAX_SYSCALL_NUM],
    /// Total running time of task
    time: usize,
}

/// task exits and submit an exit code
pub fn sys_exit(_exit_code: i32) -> ! {
    trace!("kernel: sys_exit");
    exit_current_and_run_next();
    panic!("Unreachable in sys_exit!");
}

/// current task gives up resources for other tasks
pub fn sys_yield() -> isize {
    trace!("kernel: sys_yield");
    suspend_current_and_run_next();
    0
}

/// YOUR JOB: get time with second and microsecond
/// HINT: You might reimplement it with virtual memory management.
/// HINT: What if [`TimeVal`] is splitted by two pages ?
pub fn sys_get_time(ts: *mut TimeVal, _tz: usize) -> isize {
    trace!("kernel: sys_get_time");
    let us = get_time_us();
    // 获得物理地址
    // TODO 处理跨页问题
    let pa = translated_virtual_ptr(current_user_token(), VirtAddr(ts as usize)).unwrap();
    let timeval_ptr = pa.0 as *mut TimeVal;
    unsafe {
        *timeval_ptr = TimeVal {
            sec: us / 1_000_000,
            usec: us % 1_000_000,
        };
    }
    0
}

/// YOUR JOB: Finish sys_task_info to pass testcases
/// HINT: You might reimplement it with virtual memory management.
/// HINT: What if [`TaskInfo`] is splitted by two pages ?
pub fn sys_task_info(ti: *mut TaskInfo) -> isize {
    trace!("kernel: sys_task_info NOT IMPLEMENTED YET!");
    // TODO 处理跨页问题
    let pa = translated_virtual_ptr(current_user_token(), VirtAddr(ti as usize)).unwrap();
    let taskinfo_ptr = pa.0 as *mut TaskInfo;
    unsafe {
        *taskinfo_ptr = TaskInfo {
            status: TaskStatus::Running,
            syscall_times: get_syscall_times(),
            time: get_time_ms() - get_frist_run_time(),
        };
    }
    0
}

// YOUR JOB: Implement mmap.
pub fn sys_mmap(start: usize, len: usize, port: usize) -> isize {
    trace!("kernel: sys_mmap NOT IMPLEMENTED YET!");
    info!("syscall start addr : {:#X}", start);
    task_add_map_area(start, len, port)
}

// YOUR JOB: Implement munmap.
pub fn sys_munmap(start: usize, len: usize) -> isize {
    trace!("kernel: sys_munmap NOT IMPLEMENTED YET!");
    task_remove_map_area(start, len)
}

/// change data segment size
pub fn sys_sbrk(size: i32) -> isize {
    trace!("kernel: sys_sbrk");
    if let Some(old_brk) = change_program_brk(size) {
        old_brk as isize
    } else {
        -1
    }
}
