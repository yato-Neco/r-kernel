use crate::trap::TrapFrame;
use core::arch::asm;
use core::error;
//Artificial Linguistic Internet Computer Entity
use crate::print;
use crate::println;

const TIMER_INTERVAL: u64 = 50000000;

use riscv::register::{sie, sip, sstatus, time};

pub(crate) fn interrupt() {
    let is_timer_interrput = sip::read().stimer();
    if is_timer_interrput {
        init_timer()
    }
}

pub fn init_timer() {
    let time = time::read64();
    println!("{}", time);

    println!("{:?}", set_next_timer(time + TIMER_INTERVAL));
}

fn set_next_timer(time: u64) -> Result<u64, i64> {
    /*
    unsafe {
        // mtimecmpに次の時間を設定（この例ではQEMUやVirt機などのRISC-V標準レジスタを想定）
        let mtimecmp = 0x0200_4000 as *mut u64;
        //println!("{}",mtimecmp.read());
        mtimecmp.write_volatile(time);
    }
    */

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
            0 => Result::Ok(value),
            e => Result::Err(e),
        }
    }
}

pub fn timer_interrupt(trap_frame: &mut TrapFrame) {
    panic!()
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
