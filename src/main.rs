#![no_std]
#![no_main]
#![feature(panic_info_message)]
#![feature(const_trait_impl)]
#![feature(naked_functions)]
#![feature(asm_experimental_arch)]

use core::{
    arch::{asm, global_asm},
    borrow::Borrow,
    ptr,
};
mod allocator;
mod interrupt;
mod mem;
mod print;
mod proc;
mod trap;
use alloc::{collections::VecDeque, vec};
use print::Writer;
use proc::{yield_, CURRENT_PROC, IDLE_PROC};
use trap::{trap_entry};
extern crate alloc;
use crate::{interrupt::init_timer, proc::print_process};
use riscv::register::*;

#[no_mangle]
static INIT_SP: [u8; 4096 * 1028] = [0; 4096 * 1028];

#[no_mangle]
static STACK_SIZE: usize = 4096 * 1028;

global_asm!(include_str!("trap.S"));
extern "C" {
    fn trap();
}

#[no_mangle]
#[link_section = ".entry"]
pub unsafe extern "C" fn _entry() {
    asm!("la sp, INIT_SP", "ld a0, STACK_SIZE", "add sp, sp, a0",);

    main();
}

#[no_mangle]
fn main() {
    //trapをシステムyレジスタに登録
    /*
    MTVECのMODEフィールドに1を設定する。
    例外発生時の動作はダイレクトモードと同じ。割り込み発生時は、割り込み要因によりエントリが異なる。"MTVECが指すアドレス + 割り込み要因×4" が指すエントリへ制御を移す。
    */
    /*
    mode Vectored にしても配置されるpcが正しくないバグ？なのでダイレクトモードに指定して受け取りはVectoredモード
    */
    let addr_trap_entry = trap_entry as usize;

    unsafe {
        asm!("csrw stvec, {addr_trap_entry}\n", addr_trap_entry = in(reg) addr_trap_entry);
    };

    println!("{:x}", stvec::read().address());
    println!("{:?}", stvec::read().trap_mode());
    //println!("start");
    init_timer();

    unsafe {
        sie::set_stimer();
        riscv::interrupt::supervisor::enable();
    }

    unsafe {
        let mtimecmp = 0x0200_4000 as *mut u64;
        //println!("{}",mtimecmp.read());
        //mtimecmp.write_volatile(1111111);
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
    //vi32();
    //vu8();
    //shutdown();

    loop {
        for _ in 0..400000000 {
            unsafe { asm!("nop") }
        }

        println!("loop\n");
    }
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
            "vsetvli t0, {0}, e32, m1, ta, ma",
            "vle32.v v1, ({1})",
            "vle32.v v2, ({2})",
            "vadd.vv v3, v1, v2",
            "vse32.v v3, ({3})",
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
        asm!(
            "vsetvli t0, {0}, e8, m1, ta, ma",
            "vle8.v v1, ({1})",
            "vle8.v v2, ({2})",
            "vadd.vv v3, v1, v2",
            "vse8.v v3, ({3})",
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

fn shutdown() {
    unsafe {
        asm!(
            "ecall",
            in("a6") 0,
            in("a7") 8,
        );
    }
}
