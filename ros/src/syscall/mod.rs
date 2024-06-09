mod exit;
mod io;

pub use exit::sys_exit;
pub use io::*;

const SYSCALL_WRITE: usize = 64;
const SYSCALL_EXIT: usize = 93;

pub fn router(id: usize, args: [usize; 3]){
    match id {
        SYSCALL_EXIT => sys_exit(args[0]),
        SYSCALL_WRITE => sys_write(args[0], args[1] as *const u8, args[2]),
        _ => println!("syscall #{} NOT support yet", id)
    }
}