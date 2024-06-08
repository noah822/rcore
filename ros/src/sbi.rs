pub fn console_putchar(c: usize){
    #[allow(deprecated)]
    sbi_rt::legacy::console_putchar(c);
}


use sbi_rt::{NoReason, Shutdown, SystemFailure};
pub fn shutdown(failure: bool) -> !{
    if !failure {
        sbi_rt::system_reset(Shutdown, NoReason);
    }else{
        sbi_rt::system_reset(Shutdown, SystemFailure);
    }
    unreachable!()
}