use crate::{print, println};
use core::{arch::asm, ptr};

/// プロセス最大数
const PROCS_MAX: usize = 5;
/// プロセスの配列
pub static mut PROCS: [Process; PROCS_MAX] = [Process::default(); PROCS_MAX];

/// アイドル状態のプロセスのポインタが入る変数
pub static mut IDLE_PROC: *mut Process = core::ptr::null_mut();
/// 実行中のプロセスのポインタが入る変数
pub static mut CURRENT_PROC: *mut Process = core::ptr::null_mut();


#[derive(Debug, Clone, Copy, PartialEq)]
pub enum State {
    UNUSED = 0,
    RUNNABLE = 1,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Process {
    pub pid: usize,
    pub state: State,
    pub sp: usize,
    pub stack: [u8; 4098],
}
///sd(64bit) sw(32bit)
///ld(64bit) lw(32bit)

///コンテキストスイッチ
#[naked]
pub extern "C" fn switch_context(prev_sp: *mut usize, next_sp: *const usize) {
    //
    unsafe {
        asm!(
            "addi sp, sp, -13 * 8",
            "sd ra,  0  * 8(sp)",
            "sd s0,  1  * 8(sp)",
            "sd s1,  2  * 8(sp)",
            "sd s2,  3  * 8(sp)",
            "sd s3,  4  * 8(sp)",
            "sd s4,  5  * 8(sp)",
            "sd s5,  6  * 8(sp)",
            "sd s6,  7  * 8(sp)",
            "sd s7,  8  * 8(sp)",
            "sd s8,  9  * 8(sp)",
            "sd s9,  10 * 8(sp)",
            "sd s10, 11 * 8(sp)",
            "sd s11, 12 * 8(sp)",
            "sd sp, (a0)",
            "ld sp, (a1)",
            "ld ra,  0  * 8(sp)",
            "ld s0,  1  * 8(sp)",
            "ld s1,  2  * 8(sp)",
            "ld s2,  3  * 8(sp)",
            "ld s3,  4  * 8(sp)",
            "ld s4,  5  * 8(sp)",
            "ld s5,  6  * 8(sp)",
            "ld s6,  7  * 8(sp)",
            "ld s7,  8  * 8(sp)",
            "ld s8,  9  * 8(sp)",
            "ld s9,  10 * 8(sp)",
            "ld s10, 11 * 8(sp)",
            "ld s11, 12 * 8(sp)",
            "addi sp, sp, 13 * 8",
            "ret",
            options(noreturn),
        )
    }
}

impl Process {
    /// プロセスの作成
    pub fn new(pc: fn()) -> *mut Process {
        //pidに使う変数の初期化
        let mut i: usize = 0;
        //空いているプロセスのポインタを入れる変数の初期化
        let mut proc: *mut Process = core::ptr::null_mut();

        // 配列PROCSから状態がUNUSEDな状態のプロセスを探す
        for j in 0..PROCS_MAX {
            unsafe {
                if PROCS[i].state == State::UNUSED {
                    //空いているプロセスのポインタをprocへ代入
                    proc = &mut PROCS[i] as *mut Process;
                    //代入できたらforを抜ける
                    break;
                }
            }
            //pidに使う変数に番号を代入
            i = j;
        }
        //プロセスに空きがなかったらpanic
        if proc.is_null() {
            panic!("no free process slots");
        }

        //println!("pid: {}", i + 1);

        unsafe {
            //ポインタから参照
            let _proc = &mut *proc;

            //プロセスのスタック領域の最後のポインタを取得
            let mut sp = _proc.stack.as_mut_ptr().add(_proc.stack.len() - 1) as *mut usize;

            //println!("pc pointer: {:x}", pc as u64);
            // スタックを13個のレジスタ分確保し初期化

            for _ in 0..13 {
                sp = sp.offset(-1);
                ptr::write_volatile(sp, 0);
            }

            // 呼び出し先のアドレスを設定
            sp = sp.offset(-1);
            ptr::write_volatile(sp, pc as usize);

            _proc.pid = i + 1;

            //プロセスのスータスをRUNNABLE
            _proc.state = State::RUNNABLE;

            //spのポインタを保存
            _proc.sp = sp as usize;

            //println!("sp pointer: {:?}", _proc.sp as *mut usize);

            return _proc;
        }

        //panic!("no free process slots")
    }

    ///プロセスのデフォルト
    const fn default() -> Process {
        Process {
            pid: 0,
            state: State::UNUSED,
            sp: 0,
            stack: [0; 4098],
        }
    }
}

///次のプロセスを探してコンテキストスイッチ
pub fn yield_() {
    unsafe {
        let mut next = IDLE_PROC;
        /*
        println!(
            "next pid: {}, sp: {:x}  state: {:?}",
            (*next).pid,
            (*next).sp,
            (*next).state
        );
        */
        

        //次実行するプロセスを探す
        for i in 0..PROCS_MAX {
            //println!("nextt pid: {}", ((*CURRENT_PROC).pid + i) % PROCS_MAX);

            let proc = &mut (PROCS[((*CURRENT_PROC).pid + i) % PROCS_MAX]);
            //プロセスの状態がRUNNABLEかつプロセスIDが1以上
            if proc.state == State::RUNNABLE && proc.pid > 1 {
                /*
                println!(
                    "pid: {}, sp: {:x}  state: {:?}",
                    proc.pid, proc.sp, proc.state
                );
                */
                
                //ptr::write_volatile(IDLE_PROC, proc);
                next = proc;
                //IDLE_PROC = &mut proc as *mut Process;
                break;
            }
        }

        /*
        println!(
            "next pid: {}, sp: {:x}  state: {:?}",
            (*next).pid,
            (*next).sp,
            (*next).state
        );
        println!(
            "current pid: {}, sp: {:x}  state: {:?}",
            (*CURRENT_PROC).pid,
            (*CURRENT_PROC).sp,
            (*CURRENT_PROC).state
        );
        */
        

        //次実行するプロセスと現在実行されているプロセスが同じだったらコンテキストスイッチをしない
        if *next == *CURRENT_PROC {
            return;
        }

        /*
        println!(
            "next pid: {}, sp: {:x}  state: {:?}",
            (*next).pid,
            (*next).sp,
            (*next).state
        );
        */
        

        
        //スタックのポインタを
        let ptr = (*next).stack.as_mut_ptr().add((*next).stack.len() - 1) as *mut usize;

        //スタックのポインタをレジスタsscratchへ書き込み
        asm!("csrw sscratch, {ptr}",ptr = in(reg) ptr);

        //実行中のプロセスのポインタをprevに代入
        let prev = CURRENT_PROC;

        //次のプロセスのポインタを実行中のプロセスが入る変数に代入
        CURRENT_PROC = next;
        //ptr::write_volatile(CURRENT_PROC, *next);

        //コンテキストスイッチ
        switch_context(&mut (*prev).sp, &(*next).sp);
    }

}


pub fn print_process() {
    println!("Process:");
    unsafe {
        PROCS.iter().for_each(|x| {
            println!("  pid: {}\n  sp: {:x}\n  state: {:?}", x.pid, x.sp, x.state);
        });
    }
}



pub struct MicroProcess {
    pid: u32,
    pub state: u8,
    pub sp: usize,
    pub stack: [u8; 512],
}

