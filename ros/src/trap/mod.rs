use core::arch::global_asm;

use riscv::register::sstatus::{self, Sstatus};
use riscv::register::{
    stval, scause::{self, Exception, Trap},
    stvec, mtvec::TrapMode
};


use crate::syscall;

global_asm!(include_str!("trap.S"));

#[repr(C)]
pub struct TrapFrame{
    pub registers: [usize; 32],
    pub sstatus: Sstatus,
    pub sepc: usize
}

impl TrapFrame{
    pub fn new(sp: usize, entry: usize) -> Self{
        let mut registers= [0usize; 32];
        // configure sstatus prior entering user state
        let mut sstatus = sstatus::read();
        sstatus.set_spp(sstatus::SPP::User);
        registers[2] = sp;
        println!("[kernel] new frame: sp->{:#x} pc->{:#x}", sp, entry);
        Self {
            registers, sstatus,
            sepc: entry 
        }
    }
}

#[no_mangle]
pub fn trap_handler(ctx: &mut TrapFrame) -> &mut TrapFrame{
    /*
        according to reported trap status
        take corresponding steps
    */
    let scause = scause::read();
    let stval = stval::read();
    match scause.cause() {
        // user syscall request
        Trap::Exception(Exception::UserEnvCall) => {
            let syscall_id = ctx.registers[17];
            println!("[kernel] handle syscall {} request", syscall_id);
            let args: [usize; 3] = [
                ctx.registers[10], ctx.registers[11], ctx.registers[12]
            ];
            syscall::router(syscall_id, args);
            /*
                move sepc to the next inst
                prior to trap, sepc will be set to point to `ecall`
            */
            ctx.sepc += 4;
        },
        Trap::Exception(Exception::StoreFault) | Trap::Exception(Exception::StorePageFault) => {
            println!("[kernel] app pagefaults, killed it");
            crate::batch::run_next_task();
        },
        Trap::Exception(Exception::IllegalInstruction) => {
            println!("[kernel] IllegalInstruction in application, killed it");
            crate::batch::run_next_task();
        }
        _ => {
            panic!(
                "Unsupported trap {:?}, stval = {:#x}",
                scause.cause(),
                stval
            );
        }
    }
    ctx
}

pub fn init(){
    /* configure stvec prior to entering user state */
    extern "C" {
        fn __trampoline();
    }
    unsafe {
        stvec::write(__trampoline as usize, TrapMode::Direct);
    }
    println!("[kernel] trap handler init")
}

