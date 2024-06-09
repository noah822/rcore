#![no_std]
#![no_main]
#![feature(panic_info_message)]

#[macro_use]
mod console;

mod lang_items;
mod sbi;
mod batch;
mod syscall;
mod sync;

mod trap;

use core::arch::global_asm;

global_asm!(include_str!("entry.asm"));
global_asm!(include_str!("link_app.S"));


#[no_mangle]
pub fn rust_main(){
    clear_bss();
    trap::init();
    batch::init();
    loop {}
}

fn clear_bss(){
    extern "C" {
        fn sbss(); 
        fn ebss();
    }
    (sbss as usize..ebss as usize).for_each(|p| {
        unsafe {(p as *mut u8).write(0u8)}
    })
}
