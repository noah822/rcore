#![no_std]
#![no_main]
mod lang_items;

mod console;
mod sbi;

use core::arch::global_asm;

global_asm!(include_str!("entry.asm"));

#[no_mangle]
pub fn rust_main(){
    clear_bss();
    println!("hello world");
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
