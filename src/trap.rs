use riscv::register::{mcause, scause, sip, stval, stvec, time};

use crate::{
    interrupt::{*},
    println,
};
use core::arch::{asm, global_asm};
type RregisterSize = u64;
use crate::print;
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

#[naked]
#[no_mangle]

pub unsafe extern "C" fn trap_entry() {
    asm!(
        "csrw sscratch, sp\n",
        "addi  sp, sp, -8*17\n",
        "sd    ra, 0*8(sp)\n",
        "sd    a0, 1*8(sp)\n",
        "sd    a1, 2*8(sp)\n",
        "sd    a2, 3*8(sp)\n",
        "sd    a3, 4*8(sp)\n",
        "sd    a4, 5*8(sp)\n",
        "sd    a5, 6*8(sp)\n",
        "sd    a6, 7*8(sp)\n",
        "sd    a7, 8*8(sp)\n",
        "sd    t0, 9*8(sp)\n",
        "sd    t1, 10*8(sp)\n",
        "sd    t2, 11*8(sp)\n",
        "sd    t3, 12*8(sp)\n",
        "sd    t4, 13*8(sp)\n",
        "sd    t5, 14*8(sp)\n",
        "sd    t6, 15*8(sp)\n",
        "sd    s0, 16*8(sp)\n",
        
        "csrr a0, sscratch\n",
        "sd a0, 8 * 17(sp)\n",

        "mv a0, sp\n",
        "call handle_trap\n",

        "ld    ra, 0*8(sp)",
        "ld    a0, 1*8(sp)",
        "ld    a1, 2*8(sp)",
        "ld    a2, 3*8(sp)",
        "ld    a3, 4*8(sp)",
        "ld    a4, 5*8(sp)",
        "ld    a5, 6*8(sp)",
        "ld    a6, 7*8(sp)",
        "ld    a7, 8*8(sp)",
        "ld    t0, 9*8(sp)",
        "ld    t1, 10*8(sp)",
        "ld    t2, 11*8(sp)",
        "ld    t3, 12*8(sp)",
        "ld    t4, 13*8(sp)",
        "ld    t5, 14*8(sp)",
        "ld    t6, 15*8(sp)",
        "ld    s0, 16*8(sp)",
        "addi  sp, sp, 8*17",
        "sret\n",
        options(noreturn)
    );
}

#[no_mangle]
pub unsafe  extern "C" fn handle_trap(trap_frame: &mut TrapFrame) {
    println!("handle_trap!!!");
    let scause = scause::read();
    
    let is_interrupt = scause.is_interrupt();
    let is_exception =  scause.is_exception();
    let is_timer = sip::read().stimer();
    println!("cause: {:?}", scause.cause());
    println!("is_interrupt: {:?}", scause.is_interrupt());
    println!("is_exception: {:?}", scause.is_exception());
    println!("code: {:?}", scause.code());
    println!("bits: {:x}", scause.bits());

    if is_interrupt {
        if is_timer {
            init_timer();
        }
        
    }

    if is_exception {
        panic!()
    }
}


