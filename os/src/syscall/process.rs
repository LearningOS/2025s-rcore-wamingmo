//! Process management syscalls
use crate::{
    task::{exit_current_and_run_next, suspend_current_and_run_next,get_u,write_data,record_syscalls,
    get_syscalls},
    timer::get_time_us,
};

#[repr(C)]
#[derive(Debug)]
pub struct TimeVal {
    pub sec: usize,
    pub usec: usize,
}

/// task exits and submit an exit code
pub fn sys_exit(exit_code: i32) -> ! {
    record_syscalls(93);
    trace!("[kernel] Application exited with code {}", exit_code);
    exit_current_and_run_next();
    panic!("Unreachable in sys_exit!");
}

/// current task gives up resources for other tasks
pub fn sys_yield() -> isize {
    record_syscalls(124);
    trace!("kernel: sys_yield");
    suspend_current_and_run_next();
    0
}

/// get time with second and microsecond
pub fn sys_get_time(ts: *mut TimeVal, _tz: usize) -> isize {
    record_syscalls(169);
    trace!("kernel: sys_get_time");
    let us = get_time_us();
    unsafe {
        *ts = TimeVal {
            sec: us / 1_000_000,
            usec: us % 1_000_000,
        };
    }
    0
}

// TODO: implement the syscall
pub fn sys_trace(_trace_request: usize, _id: usize, _data: usize) -> isize {
    record_syscalls(410);
    trace!("kernel: sys_trace");
    match _trace_request {
        0=>{
            get_u(_id as *const u8)
        },
        1=>{
            write_data(_id as *mut u8,_data as u8);
            0
        },
        2=>{
            get_syscalls(_id) as isize
        }
        _=>-1
    }
}
