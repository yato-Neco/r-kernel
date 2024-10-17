use riscv::register::{mcause, scause, sepc, sip, stval, stvec, time};

use crate::{interrupt::*, println};
use core::arch::{asm, global_asm};
type RregisterSize = u64;
use crate::print;

#[naked]
#[no_mangle]

pub unsafe extern "C" fn trap_entry() {
    asm!(
        "jal handle_trap\n",
        options(noreturn)
    );
}

#[no_mangle]
pub extern "C" fn print_trap() {
    println!("handle_trap!!!");
    let scause = scause::read();
    println!("cause: {:?}", scause.cause());
    println!("is_interrupt: {:?}", scause.is_interrupt());
    println!("is_exception: {:?}", scause.is_exception());
    println!("code: {:?}", scause.code());
    println!("bits: {:x}", scause.bits());
    let sip = sip::read().bits();
    println!("sepc: {:x}", sip);

}

#[no_mangle]
pub unsafe extern "C" fn handle_trap() {
    let scause = scause::read();

    let is_interrupt = scause.is_interrupt();
    let is_exception = scause.is_exception();
    let is_timer = sip::read().stimer();
    //let spec = sepc::read();

    if is_interrupt {
        if is_timer {
            set_timer();
        }
    }

    if is_exception  {
        panic!("{:?}", scause.cause())
    }
}

#[derive(Debug)]
#[repr(C, packed)]
pub struct TrapFrame {
    ra: RregisterSize,
    gp: RregisterSize,
    t0: RregisterSize,
    t1: RregisterSize,
    t2: RregisterSize,
    t3: RregisterSize,
    t4: RregisterSize,
    t5: RregisterSize,
    t6: RregisterSize,
    a0: RregisterSize,
    a1: RregisterSize,
    a2: RregisterSize,
    a3: RregisterSize,
    a4: RregisterSize,
    a5: RregisterSize,
    a6: RregisterSize,
    a7: RregisterSize,
    s0: RregisterSize,
    s1: RregisterSize,
    s2: RregisterSize,
    s3: RregisterSize,
    s4: RregisterSize,
    s5: RregisterSize,
    s6: RregisterSize,
    s7: RregisterSize,
    s8: RregisterSize,
    s9: RregisterSize,
    s10: RregisterSize,
    s11: RregisterSize,
    sp: RregisterSize,
}
