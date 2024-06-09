.altmacro
.macro SAVE_REG n
    sd x\n, \n*8(sp)
.endm
.macro LOAD_REG n
    ld x\n, \n*8(sp)
.endm
/*
    TRAP HANDLING
    I. Relevant registers in Riscv5
        [sstatus] SSP gives priority level when trap occurs
        [sepc] store last inst when before trap occurs, usually points to `ecall`
        [scause] describe trapping reason
        [stval] extra info
        [stvec] entry address of trap handler

    II. Hardware actions after trapping
    For example, when calling `ecall`

        modify SSP of [sstatus]
        set [sepc] to next inst
        update [scause] & [stval]
        jump to [stvec] and set priority level to S
*/
    .section .text
    .globl __trampoline
    .globl __restore

__trampoline:
    # duty1: save trapframe to kernel stack
    # scratch saves kernel sp
    # this inst set sscratch <= user-sp, sp <= kernel-sp
    csrrw sp, sscratch, sp

    /*
        things we need to save as a trapframe
        general registers: x0-x31 (8 bytes each)
        sstatus + sepc
    */
    # for simplicity, at this stage we save every register
    addi sp, sp, -(32+2) * 8

    sd x1, 1*8(sp)
    # skip sp
    sd x3, 3*8(sp)

    .set n, 4
    .rept 28
        SAVE_REG %n
        .set n, n+1
    .endr
    # save user stack pointer
    csrr t0, sstatus
    csrr t1, sepc
    sd t0, 32*8(sp)
    sd t1, 33*8(sp)
    csrr t2, sscratch
    sd t2, 2*8(sp)

    mv a0, sp
    call trap_handler


__restore:
    # __retore(trapframe*)
    # this function prepare the registers and other stuff extracted from a 
    # trapframe for execution
    mv sp, a0

    # restore general purpose registers
    ld x1, 1*8(sp) 
    # skip sp(x2) for a while
    ld x3, 3*8(sp)

    .set n, 4
    .rept 28
        LOAD_REG %n
        .set n, n+1
    .endr

    # restore other registers
    ld t0, 32*8(sp)
    ld t1, 33*8(sp)
    ld t2, 2*8(sp)
    csrw sstatus, t0
    csrw sepc, t1
    csrw sscratch, t2

    # release trapframe
    addi sp, sp, (32+2)*8
    csrrw sp, sscratch, sp

    # return from trap, set pc to sepc
    sret



