ENTRY_ADDR :=0x80200000
# cargo build defaults to --debug with zero arg
MODE := debug
OS_BIN_DIR :=target/riscv64gc-unknown-none-elf/${MODE}
ifeq ($(MODE), release)
	CARGO_BUILD_MODE := --release
endif

KERNEL_ELF := ${OS_BIN_DIR}/ros
KERNEL_BIN := ${OS_BIN_DIR}/ros.bin

QEMU_FLAGS := -machine virt \
			  -nographic \
			  -bios ../bootloader/rustsbi-qemu.bin \
			  -device loader,file=${KERNEL_BIN},addr=${ENTRY_ADDR}

ros: kernel_bin 
	@qemu-system-riscv64 ${QEMU_FLAGS}

kernel_bin:
	cargo build ${CARGO_BUILD_MODE}
	@rust-objcopy --strip-all ${KERNEL_ELF} -O binary ${KERNEL_BIN}

gdbserver: kernel_bin 
	@qemu-system-riscv64 ${QEMU_FLAGS} -s -S

gdbclient:
	@riscv64-unknown-elf-gdb \
	    -ex 'file ${KERNEL_ELF}' \
	    -ex 'set arch riscv:rv64' \
	    -ex 'target remote localhost:1234'