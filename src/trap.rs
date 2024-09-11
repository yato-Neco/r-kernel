use crate::{interrupt::timer_interrupt, println};
use core::arch::asm;
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
        "csrrw sp, sscratch, sp\n",
        "addi sp, sp, -8 * 31\n",
        "sd ra,  8 * 0(sp)\n",
        "sd gp,  8 * 1(sp)\n",
        "sd tp,  8 * 2(sp)\n",
        "sd t0,  8 * 3(sp)\n",
        "sd t1,  8 * 4(sp)\n",
        "sd t2,  8 * 5(sp)\n",
        "sd t3,  8 * 6(sp)\n",
        "sd t4,  8 * 7(sp)\n",
        "sd t5,  8 * 8(sp)\n",
        "sd t6,  8 * 9(sp)\n",
        "sd a0,  8 * 10(sp)\n",
        "sd a1,  8 * 11(sp)\n",
        "sd a2,  8 * 12(sp)\n",
        "sd a3,  8 * 13(sp)\n",
        "sd a4,  8 * 14(sp)\n",
        "sd a5,  8 * 15(sp)\n",
        "sd a6,  8 * 16(sp)\n",
        "sd a7,  8 * 17(sp)\n",
        "sd s0,  8 * 18(sp)\n",
        "sd s1,  8 * 19(sp)\n",
        "sd s2,  8 * 20(sp)\n",
        "sd s3,  8 * 21(sp)\n",
        "sd s4,  8 * 22(sp)\n",
        "sd s5,  8 * 23(sp)\n",
        "sd s6,  8 * 24(sp)\n",
        "sd s7,  8 * 25(sp)\n",
        "sd s8,  8 * 26(sp)\n",
        "sd s9,  8 * 27(sp)\n",
        "sd s10, 8 * 28(sp)\n",
        "sd s11, 8 * 29(sp)\n",
        "csrr a0, sscratch\n",
        "sd a0, 8 * 30(sp)\n",
        "addi a0, sp, 8 * 31\n",
        "csrw sscratch, a0\n",

        "mv a0, sp\n",
        "call handle_trap\n",

        "ld ra,  8 * 0(sp)\n",
        "ld gp,  8 * 1(sp)\n",
        "ld tp,  8 * 2(sp)\n",
        "ld t0,  8 * 3(sp)\n",
        "ld t1,  8 * 4(sp)\n",
        "ld t2,  8 * 5(sp)\n",
        "ld t3,  8 * 6(sp)\n",
        "ld t4,  8 * 7(sp)\n",
        "ld t5,  8 * 8(sp)\n",
        "ld t6,  8 * 9(sp)\n",
        "ld a0,  8 * 10(sp)\n",
        "ld a1,  8 * 11(sp)\n",
        "ld a2,  8 * 12(sp)\n",
        "ld a3,  8 * 13(sp)\n",
        "ld a4,  8 * 14(sp)\n",
        "ld a5,  8 * 15(sp)\n",
        "ld a6,  8 * 16(sp)\n",
        "ld a7,  8 * 17(sp)\n",
        "ld s0,  8 * 18(sp)\n",
        "ld s1,  8 * 19(sp)\n",
        "ld s2,  8 * 20(sp)\n",
        "ld s3,  8 * 21(sp)\n",
        "ld s4,  8 * 22(sp)\n",
        "ld s5,  8 * 23(sp)\n",
        "ld s6,  8 * 24(sp)\n",
        "ld s7,  8 * 25(sp)\n",
        "ld s8,  8 * 26(sp)\n",
        "ld s9,  8 * 27(sp)\n",
        "ld s10, 8 * 28(sp)\n",
        "ld s11, 8 * 29(sp)\n",
        "ld sp,  8 * 30(sp)\n",
        "sret\n",
        options(noreturn)
    );
}

#[no_mangle]
extern "C" fn handle_trap(trap_frame: &mut TrapFrame) {
    let mut scause: u64;
    let mut stval: u64;
    let mut sepc: u64;

    let pending: u64;
    println!("handle!!!");
    unsafe {
        asm!("csrr {}, scause", out(reg) scause);
        asm!("csrr {}, stval", out(reg) stval);
        asm!("csrr {}, sepc", out(reg) sepc);
    }

   
    println!("scause: {:#010x}",scause);
    
    //println!("{}",(pending & (1 << 7)));

    match scause {
        7 => timer_interrupt(trap_frame),
        2 => exception(stval, scause, sepc, trap_frame),
        _ => exception(stval, scause, sepc, trap_frame),
    }
}

fn exception(
    stval: RregisterSize,
    scause: RregisterSize,
    sepc: RregisterSize,
    trap_frame: &mut TrapFrame,
) {
    let trap_name = match stval {
        0x00000000 => "Instruction address misaligned",
        0x00000001 => "Instruction access fault",
        _ => "_",
    };

    panic!(
        "\n\
        unexpected trap:\n\
        scause: {scause:#010x},\nstval: {stval:#010x},\n\
        sepc: {sepc:#010x},\n\
        trap_name: {trap_name},\n\
        trap_frame: {trap_frame:#?}",
    );
}
