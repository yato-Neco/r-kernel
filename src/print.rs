use core::arch::asm;


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
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        
        for c in s.bytes() {
            unsafe {
                asm!(
                    "ecall",
                    in("a0") c,
                    in("a6") 0,
                    in("a7") 1,
                )
            }
        }
        Ok(())
    }
}