use riscv::register::{mcause, scause, sepc, sie, sip, stval, stvec, time};

use crate::{interrupt::*, println, proc::yield_};
use core::arch::{asm, global_asm,naked_asm};
type RregisterSize = u64;
use crate::print;

///BASE+要因ｘ４
#[naked]
#[no_mangle]
#[repr(align(4))]
pub unsafe extern "C" fn vec_trap_entry() {
    naked_asm!(
        "j   handle_trap",
        ".balign 4",
        "j   handle_trap",
        ".balign 4",
        "j   handle_trap",
        ".balign 4",
        "j   handle_trap",
        ".balign 4",
        "j   handle_trap",
        ".balign 4",
        "j   timer_trap",
        ".balign 4",
        "j   handle_trap",
        ".balign 4",
        "j   handle_trap",
        ".balign 4",
        "j   handle_trap",
        ".balign 4",
    )
}

#[naked]
#[no_mangle]
#[repr(align(4))]
pub unsafe extern "C" fn trap_entry() {
    naked_asm!(
        "csrrw sp, sscratch, sp",
        "addi sp, sp, -8 * 31",
        "sd ra,  8 * 0(sp)",
        "sd gp,  8 * 1(sp)",
        "sd tp,  8 * 2(sp)",
        "sd t0,  8 * 3(sp)",
        "sd t1,  8 * 4(sp)",
        "sd t2,  8 * 5(sp)",
        "sd t3,  8 * 6(sp)",
        "sd t4,  8 * 7(sp)",
        "sd t5,  8 * 8(sp)",
        "sd t6,  8 * 9(sp)",
        "sd a0,  8 * 10(sp)",
        "sd a1,  8 * 11(sp)",
        "sd a2,  8 * 12(sp)",
        "sd a3,  8 * 13(sp)",
        "sd a4,  8 * 14(sp)",
        "sd a5,  8 * 15(sp)",
        "sd a6,  8 * 16(sp)",
        "sd a7,  8 * 17(sp)",
        "sd s0,  8 * 18(sp)",
        "sd s1,  8 * 19(sp)",
        "sd s2,  8 * 20(sp)",
        "sd s3,  8 * 21(sp)",
        "sd s4,  8 * 22(sp)",
        "sd s5,  8 * 23(sp)",
        "sd s6,  8 * 24(sp)",
        "sd s7,  8 * 25(sp)",
        "sd s8,  8 * 26(sp)",
        "sd s9,  8 * 27(sp)",
        "sd s10, 8 * 28(sp)",
        "sd s11, 8 * 29(sp)",
        "csrr a0, sscratch",
        "sd a0, 8 * 30(sp)",
        "csrw sscratch, a0",

        "mv a0, sp",
        "call handle_trap",

        "ld ra,  8 * 0(sp)",
        "ld gp,  8 * 1(sp)",
        "ld tp,  8 * 2(sp)",
        "ld t0,  8 * 3(sp)",
        "ld t1,  8 * 4(sp)",
        "ld t2,  8 * 5(sp)",
        "ld t3,  8 * 6(sp)",
        "ld t4,  8 * 7(sp)",
        "ld t5,  8 * 8(sp)",
        "ld t6,  8 * 9(sp)",
        "ld a0,  8 * 10(sp)",
        "ld a1,  8 * 11(sp)",
        "ld a2,  8 * 12(sp)",
        "ld a3,  8 * 13(sp)",
        "ld a4,  8 * 14(sp)",
        "ld a5,  8 * 15(sp)",
        "ld a6,  8 * 16(sp)",
        "ld a7,  8 * 17(sp)",
        "ld s0,  8 * 18(sp)",
        "ld s1,  8 * 19(sp)",
        "ld s2,  8 * 20(sp)",
        "ld s3,  8 * 21(sp)",
        "ld s4,  8 * 22(sp)",
        "ld s5,  8 * 23(sp)",
        "ld s6,  8 * 24(sp)",
        "ld s7,  8 * 25(sp)",
        "ld s8,  8 * 26(sp)",
        "ld s9,  8 * 27(sp)",
        "ld s10, 8 * 28(sp)",
        "ld s11, 8 * 29(sp)",
        "ld sp,  8 * 30(sp)",
        
        "sret",
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
pub unsafe extern "C" fn timer_trap() {
    init_timer();
    unsafe {
        sie::set_stimer();
        riscv::interrupt::supervisor::enable();
    }
    yield_();
}

#[no_mangle]
pub unsafe extern "C" fn handle_trap() {
    let scause = scause::read();

    let is_interrupt = scause.is_interrupt();
    let is_exception = scause.is_exception();
    //let spec = sepc::read();
    //println!("handle_trap");
    if is_exception  {
        panic!("{:?}", scause.cause());
        
    }else if is_interrupt {
 
    } 
    panic!()
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
