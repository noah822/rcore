use crate::batch;

pub fn sys_exit(exit_code: usize) -> !{
    println!("[kernel] program exits with {}", exit_code);
    batch::run_next_task()
}