#![no_std]
#![no_main]
#![feature(panic_info_message)]
#![feature(const_trait_impl)]
#![feature(naked_functions)]
#![feature(asm_experimental_arch)]

use core::{arch::asm, borrow::Borrow, ptr};
mod allocator;
mod interrupt;
mod print;
mod proc;
mod trap;
mod mem;
use alloc::{collections::VecDeque, vec};
use print::Writer;
use proc::{yield_, CURRENT_PROC, IDLE_PROC};
extern crate alloc;
use crate::{
    interrupt::init_timer,
    proc::print_process,
};
use riscv::register::*;
use stvec::TrapMode;

#[no_mangle]
static INIT_SP: [u8; 4096 * 1028] = [0; 4096 * 1028];

#[no_mangle]
static STACK_SIZE: usize = 4096 * 1028;

#[no_mangle]
#[link_section = ".entry"]
pub unsafe extern "C" fn _entry() {
    asm!("la sp, INIT_SP", "ld a0, STACK_SIZE", "add sp, sp, a0",);

    main();
}

#[no_mangle]
fn main() {
    //trapをシステムyレジスタに登録
    let addr_trap_entry = trap::trap_entry as usize;
    
    unsafe {
        //asm!("csrw stvec, {addr_trap_entry}\n", addr_trap_entry = in(reg) addr_trap_entry );
        stvec::write(addr_trap_entry, TrapMode::Direct);

        //asm!("nop");
    };



    unsafe{
        //let mut o:u64; 
        //asm!("csrr {o}, mvendorid", o = out(reg) o );
        //println!("{}",o);
    }

    /*

    let a:usize = 0;
    unsafe {
        let ptr = a as *mut usize;
        let b = ptr.offset(15);
        println!("{}",*b);
    }
     
    */

    /* 
    panic!();
    init_timer();
    unsafe {
        IDLE_PROC = proc::Process::new(*core::ptr::null());
        CURRENT_PROC = IDLE_PROC;
    }
    

    proc::Process::new(task_a);
    //proc::Process::new(task_b);
    //proc::Process::new(task_c);

    //print_process();

    yield_();

    panic!();
    */
    vi32();
    panic!();

}


fn task_a() {
    println!("starting process A\n");
    loop {
        //println!("A");
        

        yield_();

        //rintln!("task sp: {:x}",(((*PROC_A).sp) as *const u64).read());
        //println!("task sp: {:x}",(((*PROC_B).sp) as *const u64).read());
        /*

        if i > 10 {
            unsafe{
                println!("unimp");
                asm!("unimp");
            };
        }
        */

        for _ in 0..400000000 {
            unsafe { asm!("nop") }
        }
    }
}

fn task_c() {
    println!("starting process C\n");
    loop {
        println!("C");
        yield_();

        /*
        unsafe {
            switch_context(&mut (*PROC_B).sp, &(*PROC_A).sp);
        }
        */

        for _ in 0..400000000 {
            unsafe { asm!("nop") }
        }
    }
}
fn task_b() {
    println!("starting process B\n");
    loop {
        println!("B");
        yield_();

        /*
        unsafe {
            switch_context(&mut (*PROC_B).sp, &(*PROC_A).sp);
        }
        */

        for _ in 0..400000000 {
            unsafe { asm!("nop") }
        }
    }
}

fn sleep() {
    loop {
        unsafe { asm!("nop") }
    }
}

fn vi32() {
    /*
        unsafe {

        let a = [0,1,2,3,4,5];
        let b = [1,1,1,1,1,1];
        let lo : u32;
        rvv_asm::rvv_asm!(
            "vsetvl x5, s3, t6",
            "1: vle256.v v3, (a0), vm",
            "2:",
            "li {lo}, 4",
            lo = out(reg) lo,
        );
    }
    */

    let vector1 = [1, 2, 3, 4];
    let vector2 = [5, 6, 7, 8];
    let mut result = [0, 0, 0, 0];

    let vector_length = vector1.len() as u32;

    unsafe {
        asm!(
            "mv t1, {0}",
            "mv t2, {1}",
            "mv t3, {2}",
            "mv t4, {3}",
            "vsetvli t0, t1, e32, m1, ta, ma",
            "vle32.v v1, (t2)",
            "vle32.v v2, (t3)",
            "vadd.vv v3, v1, v2",
            "vse32.v v3, (t4)",
            in(reg) vector_length,
            in(reg) &vector1 as *const i32,
            in(reg) &vector2 as *const i32,
            in(reg) &mut result as *mut i32
        );
    }

    println!("Result: {:?}", result);
}

fn vu8() {
    let vector1 = [1u8, 2, 3, 4];
    let vector2 = [5u8, 6, 7, 8];
    let mut result = [0u8, 0, 0, 0];

    let vector_length = vector1.len() as u32;

    unsafe {
        rvv_asm::rvv_asm!(
            "mv t1, {0}",
            "mv t2, {1}",
            "mv t3, {2}",
            "mv t4, {3}",
            "vsetvli t0, t1, e8, m1",
            "vle8.v v1, (t2)",
            "vle8.v v2, (t3)",
            "vadd.vv v3, v1, v2",
            "vse8.v v3, (t4)",
            in(reg) vector_length,
            in(reg) &vector1 as *const _,
            in(reg) &vector2 as *const _,
            in(reg) &mut result as *mut _
        );
    }

    println!("Result: {:?}", result);
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    println!("{:?}", _info);
    loop {
        unsafe { asm!("nop") }
    }
}

