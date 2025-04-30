//! Process management syscalls
use crate::task::{change_program_brk, exit_current_and_run_next, suspend_current_and_run_next,
record_syscalls,get_syscalls,current_user_token, read_data,write_data,mmap,munmap};
use crate::mm::translated_byte_buffer;
use crate::timer::get_time_us;
#[repr(C)]
#[derive(Debug,Copy,Clone)]
pub struct TimeVal {
    pub sec: usize,
    pub usec: usize,
}

/// task exits and submit an exit code
pub fn sys_exit(_exit_code: i32) -> ! {
    record_syscalls(93);
    //trace!("kernel: sys_exit");
    exit_current_and_run_next();
    panic!("Unreachable in sys_exit!");
}

/// current task gives up resources for other tasks
pub fn sys_yield() -> isize {
    record_syscalls(124);
    //trace!("kernel: sys_yield");
    suspend_current_and_run_next();
    0
}

/// YOUR JOB: get time with second and microsecond
/// HINT: You might reimplement it with virtual memory management.
/// HINT: What if [`TimeVal`] is splitted by two pages ?
pub fn sys_get_time(_ts: *mut TimeVal, _tz: usize) -> isize {
    record_syscalls(169);
    //trace!("kernel: sys_get_time");
    let us = get_time_us();
    let time = TimeVal {
        sec:us/1_000_000,
        usec:us%1_000_000,
    };
    let token = current_user_token();
    let dsts = translated_byte_buffer(token,_ts as *const u8,core::mem::size_of_val(&_ts));
    for dst in dsts.into_iter(){
        unsafe{
            let ti = dst.as_mut_ptr() as * mut TimeVal;
            *ti = time;
        }
    }
    0
}

/// TODO: Finish sys_trace to pass testcases
/// HINT: You might reimplement it with virtual memory management.
pub fn sys_trace(_trace_request: usize, _id: usize, _data: usize) -> isize {
    record_syscalls(410);
    trace!("kernel: sys_trace");
    trace!("_trace_request:{},_id:{},_data:{}",_trace_request,_id,_data);
    match _trace_request {
        0=>{
            //trace!("in match,to read_data");
            read_data(_id)
        },
        1=>{
            trace!("in match,to write");
            write_data(_id,_data as u8)
        },
        2=>{
            //trace!("in match,to get_syscalls");
            get_syscalls(_id)
        },
        _=>-1
    }
}

// YOUR JOB: Implement mmap.
pub fn sys_mmap(_start: usize, _len: usize, _port: usize) -> isize {
    record_syscalls(222);
    trace!("kernel: sys_mmap NOT IMPLEMENTED YET!");
    mmap(_start,_len,_port)
}

// YOUR JOB: Implement munmap.
pub fn sys_munmap(_start: usize, _len: usize) -> isize {
    record_syscalls(215);
    trace!("kernel: sys_munmap NOT IMPLEMENTED YET!");
    munmap(_start,_len)
}
/// change data segment size
pub fn sys_sbrk(size: i32) -> isize {
    record_syscalls(214);
    trace!("kernel: sys_sbrk");
    if let Some(old_brk) = change_program_brk(size) {
        old_brk as isize
    } else {
        -1
    }
}
