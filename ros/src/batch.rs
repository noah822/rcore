use core::default::Default;
use core::arch::asm;

use crate::trap;
use crate::sync::UPSafeCell;


use lazy_static::*;
/*
    holding instruction stream (starting address) to a series of 
    program to execute 

    by default this executor will execute each task sequentially

*/
const USER_STACK_SIZE: usize = 4096 * 2;
const KERNEL_STACK_SIZE: usize = 4096 * 2;
const MAX_APP_NUM: usize = 32;
const TASK_BASE_ADDR: usize = 0x80400000;

#[repr(align(4096))]
struct KernelStack{
    data: [u8; KERNEL_STACK_SIZE]
}

impl KernelStack{
    fn push_trapframe(&self, ctx: trap::TrapFrame) -> usize{
        let ctx_ptr = (self.get_top_sp() - core::mem::size_of::<trap::TrapFrame>()) as *mut trap::TrapFrame;
        println!("[kernel] top: {:#x} ctx_ptr: {:#x}", (self.get_top_sp()), (ctx_ptr as usize));
        unsafe {
            *ctx_ptr = ctx;
            ctx_ptr as usize
        }
    }
    fn get_top_sp(&self) -> usize {
        self.data.as_slice().as_ptr() as usize + KERNEL_STACK_SIZE
    }
}

#[repr(align(4096))]
struct UserStack{
    data: [u8; KERNEL_STACK_SIZE]
}

impl UserStack {
    fn get_top_sp(&self) -> usize {
        self.data.as_slice().as_ptr() as usize + USER_STACK_SIZE
    }
}

static KERNEL_STACK: KernelStack = KernelStack {data: [0u8; KERNEL_STACK_SIZE]};
static USER_STACK: UserStack = UserStack {data: [0u8; USER_STACK_SIZE]};

struct BatchExecutor{
    num_tasks: usize, 
    tasks_inst_saddr: [usize; MAX_APP_NUM + 1],
    queueing_task_id: usize
}

impl BatchExecutor{
    pub fn print_tasks_info(&self) {
        for task_id in 0..self.num_tasks{
            let (saddr, eaddr) = (
                self.tasks_inst_saddr[task_id],
                self.tasks_inst_saddr[task_id+1]
            );
            println!("[kernel] task{} -> [{:#x}, {:#x})", task_id, saddr, eaddr);
        }
    }

    unsafe fn new() -> Self{
        extern "C" {
            fn _num_app();
        }
        let num_tasks = (_num_app as *const usize).read_volatile();
        let inst_section_ptr = (_num_app as *const usize).add(1);

        let mut tasks_inst_saddr = [0usize; MAX_APP_NUM+1];
        tasks_inst_saddr[..num_tasks+1].copy_from_slice(
            core::slice::from_raw_parts(inst_section_ptr, num_tasks+1)
        );
        Self {
            num_tasks, tasks_inst_saddr,
            queueing_task_id: 0usize
        }
    }

    fn prepare_next_task(&mut self){
        println!(
            "[kernel] loading task {}: [{:#x}, {:#x})",
            (self.queueing_task_id),
            (self.tasks_inst_saddr[self.queueing_task_id]),
            (self.tasks_inst_saddr[self.queueing_task_id+1])
        );
        unsafe {self.load_task()};
        self.queueing_task_id += 1;
    }

    unsafe fn load_task(&self){
        let src_slice = core::slice::from_raw_parts(
           self.tasks_inst_saddr[self.queueing_task_id] as *const u8,
           self.tasks_inst_saddr[self.queueing_task_id+1]-self.tasks_inst_saddr[self.queueing_task_id]
        );
        let dst_slice = core::slice::from_raw_parts_mut(
            TASK_BASE_ADDR as *mut u8, src_slice.len()
        );
        dst_slice.copy_from_slice(src_slice);
        asm!("fence.i");
    }
}


/*
    bootstrapping procedure
    1. load user program into .data
    2. instantiate batchExecutor which reads in addr info of those programs 
    3. launch the executor
*/
lazy_static! {
    static ref EXECUTOR: UPSafeCell<BatchExecutor> = unsafe {UPSafeCell::new(BatchExecutor::new())};
}

pub fn init(){
    EXECUTOR.exclusive_access().print_tasks_info();
    run_next_task();
}
pub fn run_next_task() -> !{
    // thin wrapper over static executor run_next_task call
    let mut executor_ref = EXECUTOR.exclusive_access();
    executor_ref.prepare_next_task();

    // explicit drop is required, since this function never runs, auto-drop won't work
    drop(executor_ref);
    extern "C" {
        fn __restore(ctx: usize);
    }
    // fetch and copy instruction stream from .data section to .text section
    let trapframe = trap::TrapFrame::new(USER_STACK.get_top_sp(), TASK_BASE_ADDR);
    let sp_on_frame = KERNEL_STACK.push_trapframe(trapframe);
    unsafe{
        __restore(sp_on_frame as usize);
    }
    unreachable!();

}