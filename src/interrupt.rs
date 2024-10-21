use crate::proc::yield_;
use core::arch::asm;
use crate::print;
use crate::println;

const TIMER_INTERVAL: u64 = 15000000;
const FIRST_TIMER_INTERVAL: u64 = 5000000;

use alloc::string::ToString;
use riscv::register::sie;
use riscv::register::time;

#[no_mangle]
pub fn init_timer() {
    let time = time::read64();
    set_next_timer(time + FIRST_TIMER_INTERVAL).unwrap();
}

#[no_mangle]
pub fn set_next_timer(time: u64)  -> Result<u64, u64> {
    unsafe {
        let value: u64;
        let error: i64;

        asm!(
            "ecall",
            in("a6") 0,
            in("a7") 0,
            inlateout("a0") time - 1 => error,
            lateout("a1") value,
        );

        match error {
            0 => Ok(value),
            _ => Err(value),
        }
    }
}

#[no_mangle]
pub fn set_timer() {
    let time = time::read64();
    set_next_timer(time + TIMER_INTERVAL).unwrap();
    //yield_();
}

#[no_mangle]
pub extern "C" fn timer_handler() {
    
    panic!();
}

/*
fn set_next_timer(time: u64) {
    unsafe {
        asm!(
            "ecall",
            in("a0") time - 1,
            in("a6") 0,
            in("a7") 0,
        );
    }
}
*/
