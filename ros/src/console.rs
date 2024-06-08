use crate::sbi::console_putchar;
use core::fmt::{self, Write};

struct StdOut;

impl Write for StdOut{
    fn write_str(&mut self, s: &str) -> fmt::Result{
        s.chars().for_each(|c| console_putchar(c as usize));
        Ok(())
    }
}


pub fn console_print(args: fmt::Arguments){
    StdOut.write_fmt(args).unwrap();
}


#[macro_export]
macro_rules! print {
    ($fmt: literal $(,$args: tt)*) => {
        $crate::console::console_print(format_args!($fmt $(,$args)*))
    }
}

#[macro_export]
macro_rules! println {
    ($fmt: literal $(,$args: tt)*) => {
        $crate::console::console_print(format_args!(concat!($fmt, '\n') $(,$args)*))
    }
}
