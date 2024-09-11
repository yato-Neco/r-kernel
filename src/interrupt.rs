use crate::trap::TrapFrame;
use core::arch::asm;
//Artificial Linguistic Internet Computer Entity
use crate::print;
use crate::println;

const BASE_ADDER: usize = 0x0200_0000;
const MTIME_OFFSET: usize = 0xBFF8;
const MTIMECMP_OFFSET: usize = 0x4000;
const INTERVAL: usize = 10000000;


const MIP_MTIP:usize =  1 << 7;  // MIPレジスタのMTIPビット
use riscv::register::{time,mie};

pub fn init_timer() {
    //let mtime_ptr = (BASE_ADDER + MTIME_OFFSET) as *const usize;
    let time = time::read64();
   
    unsafe {
        
        //asm!("csrs mie, {m}",m = in(reg) MIP_MTIP)
        //let a = mtime_ptr.read_volatile();
        println!("{}",time);

        /*
        let current_time: usize;
        asm!("csrr {0}, time", out(reg) current_time);
        let next_time = current_time + INTERVAL;
        asm!("csrw timecmp, {0}", in(reg) next_time);

        */
       
        //
        //println!("mtime: {}",mtime);
        //asm!("csrs mie, a0");
        //asm!("csrw mtimecmp, a0");
    }
}

pub fn timer() {}

pub fn timer_interrupt(trap_frame: &mut TrapFrame) {
    
    unsafe {
        let mut now: u64;
        asm!("mv {}, a0",out(reg) now);
        println!("{:?}", trap_frame);
        //panic!("{now}");
    }
    

    for _ in 0..400000000 {
        unsafe { asm!("nop") }
    }
}
