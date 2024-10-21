use core::arch::asm;

use alloc::string::ToString;


#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ({
        use core::fmt::Write;
        let _ = write!(crate::Writer, $($arg)*);
    });
}

#[macro_export]
macro_rules! println {
    ($($arg:tt)*) => ({
        print!("{}\n", format_args!($($arg)*));
    });
}

pub struct Writer;

impl core::fmt::Write for Writer {
    #[no_mangle]
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        
        for c in s.bytes() {
        
            let value: u64;
            let error: i64;

            unsafe {
                
                asm!(
                    "ecall",
                    in("a0") c,
                    in("a7") 1,
                    lateout("a0") error,
                    lateout("a1") value,
                );
            }

            
            match error {
                0 => (),
                _ => panic!(""),
            };
            
            
          
        }
        Ok(())
    }
}